use rumble::api::{Central, CentralEvent, Peripheral, UUID};
use rumble::bluez::manager::Manager;
use std::sync::Arc;
// use std::thread;
// use std::time::Duration;

fn main() {
    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().nth(0).unwrap();

    // reset the adapter -- clears out any errant state
    adapter = manager.down(&adapter).unwrap();
    adapter = manager.up(&adapter).unwrap();

    // connect to the adapter
    let central = Arc::from(adapter.connect().unwrap());

    // start scanning for devices
    central.start_scan().unwrap();
    // instead of waiting, you can use central.on_event to be notified of
    // new devices

    let clone = Arc::clone(&central);
    central.on_event(Box::new(move |event| match event {
        CentralEvent::DeviceDiscovered(addr) => {
            let peripheral = clone.peripheral(addr).unwrap();

            if peripheral
                .properties()
                .local_name
                .iter()
                .any(|name| name.contains("MJ_HT_V1"))
            {
                println!("found")
            }
        }
        _ => (),
    }));
}
