use crate::Movie;
#[cfg(feature = "v0_89")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_89")))]
use glib::translate::*;

impl Movie {
    #[cfg(feature = "v0_89")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_89")))]
    #[doc(alias = "poppler_movie_get_aspect")]
    #[doc(alias = "get_aspect")]
    pub fn aspect(&self) -> (i32, i32) {
        unsafe {
            let mut width = std::mem::MaybeUninit::uninit();
            let mut height = std::mem::MaybeUninit::uninit();
            crate::ffi::poppler_movie_get_aspect(
                self.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }
}
