use std::{io::Error, path::Path};

use serde::Deserialize;

pub fn deser<T: for<'a> Deserialize<'a>>(path: impl AsRef<Path>) -> Result<T, Error> {
    let raw = std::fs::read_to_string(path)?;
    serde_json::from_str(&raw).map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))
}
