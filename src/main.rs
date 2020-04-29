use rumble::api::{Central, CentralEvent, Peripheral, UUID};
use rumble::bluez::manager::Manager;
use std::thread;
use std::time::Duration;

fn main() {
    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().nth(0).unwrap();

    // reset the adapter -- clears out any errant state
    adapter = manager.down(&adapter).unwrap();
    adapter = manager.up(&adapter).unwrap();

    // connect to the adapter
    let central = adapter.connect().unwrap();

    // start scanning for devices
    central.start_scan().unwrap();
    // instead of waiting, you can use central.on_event to be notified of
    // new devices

    thread::sleep(Duration::from_secs(2));

    // find the device we're interested in
    let temp = central
        .peripherals()
        .into_iter()
        .find(|p| {
            p.properties()
                .local_name
                .iter()
                .any(|name| name.contains("MJ_HT_V1"))
        })
        .unwrap();

    // connect to the device
    temp.connect().unwrap();

    // discover characteristics
    temp.discover_characteristics().unwrap();

    // find the characteristic we want
    let chars = temp.characteristics();
    let cmd_char = chars.iter().find(|c| c.uuid == UUID::B16(0x0024)).unwrap();

    let vec = temp.read(&cmd_char).unwrap();

    println!("{}", String::from_utf8(vec).unwrap());

    // // dance party
    // let mut rng = thread_rng();
    // for _ in 0..20 {
    //     let color_cmd = vec![0x56, rng.gen(), rng.gen(), rng.gen(), 0x00, 0xF0, 0xAA];
    //     light.command(&cmd_char, &color_cmd).unwrap();
    //     thread::sleep(Duration::from_millis(200));
    // }
}
