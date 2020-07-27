use ogg_sys::ogg_page;
use std::slice;

#[derive(Debug)]
pub struct Page {
    pub header: Vec<u8>,
    pub body: Vec<u8>,
}

pub(crate) struct InternalPage(pub(crate) ogg_page);

impl InternalPage {
    pub fn get_header(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.0.header, self.0.header_len as usize) }
    }

    pub fn get_body(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.0.body, self.0.body_len as usize) }
    }
}
