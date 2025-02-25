// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AnnotCalloutLine(Boxed<ffi::PopplerAnnotCalloutLine>);

    match fn {
        copy => |ptr| ffi::poppler_annot_callout_line_copy(mut_override(ptr)),
        free => |ptr| ffi::poppler_annot_callout_line_free(ptr),
        type_ => || ffi::poppler_annot_callout_line_get_type(),
    }
}

impl AnnotCalloutLine {
    #[doc(alias = "poppler_annot_callout_line_new")]
    pub fn new() -> AnnotCalloutLine {
        unsafe { from_glib_full(ffi::poppler_annot_callout_line_new()) }
    }
}

impl Default for AnnotCalloutLine {
    fn default() -> Self {
        Self::new()
    }
}
