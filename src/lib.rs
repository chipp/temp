use rumble::api::{BDAddr, Central, CentralEvent, Peripheral};
use std::sync::mpsc::channel;

pub fn find_device<C, P>(central: &C, addr_to_find: &BDAddr) -> Option<()>
where
    C: Central<P>,
    P: Peripheral,
{
    let (tx, rx) = channel();
    central.on_event(Box::new(move |event| {
        let _ = tx.send(event);
    }));

    central.start_scan().unwrap();

    for event in rx {
        match event {
            CentralEvent::DeviceUpdated(addr) if addr_to_find == &addr => {
                println!("found {}", addr);
                return Some(());
            }
            _ => (),
        }
    }

    None
}
