mod store;
use store::Store;

mod measurement;
use measurement::Measurement;

use rumble::api::{BDAddr, Central, CentralEvent, Peripheral, UUID};
use rumble::bluez::manager::Manager;

use time::{offset, OffsetDateTime};

use std::convert::TryFrom;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn measure() {
    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().next().unwrap();

    // reset the adapter -- clears out any errant state
    adapter = manager.down(&adapter).unwrap();
    adapter = manager.up(&adapter).unwrap();

    // connect to the adapter
    let central = adapter.connect().unwrap();

    let temp_addr = BDAddr {
        address: [0xCF, 0x82, 0xDD, 0xA8, 0x65, 0x4C],
    };

    find_device(&central, &temp_addr).unwrap();
    let temp = central.peripheral(temp_addr).unwrap();

    // connect to the device
    temp.connect().unwrap();

    // discover characteristics
    temp.discover_characteristics().unwrap();

    // find the characteristic we want
    let chars = temp.characteristics();

    let (tx, rx) = std::sync::mpsc::channel();

    let sensor_uuid = UUID::B128([
        0x6d, 0x66, 0x70, 0x44, 0x73, 0x66, 0x62, 0x75, 0x66, 0x45, 0x76, 0x64, 0x55, 0xaa, 0x6c,
        0x22,
    ]);

    for cmd in chars.iter() {
        if cmd.uuid == sensor_uuid {
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
            let store = Store::new("./data.db").unwrap();

            if let Err(err) = store.add_measurement(measurement, OffsetDateTime::now_utc()) {
                eprintln!("{}", err);
            }
        }
        Err(err) => eprintln!("{}", err),
    }

    temp.disconnect().unwrap();
}

fn find_device<C, P>(central: &C, addr_to_find: &BDAddr) -> Option<()>
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
    let store = Store::new("./data.db").unwrap();
    match store.measurements() {
        Ok(mut measurements) => {
            measurements.sort_by_key(|t| t.0);

            for (time, measurement) in measurements {
                println!(
                    "{}: {}",
                    time.to_offset(offset!(+3)).format("%c"),
                    measurement
                );
            }
        }
        Err(err) => eprintln!("cannot load measurements: {}", err),
    }
}
