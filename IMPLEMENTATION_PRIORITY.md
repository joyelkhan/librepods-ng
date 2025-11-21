# LibrePods-NG Implementation Priority & Roadmap

**Status**: Gap Analysis Complete → Ready for Implementation  
**Date**: November 22, 2025  
**Completeness**: 18% → Target: 100%

---

## Priority Matrix

```
Impact
  ↑
  │  CRITICAL        HIGH              MEDIUM
  │  (Do First)      (Do Second)       (Do Third)
  │
  │  ■ Bluetooth     ■ Flutter FFI     ■ Packaging
  │  ■ Protocol      ■ Android Root    ■ CI/CD
  │  ■ Security      ■ Tests           ■ Docs
  │
  └─────────────────────────────────────────→ Effort
```

---

## Tier 1: Critical Path (Blocking Everything Else)

### 1.1 Bluetooth Backends (4 files, 1600+ LOC)

**Why First**: Without working Bluetooth, nothing else matters.

**Files**:
- `crates/core/src/backends/bluez.rs` – Linux (BlueZ D-Bus)
- `crates/core/src/backends/winrt.rs` – Windows (WinRT API)
- `crates/core/src/backends/corebluetooth.rs` – macOS (CoreBluetooth)
- `crates/core/src/backends/android.rs` – Android (JNI)

**Acceptance Criteria**:
- [ ] Device discovery works on all 4 platforms
- [ ] Connection/disconnection works
- [ ] Read/write characteristics work
- [ ] Notifications work
- [ ] Error handling is robust
- [ ] `cargo test --lib backends` passes

**Effort**: 80 hours  
**Timeline**: Days 1-5

---

### 1.2 Protocol Implementation (1 file, 600+ LOC)

**Why Critical**: Defines the entire AirPods communication format.

**File**: `crates/core/src/protocol.rs`

**Must Include**:
- All 15 message types (Battery, ANC, Transparency, etc.)
- CRC-16 checksum calculation
- HKDF key derivation
- Message serialization/deserialization
- Firmware version parsing
- State machine integration

**Acceptance Criteria**:
- [ ] All 15 message types defined
- [ ] CRC validation works
- [ ] HKDF produces correct keys
- [ ] Round-trip serialization works
- [ ] `cargo test --lib protocol` passes with ≥95% coverage

**Effort**: 40 hours  
**Timeline**: Days 3-7

---

### 1.3 Security Module (1 file, 300+ LOC)

**Why Critical**: Protects against replay attacks and eavesdropping.

**File**: `crates/core/src/security.rs`

**Must Include**:
- AES-256-GCM encryption/decryption
- HKDF key derivation
- Constant-time comparison
- Zeroize on drop (memory safety)
- Replay attack prevention (nonce tracking)
- IV generation

**Acceptance Criteria**:
- [ ] Encryption/decryption round-trip works
- [ ] Constant-time comparison verified
- [ ] Zeroize clears memory
- [ ] Replay detection works
- [ ] `cargo test --lib security` passes

**Effort**: 30 hours  
**Timeline**: Days 5-8

---

## Tier 2: Unblocking Mobile & FFI

### 2.1 Flutter Rust Bridge (3 files, 700+ LOC)

**Why High Priority**: Enables cross-platform UI.

**Files**:
- `crates/ffi/src/lib.rs` – Rust FFI layer (200+ LOC)
- `flutter/lib/bridge_generated.dart` – Auto-generated (auto)
- `flutter/lib/main.dart` – UI implementation (500+ LOC)

**Acceptance Criteria**:
- [ ] `flutter_rust_bridge` generates code without errors
- [ ] Bluetooth device list updates in real-time
- [ ] Battery status displays correctly
- [ ] Feature toggles work
- [ ] App runs on Android, iOS, Linux, Windows, macOS

**Effort**: 40 hours  
**Timeline**: Days 9-12

---

### 2.2 Android Root Module (3 files, 80+ LOC)

**Why High Priority**: Enables system-level access.

**Files**:
- `android/root_module/module.prop` – Magisk descriptor
- `android/root_module/service.sh` – Install script
- `android/root_module/sepolicy.te` – SELinux policy

**Acceptance Criteria**:
- [ ] Magisk module installs without errors
- [ ] Service starts on boot
- [ ] SELinux policy allows Bluetooth access
- [ ] Uninstall is clean

**Effort**: 15 hours  
**Timeline**: Days 13-14

---

## Tier 3: Distribution & Packaging

### 3.1 Build Scripts (5 files, 250+ LOC)

