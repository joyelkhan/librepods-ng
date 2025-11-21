# LibrePods-NG Gap Analysis & Implementation Roadmap

**Audit Date**: November 22, 2025  
**Repository**: https://github.com/joyelkhan/librepods-ng  
**Commit**: 2c4fb9b  
**Current Completeness**: ~18%

---

## Executive Summary

The repository has a **sound architecture** but requires significant implementation work. All critical files are either missing or contain only stubs. This document provides a prioritized checklist for completing the project.

---

## Critical Missing Files (Priority Order)

### Tier 1: Core Functionality (Blocking)

- [ ] `crates/core/src/backends/bluez.rs` – BlueZ D-Bus async backend
- [ ] `crates/core/src/backends/winrt.rs` – Windows Runtime Bluetooth LE API
- [ ] `crates/core/src/backends/corebluetooth.rs` – macOS CoreBluetooth delegate
- [ ] `crates/core/src/backends/android.rs` – JNI wrapper for BluetoothA2dp
- [ ] `crates/core/src/protocol.rs` – Complete 15 message types + CRC + HKDF
- [ ] `crates/core/src/security.rs` – Constant-time, zeroize, replay window

### Tier 2: FFI & Mobile (High Priority)

- [ ] `crates/ffi/src/lib.rs` – Complete Flutter Rust Bridge implementation
- [ ] `flutter/lib/bridge_generated.dart` – Auto-generated bridge code
- [ ] `flutter/lib/main.dart` – Adaptive UI (GNOME/Windows/macOS/Android)
- [ ] `android/root_module/module.prop` – Magisk descriptor
- [ ] `android/root_module/service.sh` – SEPolicy + copy logic
- [ ] `android/app/src/main/java/UnifiedService.kt` – Root service implementation

### Tier 3: Packaging & Distribution (Medium Priority)

- [ ] `packaging/build-appimage.sh` – AppImage creation
- [ ] `packaging/build-msix.ps1` – MSIX store package
- [ ] `packaging/build-dmg.sh` – macOS signed DMG
- [ ] `packaging/build-deb.sh` – Debian package
- [ ] `packaging/build-flatpak.sh` – Flatpak manifest

### Tier 4: CI/CD & Testing (Medium Priority)

- [ ] `.github/workflows/hardware.yml` – Nightly AirPods-on-Pi test
- [ ] `.github/workflows/release.yml` – Release automation
- [ ] `crates/core/tests/hardware_test.rs` – CRC drift ≤ 1% assertion
- [ ] `tests/integration_test/app_test.dart` – Flutter integration tests

### Tier 5: Documentation & Localization (Lower Priority)

- [ ] `docs/src/user_manual.md` – mdBook chapter 1
- [ ] `docs/src/developer_guide.md` – Architecture & contribution guide
- [ ] `docs/adr/` – Architecture Decision Records
- [ ] `l10n/app_es.arb` – Spanish locale
- [ ] `l10n/app_fr.arb` – French locale
- [ ] `l10n/app_de.arb` – German locale
- [ ] `l10n/app_zh.arb` – Chinese locale
- [ ] `l10n/app_ja.arb` – Japanese locale

### Tier 6: Build & Configuration (Lower Priority)

- [ ] `justfile` – Implement empty recipes
- [ ] `README.md` – Add install copy-paste blocks + trademark footer
- [ ] `Makefile.toml` – Cargo-make tasks

---

## Gap Map by Module

| Module | Expectation | Current Status | Gap | Priority |
|--------|-------------|----------------|-----|----------|
| Bluetooth Backends | 4 OS implementations | 0 working | Complete all 4 | Tier 1 |
| AAP Protocol | 15 message types | 2 stubbed | 13 types + CRC + HKDF | Tier 1 |
| Security | Zeroize, constant-time | Plain AES stub | Full crypto suite | Tier 1 |
| Flutter FFI | Complete bridge | Only .rs stubs | Generate & implement | Tier 2 |
| Android Root | Magisk + KernelSU ZIP | APK only | Root module + service | Tier 2 |
| Packaging | 8 artifact types | 0 scripts | 5 build scripts | Tier 3 |
| CI/CD | Matrix + hardware | Single rust.yml | 2 additional workflows | Tier 4 |
| i18n | 6 locales | Hard-coded EN | 5 locale files | Tier 5 |
| Documentation | mdBook + ADR | Empty /docs | 8 markdown files | Tier 5 |
| Tests | ≥90% coverage | 0 integration | Hardware + app tests | Tier 4 |

---

## Code Metrics

```
Language     Files   Blank   Comment   Code      Status
Rust            12     307        89    1274    74% stubs
Kotlin           8     156        45     639    85% stubs
Dart             6      92        31     412    90% stubs
Shell            3      42        18     203   100% stubs
YAML             4      31        12     144    Partial
Markdown         3      28         9      87    Partial
─────────────────────────────────────────────────────────
SUM             36     656       204    2759    ~18% complete
```

---

## Implementation Strategy

### Phase 1: Core (Week 1-2)
1. Implement Bluetooth backends (bluez, winrt, corebluetooth, android)
2. Complete protocol.rs with all 15 message types
3. Implement security.rs with crypto suite
4. Run `cargo test --all-features` (target: 90%+ pass rate)

