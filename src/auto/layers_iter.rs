// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Document, Layer};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LayersIter(Boxed<ffi::PopplerLayersIter>);

    match fn {
        copy => |ptr| ffi::poppler_layers_iter_copy(mut_override(ptr)),
        free => |ptr| ffi::poppler_layers_iter_free(ptr),
        type_ => || ffi::poppler_layers_iter_get_type(),
    }
}

impl LayersIter {
    #[doc(alias = "poppler_layers_iter_new")]
    pub fn new(document: &Document) -> LayersIter {
        unsafe { from_glib_full(ffi::poppler_layers_iter_new(document.to_glib_none().0)) }
    }

    #[doc(alias = "poppler_layers_iter_get_child")]
    #[doc(alias = "get_child")]
    #[must_use]
    pub fn child(&mut self) -> Option<LayersIter> {
        unsafe {
            from_glib_full(ffi::poppler_layers_iter_get_child(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "poppler_layers_iter_get_layer")]
    #[doc(alias = "get_layer")]
    pub fn layer(&mut self) -> Option<Layer> {
        unsafe {
            from_glib_full(ffi::poppler_layers_iter_get_layer(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "poppler_layers_iter_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::poppler_layers_iter_get_title(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "poppler_layers_iter_next")]
    pub fn next(&mut self) -> bool {
        unsafe { from_glib(ffi::poppler_layers_iter_next(self.to_glib_none_mut().0)) }
    }
}
