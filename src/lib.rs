#![cfg_attr(feature = "dox", feature(doc_cfg))]

#[allow(unused_imports)]
mod auto;
pub use auto::*;
pub use poppler_sys as ffi;
mod color;
mod document;
mod movie;
mod page_range;
mod point;
mod quadrilateral;
mod rectangle;
pub use page_range::PageRange;

pub mod functions {
    pub use super::auto::functions::*;
}
pub mod builders {
    pub use super::auto::builders::*;
}
pub use functions::*;

pub mod prelude {
    pub use super::auto::traits::*;
}
