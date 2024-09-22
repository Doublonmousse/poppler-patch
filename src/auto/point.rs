// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;

glib::wrapper! {
    pub struct Point(BoxedInline<ffi::PopplerPoint>);

    match fn {
        copy => |ptr| ffi::poppler_point_copy(mut_override(ptr)),
        free => |ptr| ffi::poppler_point_free(ptr),
        type_ => || ffi::poppler_point_get_type(),
    }
}

impl Point {
    #[doc(alias = "poppler_point_new")]
    pub fn new() -> Point {
        unsafe { from_glib_full(ffi::poppler_point_new()) }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new()
    }
}
