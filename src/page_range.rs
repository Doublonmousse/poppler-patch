#[repr(transparent)]
pub struct PageRange(crate::ffi::PopplerPageRange);

impl PageRange {
    pub fn start_page(&self) -> i32 {
        self.0.start_page
    }

    pub fn end_page(&self) -> i32 {
        self.0.end_page
    }
}
