//! Read flatbuffers written with one version of the `BatteryState` schema using
//! code generated from another version.
//!
//! - `v1` reports `charge_percent` from 0 to 100.
//! - `v2` deprecates `charge_percent` and adds `charge_state` from 0.0 to
//!   1.0 and `temperature`.

use flatbuffers::FlatBufferBuilder;

#[allow(dead_code, unsafe_code, clippy::all, clippy::pedantic)]
mod generated {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/v1/battery_state_generated.rs"));
    }

    pub mod v2 {
        include!(concat!(env!("OUT_DIR"), "/v2/battery_state_generated.rs"));
    }
}

use generated::{v1, v2};

const VOLTAGE: f32 = 24.1;
const CURRENT: f32 = 1.5;
const CHARGE_PERCENT: f32 = 87.0;
const CHARGE_STATE: f32 = 0.87;
const TEMPERATURE: f32 = 35.0;

fn write_v1_battery_state() -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();
    let battery_state = v1::BatteryState::create(
        &mut builder,
        &v1::BatteryStateArgs {
            voltage: VOLTAGE,
            current: CURRENT,
            charge_percent: CHARGE_PERCENT,
        },
    );
    builder.finish(battery_state, None);
    builder.finished_data().to_vec()
}

fn write_v2_battery_state() -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();
    // charge_percent is deprecated. It cannot be set in v2::BatteryStateArgs.
    let battery_state = v2::BatteryState::create(
        &mut builder,
        &v2::BatteryStateArgs {
            voltage: VOLTAGE,
            current: CURRENT,
            charge_state: CHARGE_STATE,
            temperature: Some(TEMPERATURE),
        },
    );
    builder.finish(battery_state, None);
    builder.finished_data().to_vec()
}

fn print_table(writer: &str, reader: &str, rows: &[(&str, String, String)]) {
    let written = format!("written by {writer}");
    let read = format!("read by {reader}");
    println!("  {:<16}{written:<16}{read}", "field");
    for (field, written, read) in rows {
        println!("  {field:<16}{written:<16}{read}");
    }
}

fn main() {
    let v1_data = write_v1_battery_state();
    let v2_data = write_v2_battery_state();

    println!("Backwards compatible: v2 code reads v1 data.");
    println!();
    let b = v2::root_as_battery_state(&v1_data).unwrap();
    print_table(
        "v1",
        "v2",
        &[
            (
                "voltage",
                format!("{VOLTAGE:?}"),
                format!("{:?}", b.voltage()),
            ),
            (
                "current",
                format!("{CURRENT:?}"),
                format!("{:?}", b.current()),
            ),
            (
                "charge_percent",
                format!("{CHARGE_PERCENT:?}"),
                "no accessor".to_owned(),
            ),
            (
                "charge_state",
                "absent".to_owned(),
                format!("{:?} (default)", b.charge_state()),
            ),
            (
                "temperature",
                "absent".to_owned(),
                format!("{:?} (default)", b.temperature()),
            ),
        ],
    );
    println!();

    println!("Forwards compatible: v1 code reads v2 data.");
    println!();
    let b = v1::root_as_battery_state(&v2_data).unwrap();
    print_table(
        "v2",
        "v1",
        &[
            (
                "voltage",
                format!("{VOLTAGE:?}"),
                format!("{:?}", b.voltage()),
            ),
            (
                "current",
                format!("{CURRENT:?}"),
                format!("{:?}", b.current()),
            ),
            (
                "charge_percent",
                "absent".to_owned(),
                format!("{:?} (default)", b.charge_percent()),
            ),
            (
                "charge_state",
                format!("{CHARGE_STATE:?}"),
                "unknown to v1".to_owned(),
            ),
            (
                "temperature",
                format!("{TEMPERATURE:?}"),
                "unknown to v1".to_owned(),
            ),
        ],
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v2_reads_v1() {
        let data = write_v1_battery_state();
        let b = v2::root_as_battery_state(&data).unwrap();
        assert_eq!(b.voltage(), VOLTAGE);
        assert_eq!(b.charge_state(), 0.0_f32);
        assert_eq!(b.temperature(), None);
    }

    #[test]
    fn v1_reads_v2() {
        let data = write_v2_battery_state();
        let b = v1::root_as_battery_state(&data).unwrap();
        assert_eq!(b.voltage(), VOLTAGE);
        assert_eq!(b.charge_percent(), 0.0_f32);
    }
}
