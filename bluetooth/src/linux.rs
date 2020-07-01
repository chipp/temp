use rumble::api::Central;
use rumble::bluez::{
    adapter::ConnectedAdapter, adapter::LEAdvertisingData, adapter::LEAdvertisingInfo,
    manager::Manager,
};

use super::ServiceData;
use std::sync::mpsc::{self, Receiver, Sender};

pub struct BLEAdvertisementObserver {
    tx: Sender<ServiceData>,
    rx: Receiver<ServiceData>,

    adapter: ConnectedAdapter,
}

impl BLEAdvertisementObserver {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        let manager = Manager::new().unwrap();

        let adapters = manager.adapters().unwrap();
        let mut adapter = adapters.into_iter().nth(0).unwrap();

        adapter = manager.down(&adapter).unwrap();
        adapter = manager.up(&adapter).unwrap();

        let central = adapter.connect().unwrap();

        BLEAdvertisementObserver {
            tx,
            rx,
            adapter: central,
        }
    }

    pub fn receiver(&self) -> &Receiver<ServiceData> {
        &self.rx
    }

    pub fn start_scan(&self) {
        let tx = self.tx.clone();
        self.adapter.on_advertisement(Box::new(move |info| {
            let tx = tx.clone();
            Self::handle_event(info, tx)
        }));
        self.adapter.start_scan().unwrap();
    }

    fn handle_event(info: LEAdvertisingInfo, tx: Sender<ServiceData>) {
        for data in info.data {
            if let LEAdvertisingData::ServiceData16(uuid, data) = data {
                if uuid == 0xfe95 {
                    tx.send(ServiceData(Vec::from(data)))
                        .expect("receiver was dropped");
                }
            }
        }
    }
}
