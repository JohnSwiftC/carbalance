use crate::route::Map;
use std::fs::File;
use std::path::Path;
pub fn load_map<T: AsRef<Path>>(path: T) -> Result<Map, std::io::Error> {
    let mut map = Map::new();

    let mut file = File::open(path)?;

    Ok(map)
}
