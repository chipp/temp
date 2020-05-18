use bluetooth::{BLEAdvertisementObserver, ServiceData};
use reader::*;

fn main() {
    env_logger::init();

    let observer = BLEAdvertisementObserver::new();
    observer.start_scan();

    println!("started scan");

    for data in observer.receiver().iter() {
        let ServiceData(bytes) = data;
        if let Some((mac_addr, event)) = parse_event(&bytes) {
            println!("{}: {}", mac_addr, event);
        }
    }
}
