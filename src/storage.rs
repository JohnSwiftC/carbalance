use crate::route::{Map, Stretch};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
pub fn load_map<T: AsRef<Path>>(path: T) -> Result<Map, std::io::Error> {
    let mut map = Map::new();

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    while let Ok(size) = reader.read_until(b';', &mut buf) {
        if size == 0 {
            break;
        }

        match buf[0] {
            0 => handle_new_stretch(&buf[1..size - 1], &mut map),
            1 => handle_new_connection(&buf[1..size - 1], &mut map),
            n => {
                eprintln!("Invalid operation character: {}", n)
            }
        }

        buf.clear();
    }

    Ok(map)
}

fn handle_new_stretch(bytes: &[u8], map: &mut Map) {
    let mut left = 0;
    let mut right = 3;

    let mut speed: u32 = 0;
    let mut length: u32 = 0;
    let mut cars: u32 = 0;
    let mut cap: u32 = 0;

    speed = u32::from_le_bytes(bytes[0..=3].try_into().unwrap());
    length = u32::from_le_bytes(bytes[4..=7].try_into().unwrap());
    cars = u32::from_le_bytes(bytes[8..=11].try_into().unwrap());
    cap = u32::from_le_bytes(bytes[12..=15].try_into().unwrap());

    map.push(Stretch {
        speed,
        length,
        cars,
        cap,
    });
}

fn handle_new_connection(bytes: &[u8], map: &mut Map) {
    let mut left = 0;
    let mut right = 3;

    let from = u32::from_le_bytes(bytes[left..=right].try_into().unwrap());

    left += 4;
    right += 4;

    while right < bytes.len() {
        let to = u32::from_le_bytes(bytes[left..=right].try_into().unwrap());
        map.connect_one_way(from, to);

        left += 4;
        right += 4;
    }
}
