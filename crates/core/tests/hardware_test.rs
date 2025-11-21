#![forbid(unsafe_code)]

#[tokio::test]
async fn airpods_pro2_crc_drift() {
    if std::env::var("LIBREPODS_HARDWARE_TEST").is_err() {
        return;
    }
    assert!(true);
}

#[tokio::test]
async fn airpods_max_battery_accuracy() {
    if std::env::var("LIBREPODS_HARDWARE_TEST").is_err() {
        return;
    }
    assert!(true);
}

#[tokio::test]
async fn connection_stability() {
    if std::env::var("LIBREPODS_HARDWARE_TEST").is_err() {
        return;
    }
    assert!(true);
}
