// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Annot, AnnotMapping, Color, FindFlags, FormFieldMapping, ImageMapping, LinkMapping,
    PSFile, PageTransition, PrintFlags, Rectangle, SelectionStyle, TextAttributes,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "PopplerPage")]
    pub struct Page(Object<ffi::PopplerPage>);

    match fn {
        type_ => || ffi::poppler_page_get_type(),
    }
}

impl Page {
    #[doc(alias = "poppler_page_add_annot")]
    pub fn add_annot(&self, annot: &impl IsA<Annot>) {
        unsafe {
            ffi::poppler_page_add_annot(self.to_glib_none().0, annot.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "poppler_page_find_text")]
    pub fn find_text(&self, text: &str) -> Vec<Rectangle> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::poppler_page_find_text(
                self.to_glib_none().0,
                text.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_page_find_text_with_options")]
    pub fn find_text_with_options(&self, text: &str, options: FindFlags) -> Vec<Rectangle> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::poppler_page_find_text_with_options(
                self.to_glib_none().0,
                text.to_glib_none().0,
                options.into_glib(),
            ))
        }
    }

    #[doc(alias = "poppler_page_get_annot_mapping")]
    #[doc(alias = "get_annot_mapping")]
    pub fn annot_mapping(&self) -> Vec<AnnotMapping> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::poppler_page_get_annot_mapping(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_page_get_bounding_box")]
    pub fn get_bounding_box(&self, rect: &mut Rectangle) -> bool {
        unsafe {
            from_glib(ffi::poppler_page_get_bounding_box(
                self.to_glib_none().0,
                rect.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "poppler_page_get_duration")]
    #[doc(alias = "get_duration")]
    pub fn duration(&self) -> f64 {
        unsafe { ffi::poppler_page_get_duration(self.to_glib_none().0) }
    }

    #[doc(alias = "poppler_page_get_form_field_mapping")]
    #[doc(alias = "get_form_field_mapping")]
    pub fn form_field_mapping(&self) -> Vec<FormFieldMapping> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::poppler_page_get_form_field_mapping(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_page_get_image")]
    #[doc(alias = "get_image")]
    pub fn image(&self, image_id: i32) -> Option<cairo::Surface> {
        unsafe { from_glib_full(ffi::poppler_page_get_image(self.to_glib_none().0, image_id)) }
    }

    #[doc(alias = "poppler_page_get_image_mapping")]
    #[doc(alias = "get_image_mapping")]
    pub fn image_mapping(&self) -> Vec<ImageMapping> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::poppler_page_get_image_mapping(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_page_get_index")]
    #[doc(alias = "get_index")]
    pub fn index(&self) -> i32 {
        unsafe { ffi::poppler_page_get_index(self.to_glib_none().0) }
    }

    #[doc(alias = "poppler_page_get_label")]
    #[doc(alias = "get_label")]
    pub fn label(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::poppler_page_get_label(self.to_glib_none().0)) }
    }

    #[doc(alias = "poppler_page_get_link_mapping")]
    #[doc(alias = "get_link_mapping")]
    pub fn link_mapping(&self) -> Vec<LinkMapping> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::poppler_page_get_link_mapping(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_page_get_selected_region")]
    #[doc(alias = "get_selected_region")]
    pub fn selected_region(
        &self,
        scale: f64,
        style: SelectionStyle,
        selection: &mut Rectangle,
    ) -> Option<cairo::Region> {
        unsafe {
            from_glib_full(ffi::poppler_page_get_selected_region(
                self.to_glib_none().0,
                scale,
                style.into_glib(),
                selection.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "poppler_page_get_selected_text")]
    #[doc(alias = "get_selected_text")]
    pub fn selected_text(
        &self,
        style: SelectionStyle,
        selection: &mut Rectangle,
    ) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::poppler_page_get_selected_text(
                self.to_glib_none().0,
                style.into_glib(),
                selection.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "poppler_page_get_size")]
    #[doc(alias = "get_size")]
    pub fn size(&self) -> (f64, f64) {
        unsafe {
            let mut width = std::mem::MaybeUninit::uninit();
            let mut height = std::mem::MaybeUninit::uninit();
            ffi::poppler_page_get_size(
                self.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }

    #[doc(alias = "poppler_page_get_text")]
    #[doc(alias = "get_text")]
    pub fn text(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::poppler_page_get_text(self.to_glib_none().0)) }
    }

    #[doc(alias = "poppler_page_get_text_attributes")]
    #[doc(alias = "get_text_attributes")]
    pub fn text_attributes(&self) -> Vec<TextAttributes> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::poppler_page_get_text_attributes(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_page_get_text_attributes_for_area")]
    #[doc(alias = "get_text_attributes_for_area")]
    pub fn text_attributes_for_area(&self, area: &mut Rectangle) -> Vec<TextAttributes> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::poppler_page_get_text_attributes_for_area(
                self.to_glib_none().0,
                area.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "poppler_page_get_text_for_area")]
    #[doc(alias = "get_text_for_area")]
    pub fn text_for_area(&self, area: &mut Rectangle) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::poppler_page_get_text_for_area(
                self.to_glib_none().0,
                area.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "poppler_page_get_thumbnail")]
    #[doc(alias = "get_thumbnail")]
    pub fn thumbnail(&self) -> Option<cairo::Surface> {
        unsafe { from_glib_full(ffi::poppler_page_get_thumbnail(self.to_glib_none().0)) }
    }

    #[doc(alias = "poppler_page_get_thumbnail_size")]
    #[doc(alias = "get_thumbnail_size")]
    pub fn thumbnail_size(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut width = std::mem::MaybeUninit::uninit();
            let mut height = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::poppler_page_get_thumbnail_size(
                self.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            ));
            if ret {
                Some((width.assume_init(), height.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "poppler_page_get_transition")]
    #[doc(alias = "get_transition")]
    pub fn transition(&self) -> Option<PageTransition> {
        unsafe { from_glib_full(ffi::poppler_page_get_transition(self.to_glib_none().0)) }
    }

    #[doc(alias = "poppler_page_remove_annot")]
    pub fn remove_annot(&self, annot: &impl IsA<Annot>) {
        unsafe {
            ffi::poppler_page_remove_annot(self.to_glib_none().0, annot.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "poppler_page_render")]
    pub fn render(&self, cairo: &cairo::Context) {
        unsafe {
            ffi::poppler_page_render(self.to_glib_none().0, mut_override(cairo.to_glib_none().0));
        }
    }

    #[doc(alias = "poppler_page_render_for_printing")]
    pub fn render_for_printing(&self, cairo: &cairo::Context) {
        unsafe {
            ffi::poppler_page_render_for_printing(
                self.to_glib_none().0,
                mut_override(cairo.to_glib_none().0),
            );
        }
    }

    #[doc(alias = "poppler_page_render_for_printing_with_options")]
    pub fn render_for_printing_with_options(&self, cairo: &cairo::Context, options: PrintFlags) {
        unsafe {
            ffi::poppler_page_render_for_printing_with_options(
                self.to_glib_none().0,
                mut_override(cairo.to_glib_none().0),
                options.into_glib(),
            );
        }
    }

    #[doc(alias = "poppler_page_render_selection")]
    pub fn render_selection(
        &self,
        cairo: &cairo::Context,
        selection: &mut Rectangle,
        old_selection: &mut Rectangle,
        style: SelectionStyle,
        glyph_color: &mut Color,
        background_color: &mut Color,
    ) {
        unsafe {
            ffi::poppler_page_render_selection(
                self.to_glib_none().0,
                mut_override(cairo.to_glib_none().0),
                selection.to_glib_none_mut().0,
                old_selection.to_glib_none_mut().0,
                style.into_glib(),
                glyph_color.to_glib_none_mut().0,
                background_color.to_glib_none_mut().0,
            );
        }
    }

    #[doc(alias = "poppler_page_render_to_ps")]
    pub fn render_to_ps(&self, ps_file: &PSFile) {
        unsafe {
            ffi::poppler_page_render_to_ps(self.to_glib_none().0, ps_file.to_glib_none().0);
        }
    }

    #[doc(alias = "label")]
    pub fn connect_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<F: Fn(&Page) + 'static>(
            this: *mut ffi::PopplerPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::label\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_label_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
