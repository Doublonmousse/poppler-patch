// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TextAttributes(Boxed<ffi::PopplerTextAttributes>);

    match fn {
        copy => |ptr| ffi::poppler_text_attributes_copy(mut_override(ptr)),
        free => |ptr| ffi::poppler_text_attributes_free(ptr),
        type_ => || ffi::poppler_text_attributes_get_type(),
    }
}

impl TextAttributes {
    #[doc(alias = "poppler_text_attributes_new")]
    pub fn new() -> TextAttributes {
        unsafe { from_glib_full(ffi::poppler_text_attributes_new()) }
    }
}

impl Default for TextAttributes {
    fn default() -> Self {
        Self::new()
    }
}