**Why Medium Priority**: Enables distribution.

**Files**:
- `packaging/build-appimage.sh` – Linux AppImage
- `packaging/build-msix.ps1` – Windows Store
- `packaging/build-dmg.sh` – macOS DMG
- `packaging/build-deb.sh` – Debian package
- `packaging/build-flatpak.sh` – Flatpak

**Acceptance Criteria**:
- [ ] Each script produces valid artifact
- [ ] Artifacts are signed
- [ ] Checksums are generated
- [ ] Installation works on target platform
- [ ] Uninstall is clean

**Effort**: 30 hours  
**Timeline**: Days 15-18

---

## Tier 4: Testing & CI/CD

### 4.1 Hardware Testing (2 files, 100+ LOC)

**Why Medium Priority**: Ensures real-world compatibility.

**Files**:
- `.github/workflows/hardware.yml` – Nightly test runner
- `crates/core/tests/hardware_test.rs` – Test suite

**Acceptance Criteria**:
- [ ] Tests run nightly on Raspberry Pi with real AirPods
- [ ] CRC drift ≤ 1%
- [ ] Connection stability ≥ 99%
- [ ] Battery reading accuracy ≥ 95%

**Effort**: 20 hours  
**Timeline**: Days 19-21

---

### 4.2 Release Automation (1 file, 50+ LOC)

**Why Medium Priority**: Enables continuous delivery.

**File**: `.github/workflows/release.yml`

**Acceptance Criteria**:
- [ ] Triggered on tag push
- [ ] Builds all 5 platforms
- [ ] Generates all 8 artifacts
- [ ] Creates GitHub release
- [ ] Uploads to stores

**Effort**: 10 hours  
**Timeline**: Days 21-22

---

## Tier 5: Documentation & Localization

### 5.1 Documentation (4 files, 400+ LOC)

**Why Lower Priority**: Doesn't block functionality.

**Files**:
- `docs/src/user_manual.md` – User guide
- `docs/src/developer_guide.md` – Dev guide
- `docs/adr/0001-*.md` – Architecture decisions
- `docs/adr/0002-*.md` – Design rationale

**Acceptance Criteria**:
- [ ] User manual covers all features
- [ ] Developer guide explains architecture
- [ ] ADRs document key decisions
- [ ] mdBook builds without errors

**Effort**: 20 hours  
**Timeline**: Days 23-25

---

### 5.2 Localization (5 files, 500+ LOC)

**Why Lower Priority**: Doesn't block core functionality.

**Files**:
- `l10n/app_es.arb` – Spanish
- `l10n/app_fr.arb` – French
- `l10n/app_de.arb` – German
- `l10n/app_zh.arb` – Chinese
- `l10n/app_ja.arb` – Japanese

**Acceptance Criteria**:
- [ ] All strings translated
- [ ] No missing keys
- [ ] Plurals handled correctly
- [ ] RTL languages supported (if applicable)

**Effort**: 15 hours  
**Timeline**: Days 25-27

---

## Parallel Work Streams

### Stream A: Core (Tier 1)
- Developer 1: Bluetooth backends
- Developer 2: Protocol + Security
- **Duration**: 8 days
- **Blocker for**: Everything

### Stream B: Mobile (Tier 2)
- Developer 3: Flutter FFI
- Developer 4: Android root module
- **Starts**: Day 9 (after Tier 1 API stable)
- **Duration**: 6 days

### Stream C: Distribution (Tier 3)
- Developer 5: Build scripts
- **Starts**: Day 15 (after Tier 1 complete)
- **Duration**: 4 days

### Stream D: Testing (Tier 4)
- Developer 6: Hardware tests
- Developer 7: Release automation
- **Starts**: Day 19 (after Tier 3 complete)
- **Duration**: 4 days

### Stream E: Docs (Tier 5)
- Developer 8: Documentation
- Developer 9: Localization
- **Starts**: Day 23 (parallel with Tier 4)
- **Duration**: 5 days

---

## Weekly Milestones

### Week 1 (Days 1-5)
- [ ] Bluetooth backends 50% complete
- [ ] Protocol 50% complete
- [ ] All Tier 1 APIs defined

### Week 2 (Days 6-10)
- [ ] Bluetooth backends 100% complete
- [ ] Protocol 100% complete
- [ ] Security 100% complete
- [ ] Flutter FFI 50% complete
- [ ] `cargo test --all-features` passes

