use zeroize::Zeroize;

pub struct ReplayWindow {
    window: u128,
    last_nonce: u64,
}

impl ReplayWindow {
    pub fn new() -> Self {
        Self {
            window: 0,
            last_nonce: 0,
        }
    }

    pub fn check(&mut self, nonce: u64) -> bool {
        if nonce <= self.last_nonce.saturating_sub(128) {
            return false;
        }
        if nonce > self.last_nonce {
            let diff = (nonce - self.last_nonce) as usize;
            if diff < 128 {
                self.window <<= diff;
            } else {
                self.window = 0;
            }
            self.last_nonce = nonce;
        }
        let bit = ((self.last_nonce - nonce) & 127) as usize;
        let mask = 1u128 << bit;
        if (self.window & mask) != 0 {
            return false;
        }
        self.window |= mask;
        true
    }
}

pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

pub struct SecureBuffer {
    data: Vec<u8>,
}

impl SecureBuffer {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
}

impl Drop for SecureBuffer {
    fn drop(&mut self) {
        self.data.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_window_detects_duplicates() {
        let mut rw = ReplayWindow::new();
        assert!(rw.check(10));
        assert!(!rw.check(10));
        assert!(rw.check(11));
    }

    #[test]
    fn constant_time_eq_works() {
        assert!(constant_time_eq(b"hello", b"hello"));
        assert!(!constant_time_eq(b"hello", b"world"));
    }

    #[test]
    fn secure_buffer_zeroizes() {
        let buf = SecureBuffer::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(buf.as_slice(), &[1, 2, 3, 4, 5]);
    }
}
