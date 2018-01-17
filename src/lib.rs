
// Serialization:
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

#[macro_use]
extern crate error_chain;

extern crate siege_math;

mod errors;
pub use errors::{Error, ErrorKind};

pub mod vertex;
pub use self::vertex::*;

pub mod mesh;
pub use self::mesh::*;
