use librepods_core::*;

#[test]
fn test_engine_initialization() {
    let engine = Engine::new();
    assert_eq!(engine.devices().count(), 0);
}

#[test]
fn test_device_registration() {
    let mut engine = Engine::new();
    let device = Device::new(
        "test_001".to_string(),
        "Test AirPods".to_string(),
        DeviceModel::AirPods3,
    );
    
    engine.register_device(device);
    assert_eq!(engine.devices().count(), 1);
}

#[test]
fn test_device_retrieval() {
    let mut engine = Engine::new();
    let device = Device::new(
        "test_001".to_string(),
        "Test AirPods".to_string(),
        DeviceModel::AirPods3,
    );
    
    engine.register_device(device);
    let retrieved = engine.get_device("test_001");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name(), "Test AirPods");
}

#[test]
fn test_message_parsing() {
    let msg = Message::new(MessageType::BatteryStatus, vec![50, 60, 70]);
    let serialized = msg.serialize();
    assert!(serialized.len() > 0);
}

#[test]
fn test_device_capabilities() {
    let mut device = Device::new(
        "test_001".to_string(),
        "Test AirPods".to_string(),
        DeviceModel::AirPodsProGen2,
    );
    
    device.add_capability(DeviceCapability::BatteryMonitoring);
    device.add_capability(DeviceCapability::NoiseControl);
    
    assert!(device.has_capability(DeviceCapability::BatteryMonitoring));
    assert!(device.has_capability(DeviceCapability::NoiseControl));
    assert!(!device.has_capability(DeviceCapability::HeartRate));
}

#[test]
fn test_device_state_transitions() {
    let mut device = Device::new(
        "test_001".to_string(),
        "Test AirPods".to_string(),
        DeviceModel::AirPods3,
    );
    
    assert_eq!(*device.state(), DeviceState::Disconnected);
    
    device.set_state(DeviceState::Connecting);
    assert_eq!(*device.state(), DeviceState::Connecting);
    
    device.set_state(DeviceState::Connected);
    assert_eq!(*device.state(), DeviceState::Connected);
}

#[test]
fn test_device_metadata() {
    let mut device = Device::new(
        "test_001".to_string(),
        "Test AirPods".to_string(),
        DeviceModel::AirPods3,
    );
    
    device.set_metadata("firmware".to_string(), "7.1".to_string());
    assert_eq!(device.get_metadata("firmware"), Some("7.1"));
}
