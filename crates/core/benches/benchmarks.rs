use criterion::{black_box, criterion_group, criterion_main, Criterion};
use librepods_core::*;

fn bench_device_creation(c: &mut Criterion) {
    c.bench_function("create_device", |b| {
        b.iter(|| {
            Device::new(
                black_box("test_id".to_string()),
                black_box("Test AirPods".to_string()),
                black_box(DeviceModel::AirPods3),
            )
        })
    });
}

fn bench_message_serialization(c: &mut Criterion) {
    c.bench_function("serialize_message", |b| {
        let msg = Message::new(MessageType::BatteryStatus, vec![50, 60, 70]);
        b.iter(|| msg.serialize())
    });
}

fn bench_engine_operations(c: &mut Criterion) {
    c.bench_function("register_device", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            let device = Device::new(
                black_box("test_id".to_string()),
                black_box("Test AirPods".to_string()),
                black_box(DeviceModel::AirPods3),
            );
            engine.register_device(device);
        })
    });
}

criterion_group!(benches, bench_device_creation, bench_message_serialization, bench_engine_operations);
criterion_main!(benches);
