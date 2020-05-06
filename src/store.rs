use super::measurement::Measurement;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use lmdb_rs::{DbFlags, DbHandle, EnvBuilder, Environment, MdbError};

const SENSOR_DATA_DB: &str = "sensor_data";

pub struct Store {
    env: Environment,
    sensor_data: DbHandle,
}

impl Store {
    pub fn new(path: &str) -> Result<Store, MdbError> {
        let env = EnvBuilder::new().max_dbs(3).open(path, 0o664)?;

        Ok(Store {
            sensor_data: env.create_db(SENSOR_DATA_DB, DbFlags::empty())?,
            env,
        })
    }
}

impl Store {
    pub fn add_measurement(
        &self,
        measurement: Measurement,
        time: DateTime<Local>,
    ) -> Result<(), MdbError> {
        let txn = self.env.new_transaction()?;

        {
            let db = txn.bind(&self.sensor_data);

            let time = time.timestamp();
            db.set(&time, &measurement.to_bytes())?;
        }

        txn.commit()?;

        Ok(())
    }

    pub fn measurements(&self) -> Result<Vec<(DateTime<Local>, Measurement)>, MdbError> {
        let reader = self.env.get_reader()?;
        let items = reader.bind(&self.sensor_data);

        let mut measurements = Vec::<(DateTime<Local>, Measurement)>::new();
        measurements.reserve(items.stat()?.ms_entries);

        for item in items.iter()? {
            let timestamp =
                Local.from_utc_datetime(&NaiveDateTime::from_timestamp(item.get_key(), 0));
            let measurement = Measurement::from_bytes(item.get_value());

            measurements.push((timestamp, measurement));
        }

        Ok(measurements)
    }
}

impl Measurement {
    fn to_bytes(&self) -> Vec<u8> {
        let temperature = self.temperature.to_ne_bytes();
        let humidity = self.humidity.to_ne_bytes();

        [temperature, humidity].concat()
    }

    fn from_bytes(bytes: &[u8]) -> Measurement {
        if bytes.len() != 8 {
            panic!("invalid bytes for measurement: {:?}", bytes);
        }

        let mut temperature = [0; 4];
        let mut humidity = [0; 4];

        temperature.copy_from_slice(&bytes[..4]);
        humidity.copy_from_slice(&bytes[4..]);

        Measurement {
            temperature: f32::from_ne_bytes(temperature),
            humidity: f32::from_ne_bytes(humidity),
        }
    }
}
