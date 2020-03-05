use ogg_sys::ogg_page;
use std::slice;
use std::fmt;

pub struct Page(pub(crate) ogg_page);

impl Page {
    pub fn get_header(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.0.header, self.0.header_len as usize) }
    }

    pub fn get_body(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.0.body, self.0.body_len as usize) }
    }
}

impl fmt::Debug for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Page")
            .field("header", &self.get_header())
            .field("body", &self.get_body())
            .finish()
    }
}
