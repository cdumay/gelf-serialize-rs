extern crate serde;

#[macro_use]
extern crate serde_derive;

mod error;
mod ser;

const GELF_FIELDS: [&'static str; 9] = ["version", "host", "short_message", "full_message", "timestamp", "level", "facility", "line", "file"];