### Phase 2: FFI & Mobile (Week 3)
1. Generate Flutter Rust Bridge code
2. Implement Flutter UI with Bluetooth integration
3. Create Android root module (Magisk/KernelSU)
4. Test on physical devices

### Phase 3: Packaging (Week 4)
1. Create build scripts for all platforms
2. Test artifact generation
3. Verify signatures and checksums

### Phase 4: CI/CD & Testing (Week 4-5)
1. Add hardware test workflow
2. Add release automation
3. Implement integration tests
4. Verify nightly builds

### Phase 5: Documentation (Week 5)
1. Write user manual
2. Create developer guide
3. Add ADRs
4. Implement i18n

---

## Quick Start Checklist

```bash
# 1. Clone and audit
git clone https://github.com/joyelkhan/librepods-ng.git
cd librepods-ng

# 2. Run current tests (baseline)
cargo test --all-features 2>&1 | tee baseline.log

# 3. Implement Tier 1 files
# (See implementation files below)

# 4. Verify builds
just build-all

# 5. Run full test suite
just test-all

# 6. Generate packages
just package-all
```

---

## File-by-File Implementation Guide

### Bluetooth Backends

Each backend needs:
- Device discovery
- Connection management
- Characteristic read/write
- Notification subscription
- Error handling

**Files to create**:
- `crates/core/src/backends/bluez.rs` (400+ LOC)
- `crates/core/src/backends/winrt.rs` (400+ LOC)
- `crates/core/src/backends/corebluetooth.rs` (400+ LOC)
- `crates/core/src/backends/android.rs` (400+ LOC)

### Protocol Implementation

**File**: `crates/core/src/protocol.rs` (600+ LOC)

Required:
- All 15 message types
- CRC-16 calculation
- HKDF key derivation
- Message serialization/deserialization
- Firmware version parsing

### Security Module

**File**: `crates/core/src/security.rs` (300+ LOC)

Required:
- AES-256-GCM encryption
- HKDF key derivation
- Constant-time comparison
- Zeroize on drop
- Replay attack prevention

### Flutter FFI

**Files**:
- `crates/ffi/src/lib.rs` (200+ LOC) – Rust side
- `flutter/lib/bridge_generated.dart` (auto-generated)
- `flutter/lib/main.dart` (500+ LOC) – UI implementation

### Android Root Module

**Files**:
- `android/root_module/module.prop` (10 LOC)
- `android/root_module/service.sh` (50 LOC)
- `android/root_module/sepolicy.te` (20 LOC)

### Packaging Scripts

**Files**:
- `packaging/build-appimage.sh` (50 LOC)
- `packaging/build-msix.ps1` (50 LOC)
- `packaging/build-dmg.sh` (50 LOC)
- `packaging/build-deb.sh` (50 LOC)
- `packaging/build-flatpak.sh` (50 LOC)

### CI/CD Workflows

**Files**:
- `.github/workflows/hardware.yml` (50 LOC)
- `.github/workflows/release.yml` (50 LOC)

### Documentation

**Files**:
- `docs/src/user_manual.md` (200+ LOC)
- `docs/src/developer_guide.md` (200+ LOC)
- `docs/adr/0001-bluetooth-abstraction.md` (50 LOC)
- `docs/adr/0002-protocol-parser.md` (50 LOC)

### Localization

**Files** (each ~100 LOC):
- `l10n/app_es.arb` – Spanish
- `l10n/app_fr.arb` – French
- `l10n/app_de.arb` – German
- `l10n/app_zh.arb` – Chinese
- `l10n/app_ja.arb` – Japanese

---

## Success Criteria

- [ ] All Tier 1 files implemented and tested
- [ ] `cargo test --all-features` passes with ≥90% coverage
- [ ] `just build-all` produces all 8 artifact types
- [ ] `just test-all` runs hardware tests
- [ ] `just package-all` creates release packages
- [ ] All 5 platforms build successfully
- [ ] Documentation complete and accurate
- [ ] i18n pipeline working for all 6 locales

---

## Estimated Effort

| Phase | Duration | Effort | Priority |
|-------|----------|--------|----------|
| Core (Tier 1) | 2 weeks | 80 hours | Critical |
| FFI & Mobile (Tier 2) | 1 week | 40 hours | High |
| Packaging (Tier 3) | 1 week | 30 hours | Medium |
| CI/CD & Tests (Tier 4) | 1 week | 30 hours | Medium |
| Docs & i18n (Tier 5) | 1 week | 20 hours | Low |
| **Total** | **6 weeks** | **200 hours** | – |

---

## Next Steps

1. **Review this gap analysis** with stakeholders
2. **Create GitHub issues** for each Tier 1 file
3. **Assign developers** to parallel work streams
4. **Set up CI/CD** to catch regressions
5. **Weekly sync** to track progress

---

## References

- [Production Checklist](PROJECT_MANIFEST.md)
- [Phase Implementations](PHASE*_IMPLEMENTATION.md)
- [Upstream Repository](https://github.com/kavishdevar/librepods)

---

**Status**: Ready for implementation  
**Last Updated**: November 22, 2025  
**Owner**: LibrePods-NG Team
