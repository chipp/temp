use rumble::api::{BDAddr, Central, Peripheral, UUID};
use rumble::bluez::manager::Manager;
use std::time::Duration;

use temp_reader::*;

fn main() {
    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().next().unwrap();

    // reset the adapter -- clears out any errant state
    adapter = manager.down(&adapter).unwrap();
    adapter = manager.up(&adapter).unwrap();

    // connect to the adapter
    let central = adapter.connect().unwrap();

    let addr = BDAddr {
        address: [0xCF, 0x82, 0xDD, 0xA8, 0x65, 0x4C],
    };

    find_device(&central, &addr).unwrap();
    let temp = central.peripheral(addr).unwrap();

    // connect to the device
    println!("{}", temp);
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

            println!("subscribed {}", cmd.uuid);

            let tx = tx.clone();
            temp.on_notification(Box::new(move |notification| {
                tx.send(notification.value).unwrap()
            }));
        }
    }

    let data = rx.recv_timeout(Duration::from_secs(5)).unwrap();
    if let Ok(string) = String::from_utf8(data) {
        println!("{}", string);
    }

    temp.disconnect().unwrap();
}
