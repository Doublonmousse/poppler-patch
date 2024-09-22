use crate::Document;

#[cfg(feature = "v0_80")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_80")))]
use crate::{ffi, PageRange};

#[cfg(feature = "v0_80")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_80")))]
use glib::translate::*;

impl Document {
    #[cfg(feature = "v0_80")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_80")))]
    #[doc(alias = "poppler_document_get_print_page_ranges")]
    #[doc(alias = "get_print_page_ranges")]
    pub fn print_page_ranges(&self) -> &[PageRange] {
        unsafe {
            let mut n_ranges = std::mem::MaybeUninit::uninit();
            let pages = std::slice::from_raw_parts(
                ffi::poppler_document_get_print_page_ranges(
                    self.to_glib_none().0,
                    n_ranges.as_mut_ptr(),
                ),
                n_ranges.assume_init() as usize,
            );
            &*(pages as *const [ffi::PopplerPageRange] as *const [PageRange])
        }
    }
}
