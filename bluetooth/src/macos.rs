use core_bluetooth::central::{CentralEvent, CentralManager};
use core_bluetooth::uuid::Uuid;
use core_bluetooth::ManagerState;

use super::ServiceData;
use std::sync::mpsc::{self, Receiver, Sender};

pub struct BLEAdvertisementObserver {
    tx: Sender<ServiceData>,
    rx: Receiver<ServiceData>,
}

impl BLEAdvertisementObserver {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        BLEAdvertisementObserver { tx, rx }
    }

    pub fn receiver(&self) -> &Receiver<ServiceData> {
        &self.rx
    }

    pub fn start_scan(&self) {
        let tx = self.tx.clone();

        std::thread::spawn(move || {
            let (central, rx) = CentralManager::new();

            println!("waiting");

            for event in rx {
                let tx = tx.clone();
                Self::handle_event(event, tx, &central);
            }
        });
    }

    fn handle_event(event: CentralEvent, tx: Sender<ServiceData>, central: &CentralManager) {
        match event {
            CentralEvent::ManagerStateChanged { new_state }
                if new_state == ManagerState::PoweredOn =>
            {
                central.scan();
            }
            CentralEvent::PeripheralDiscovered {
                advertisement_data, ..
            } => {
                let uuid = Uuid::from_slice(&[0xfe, 0x95]);

                if let Some(data) = advertisement_data.service_data().get(uuid) {
                    tx.send(ServiceData(Vec::from(data)))
                        .expect("receiver was dropped");
                }
            }
            _ => (),
        }
    }
}
