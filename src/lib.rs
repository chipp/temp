mod store;
use store::Store;

mod measurement;
use measurement::Measurement;

use rumble::api::{BDAddr, Central, CentralEvent, Peripheral, UUID};
use rumble::bluez::manager::Manager;

use chrono::Local;

use std::convert::TryFrom;
use std::sync::mpsc::channel;
use std::time::Duration;

const SENSOR_DATA_CHAR_UUID: UUID = UUID::B128([
    0x6d, 0x66, 0x70, 0x44, 0x73, 0x66, 0x62, 0x75, 0x66, 0x45, 0x76, 0x64, 0x55, 0xaa, 0x6c, 0x22,
]);

const FIRMWARE_CHAR_UUID: UUID = UUID::B16(0x2A26);
const BATTERY_CHAR_UUID: UUID = UUID::B16(0x2A19);

const DB_PATH: &str = "/var/db/temperature/";

pub fn measure() {
    let temp = initialize();

    let chars = temp.characteristics();

    let (tx, rx) = std::sync::mpsc::channel();

    for cmd in chars.iter() {
        if cmd.uuid == SENSOR_DATA_CHAR_UUID {
            temp.subscribe(&cmd).unwrap();

            let tx = tx.clone();
            temp.on_notification(Box::new(move |notification| {
                tx.send(notification.value).unwrap()
            }));
        }
    }

    match rx.recv_timeout(Duration::from_secs(5)) {
        Ok(data) => {
            let measurement = Measurement::try_from(data.as_slice()).unwrap();
            let store = Store::new(DB_PATH).unwrap();

            if let Err(err) = store.add_measurement(measurement, Local::now()) {
                eprintln!("{}", err);
            }
        }
        Err(err) => eprintln!("{}", err),
    }

    temp.disconnect().unwrap();
}

fn initialize() -> impl Peripheral {
    let manager = Manager::new().unwrap();
    let adapters = manager.adapters().unwrap();

    let mut adapter = adapters.into_iter().next().unwrap();
    adapter = manager.down(&adapter).unwrap();
    adapter = manager.up(&adapter).unwrap();
    let central = adapter.connect().unwrap();

    let sensor_addr = BDAddr {
        address: [0xCF, 0x82, 0xDD, 0xA8, 0x65, 0x4C],
    };

    discover_peripheral(&central, &sensor_addr).unwrap();

    let temp = central.peripheral(sensor_addr).unwrap();
    temp.connect().unwrap();
    temp.discover_characteristics().unwrap();

    temp
}

fn discover_peripheral<C, P>(central: &C, addr_to_find: &BDAddr) -> Option<()>
where
    C: Central<P>,
    P: Peripheral,
{
    let (tx, rx) = channel();
    central.on_event(Box::new(move |event| {
        let _ = tx.send(event);
    }));

    central.start_scan().unwrap();

    rx.into_iter().find_map(|event| match event {
        CentralEvent::DeviceUpdated(addr) if addr_to_find == &addr => Some(()),
        _ => None,
    })
}

pub fn list() {
    let store = Store::new(DB_PATH).unwrap();
    match store.measurements() {
        Ok(mut measurements) => {
            measurements.sort_by_key(|t| t.0);

            for (time, measurement) in measurements {
                println!(
                    "{}: {}",
                    time.with_timezone(&Local).format("%c"),
                    measurement
                );
            }
        }
        Err(err) => eprintln!("cannot load measurements: {}", err),
    }
}

pub fn info() {
    let temp = initialize();

    let chars = temp.characteristics();

    for chr in chars.iter() {
        if chr.uuid == FIRMWARE_CHAR_UUID {
            let mut data = temp.read(&chr).unwrap();
            data.remove(0);

            println!(
                "firmware: {}",
                std::str::from_utf8(&data).unwrap_or("unknown")
            );

            break;
        }
    }

    for chr in chars.iter() {
        if chr.uuid == BATTERY_CHAR_UUID {
            let data = temp.read(&chr).unwrap();
            println!("battery: {}", data[1]);
            break;
        }
    }

    temp.disconnect().unwrap();
}
