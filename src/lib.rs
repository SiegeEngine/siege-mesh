
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

#[macro_use]
extern crate error_chain;

#[cfg(feature="siege-math")]
extern crate siege_math;
#[cfg(feature="cgmath")]
extern crate cgmath;
#[cfg(feature="nalgebra")]
extern crate nalgebra;

pub mod math;
pub use self::math::*;

mod errors;
pub use errors::{Error, ErrorKind};

pub mod vertex;
pub use self::vertex::*;

pub mod mesh;
pub use self::mesh::*;
