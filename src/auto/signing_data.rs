// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, CertificateInfo, Color, Rectangle};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SigningData(Boxed<ffi::PopplerSigningData>);

    match fn {
        copy => |ptr| ffi::poppler_signing_data_copy(ptr),
        free => |ptr| ffi::poppler_signing_data_free(ptr),
        type_ => || ffi::poppler_signing_data_get_type(),
    }
}

impl SigningData {
    #[doc(alias = "poppler_signing_data_new")]
    pub fn new() -> SigningData {
        unsafe { from_glib_full(ffi::poppler_signing_data_new()) }
    }

    #[doc(alias = "poppler_signing_data_get_background_color")]
    #[doc(alias = "get_background_color")]
    pub fn background_color(&self) -> Option<Color> {
        unsafe {
            from_glib_none(ffi::poppler_signing_data_get_background_color(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_signing_data_get_border_color")]
    #[doc(alias = "get_border_color")]
    pub fn border_color(&self) -> Option<Color> {
        unsafe {
            from_glib_none(ffi::poppler_signing_data_get_border_color(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_signing_data_get_border_width")]
    #[doc(alias = "get_border_width")]
    pub fn border_width(&self) -> f64 {
        unsafe { ffi::poppler_signing_data_get_border_width(self.to_glib_none().0) }
    }

    #[doc(alias = "poppler_signing_data_get_certificate_info")]
    #[doc(alias = "get_certificate_info")]
    pub fn certificate_info(&self) -> Option<CertificateInfo> {
        unsafe {
            from_glib_none(ffi::poppler_signing_data_get_certificate_info(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_signing_data_get_destination_filename")]
    #[doc(alias = "get_destination_filename")]
    pub fn destination_filename(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::poppler_signing_data_get_destination_filename(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_signing_data_get_document_owner_password")]
    #[doc(alias = "get_document_owner_password")]
    pub fn document_owner_password(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::poppler_signing_data_get_document_owner_password(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_signing_data_get_document_user_password")]
    #[doc(alias = "get_document_user_password")]
    pub fn document_user_password(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::poppler_signing_data_get_document_user_password(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_signing_data_get_field_partial_name")]
    #[doc(alias = "get_field_partial_name")]
    pub fn field_partial_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::poppler_signing_data_get_field_partial_name(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_signing_data_get_font_color")]
    #[doc(alias = "get_font_color")]
    pub fn font_color(&self) -> Option<Color> {
        unsafe {
            from_glib_none(ffi::poppler_signing_data_get_font_color(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_signing_data_get_font_size")]
    #[doc(alias = "get_font_size")]
    pub fn font_size(&self) -> f64 {
        unsafe { ffi::poppler_signing_data_get_font_size(self.to_glib_none().0) }
    }

    #[doc(alias = "poppler_signing_data_get_image_path")]
    #[doc(alias = "get_image_path")]
    pub fn image_path(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::poppler_signing_data_get_image_path(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_signing_data_get_left_font_size")]
    #[doc(alias = "get_left_font_size")]
    pub fn left_font_size(&self) -> f64 {
        unsafe { ffi::poppler_signing_data_get_left_font_size(self.to_glib_none().0) }
    }

    #[doc(alias = "poppler_signing_data_get_location")]
    #[doc(alias = "get_location")]
    pub fn location(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::poppler_signing_data_get_location(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_signing_data_get_page")]
    #[doc(alias = "get_page")]
    pub fn page(&self) -> i32 {
        unsafe { ffi::poppler_signing_data_get_page(self.to_glib_none().0) }
    }

    #[doc(alias = "poppler_signing_data_get_password")]
    #[doc(alias = "get_password")]
    pub fn password(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::poppler_signing_data_get_password(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_signing_data_get_reason")]
    #[doc(alias = "get_reason")]
    pub fn reason(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::poppler_signing_data_get_reason(self.to_glib_none().0)) }
    }

    #[doc(alias = "poppler_signing_data_get_signature_rectangle")]
    #[doc(alias = "get_signature_rectangle")]
    pub fn signature_rectangle(&self) -> Option<Rectangle> {
        unsafe {
            from_glib_none(ffi::poppler_signing_data_get_signature_rectangle(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_signing_data_get_signature_text")]
    #[doc(alias = "get_signature_text")]
    pub fn signature_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::poppler_signing_data_get_signature_text(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_signing_data_get_signature_text_left")]
    #[doc(alias = "get_signature_text_left")]
    pub fn signature_text_left(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::poppler_signing_data_get_signature_text_left(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "poppler_signing_data_set_background_color")]
    pub fn set_background_color(&mut self, background_color: &Color) {
        unsafe {
            ffi::poppler_signing_data_set_background_color(
                self.to_glib_none_mut().0,
                background_color.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "poppler_signing_data_set_border_color")]
    pub fn set_border_color(&mut self, border_color: &Color) {
        unsafe {
            ffi::poppler_signing_data_set_border_color(
                self.to_glib_none_mut().0,
                border_color.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "poppler_signing_data_set_border_width")]
    pub fn set_border_width(&mut self, border_width: f64) {
        unsafe {
            ffi::poppler_signing_data_set_border_width(self.to_glib_none_mut().0, border_width);
        }
    }

    #[doc(alias = "poppler_signing_data_set_certificate_info")]
    pub fn set_certificate_info(&mut self, certificate_info: &CertificateInfo) {
        unsafe {
            ffi::poppler_signing_data_set_certificate_info(
                self.to_glib_none_mut().0,
                certificate_info.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "poppler_signing_data_set_destination_filename")]
    pub fn set_destination_filename(&mut self, filename: &str) {
        unsafe {
            ffi::poppler_signing_data_set_destination_filename(
                self.to_glib_none_mut().0,
                filename.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "poppler_signing_data_set_document_owner_password")]
    pub fn set_document_owner_password(&mut self, document_owner_password: &str) {
        unsafe {
            ffi::poppler_signing_data_set_document_owner_password(
                self.to_glib_none_mut().0,
                document_owner_password.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "poppler_signing_data_set_document_user_password")]
    pub fn set_document_user_password(&mut self, document_user_password: &str) {
        unsafe {
            ffi::poppler_signing_data_set_document_user_password(
                self.to_glib_none_mut().0,
                document_user_password.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "poppler_signing_data_set_field_partial_name")]
    pub fn set_field_partial_name(&mut self, field_partial_name: &str) {
        unsafe {
            ffi::poppler_signing_data_set_field_partial_name(
                self.to_glib_none_mut().0,
                field_partial_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "poppler_signing_data_set_font_color")]
    pub fn set_font_color(&mut self, font_color: &Color) {
        unsafe {
            ffi::poppler_signing_data_set_font_color(
                self.to_glib_none_mut().0,
                font_color.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "poppler_signing_data_set_font_size")]
    pub fn set_font_size(&mut self, font_size: f64) {
        unsafe {
            ffi::poppler_signing_data_set_font_size(self.to_glib_none_mut().0, font_size);
        }
    }

    #[doc(alias = "poppler_signing_data_set_image_path")]
    pub fn set_image_path(&mut self, image_path: &str) {
        unsafe {
            ffi::poppler_signing_data_set_image_path(
                self.to_glib_none_mut().0,
                image_path.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "poppler_signing_data_set_left_font_size")]
    pub fn set_left_font_size(&mut self, font_size: f64) {
        unsafe {
            ffi::poppler_signing_data_set_left_font_size(self.to_glib_none_mut().0, font_size);
        }
    }

    #[doc(alias = "poppler_signing_data_set_location")]
    pub fn set_location(&mut self, location: &str) {
        unsafe {
            ffi::poppler_signing_data_set_location(
                self.to_glib_none_mut().0,
                location.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "poppler_signing_data_set_page")]
    pub fn set_page(&mut self, page: i32) {
        unsafe {
            ffi::poppler_signing_data_set_page(self.to_glib_none_mut().0, page);
        }
    }

    #[doc(alias = "poppler_signing_data_set_password")]
    pub fn set_password(&mut self, password: &str) {
        unsafe {
            ffi::poppler_signing_data_set_password(
                self.to_glib_none_mut().0,
                password.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "poppler_signing_data_set_reason")]
    pub fn set_reason(&mut self, reason: &str) {
        unsafe {
            ffi::poppler_signing_data_set_reason(
                self.to_glib_none_mut().0,
                reason.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "poppler_signing_data_set_signature_rectangle")]
    pub fn set_signature_rectangle(&mut self, signature_rect: &Rectangle) {
        unsafe {
            ffi::poppler_signing_data_set_signature_rectangle(
                self.to_glib_none_mut().0,
                signature_rect.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "poppler_signing_data_set_signature_text")]
    pub fn set_signature_text(&mut self, signature_text: &str) {
        unsafe {
            ffi::poppler_signing_data_set_signature_text(
                self.to_glib_none_mut().0,
                signature_text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "poppler_signing_data_set_signature_text_left")]
    pub fn set_signature_text_left(&mut self, signature_text_left: &str) {
        unsafe {
            ffi::poppler_signing_data_set_signature_text_left(
                self.to_glib_none_mut().0,
                signature_text_left.to_glib_none().0,
            );
        }
    }
}

#[cfg(feature = "v23_7")]
#[cfg_attr(docsrs, doc(cfg(feature = "v23_7")))]
impl Default for SigningData {
    fn default() -> Self {
        Self::new()
    }
}
