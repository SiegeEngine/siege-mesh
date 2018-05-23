pub use self::inner::*;

#[cfg(all(feature = "siege-math", not(feature = "cgmath"), not(feature = "nalgebra")))]
mod inner {
    pub use siege_math::{Point3, Vec4};
}

#[cfg(all(feature = "cgmath", not(feature = "siege-math"), not(feature = "nalgebra")))]
mod inner {
    pub use cgmath::Point3;
    pub use cgmath::Vector4 as Vec4;
}

#[cfg(all(feature = "nalgebra", not(feature = "siege-math"), not(feature = "cgmath")))]
mod inner {
    pub use nalgebra::Point3;
    pub use nalgebra::Vector4 as Vec4;
}