### Week 3 (Days 11-15)
- [ ] Flutter FFI 100% complete
- [ ] Android root module 100% complete
- [ ] Packaging scripts 50% complete
- [ ] Flutter app runs on all platforms

### Week 4 (Days 16-20)
- [ ] Packaging scripts 100% complete
- [ ] Hardware tests 100% complete
- [ ] Release automation 100% complete
- [ ] All artifacts build successfully

### Week 5 (Days 21-25)
- [ ] Documentation 100% complete
- [ ] Localization 50% complete
- [ ] mdBook builds
- [ ] All tests pass

### Week 6 (Days 26-30)
- [ ] Localization 100% complete
- [ ] Final QA & polish
- [ ] v1.0.0 release ready

---

## Success Metrics

### Code Quality
- [ ] `cargo test --all-features` ≥ 95% pass rate
- [ ] Code coverage ≥ 90%
- [ ] Zero clippy warnings
- [ ] Zero unsafe code

### Functionality
- [ ] All 15 message types working
- [ ] All 4 Bluetooth backends working
- [ ] All 5 platforms building
- [ ] All 8 artifact types generated

### Performance
- [ ] Connection time ≤ 2 seconds
- [ ] Battery read latency ≤ 100ms
- [ ] Memory usage ≤ 50MB
- [ ] CPU usage ≤ 5% idle

### Reliability
- [ ] CRC drift ≤ 1%
- [ ] Connection stability ≥ 99%
- [ ] Battery accuracy ≥ 95%
- [ ] Uptime ≥ 99.9%

---

## Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-----------|
| Bluetooth API incompatibility | Medium | High | Early prototyping on each platform |
| FFI performance issues | Low | Medium | Benchmark early, optimize late |
| Android root detection | Medium | High | Test on multiple devices/ROMs |
| Store approval delays | Low | High | Submit early, iterate quickly |
| Localization quality | Low | Low | Use professional translators |

---

## Resource Allocation

| Role | Count | Tier 1 | Tier 2 | Tier 3 | Tier 4 | Tier 5 |
|------|-------|--------|--------|--------|--------|--------|
| Rust Dev | 2 | 100% | 50% | – | 50% | – |
| Mobile Dev | 2 | – | 100% | – | – | 50% |
| DevOps | 1 | – | – | 100% | 100% | – |
| QA | 1 | 50% | 50% | – | 100% | 50% |
| Docs | 1 | – | – | – | – | 100% |

---

## Deliverables Checklist

### Phase 1: Core
- [ ] `crates/core/src/backends/bluez.rs`
- [ ] `crates/core/src/backends/winrt.rs`
- [ ] `crates/core/src/backends/corebluetooth.rs`
- [ ] `crates/core/src/backends/android.rs`
- [ ] `crates/core/src/protocol.rs`
- [ ] `crates/core/src/security.rs`

### Phase 2: Mobile
- [ ] `crates/ffi/src/lib.rs`
- [ ] `flutter/lib/bridge_generated.dart`
- [ ] `flutter/lib/main.dart`
- [ ] `android/root_module/module.prop`
- [ ] `android/root_module/service.sh`
- [ ] `android/root_module/sepolicy.te`

### Phase 3: Distribution
- [ ] `packaging/build-appimage.sh`
- [ ] `packaging/build-msix.ps1`
- [ ] `packaging/build-dmg.sh`
- [ ] `packaging/build-deb.sh`
- [ ] `packaging/build-flatpak.sh`

### Phase 4: Testing
- [ ] `.github/workflows/hardware.yml`
- [ ] `.github/workflows/release.yml`
- [ ] `crates/core/tests/hardware_test.rs`

### Phase 5: Documentation
- [ ] `docs/src/user_manual.md`
- [ ] `docs/src/developer_guide.md`
- [ ] `docs/adr/0001-*.md`
- [ ] `docs/adr/0002-*.md`
- [ ] `l10n/app_es.arb`
- [ ] `l10n/app_fr.arb`
- [ ] `l10n/app_de.arb`
- [ ] `l10n/app_zh.arb`
- [ ] `l10n/app_ja.arb`

---

## Next Actions

1. **Review this roadmap** with team
2. **Assign developers** to streams
3. **Create GitHub issues** for each file
4. **Set up project board** with milestones
5. **Schedule daily standups** to track progress
6. **Begin Week 1** with Tier 1 implementation

---

**Status**: Ready for Implementation  
**Target Completion**: 6 weeks  
**Estimated Effort**: 200 hours  
**Team Size**: 9 developers

---

*Last Updated: November 22, 2025*
