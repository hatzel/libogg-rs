use std::mem;
use std::fmt;

use ogg_sys::ogg_stream_state;

use crate::packet::Packet;
use crate::page::Page;

pub struct Stream(ogg_stream_state);

impl Stream {
    pub fn new(serial: i32) -> Self {
        unsafe {
            let mut ogg_stream_state = mem::MaybeUninit::uninit().assume_init();
            if ogg_sys::ogg_stream_init(&mut ogg_stream_state, serial) != 0 {
                panic!("Out of memory!");
            }
            Stream(ogg_stream_state)
        }
    }

    // pub body_data: *mut libc::c_uchar,
    // pub body_storage: libc::c_long,
    // pub body_fill: libc::c_long,
    // pub body_returned: libc::c_long,

    // pub lacing_vals: *mut libc::c_int,
    // pub granule_vals: *mut ogg_int64_t,
    pub fn get_lacing_storage(&self) -> i64 {
        self.0.lacing_storage
    }

    pub fn get_lacing_fill(&self) -> i64 {
        self.0.lacing_fill
    }

    pub fn get_lacing_packet(&self) -> i64 {
        self.0.lacing_packet
    }

    pub fn get_lacing_returned(&self) -> i64 {
        self.0.lacing_returned
    }

    pub fn get_header(&self) -> &[u8; 282] {
        &self.0.header
    }

    pub fn get_header_fill(&self) -> i32 {
        self.0.header_fill
    }

    pub fn get_eos(&self) -> i32 {
        self.0.e_o_s
    }

    pub fn get_bos(&self) -> i32 {
        self.0.b_o_s
    }

    pub fn get_serialno(&self) -> i64 {
        self.0.serialno
    }

    pub fn get_pageno(&self) -> i64 {
        self.0.pageno
    }

    pub fn get_packetno(&self) -> i64 {
        self.0.packetno
    }

    pub fn get_granulepos(&self) -> i64 {
        self.0.granulepos
    }

    pub fn packetin(&mut self, packet: &mut Packet) {
        // Internally the data is copied so this is safe
        unsafe { ogg_sys::ogg_stream_packetin(&mut self.0, &mut packet.inner); }
    }
}

impl fmt::Debug for Stream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Stream")
            .field("eos", &self.get_eos())
            .field("bos", &self.get_bos())
            .field("serialno", &self.get_serialno())
            .field("pageno", &self.get_pageno())
            .field("packetno", &self.get_packetno())
            .field("granulepos", &self.get_granulepos())
            .finish()
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        // Don't call destroy here, it will attempt to free self.0 itself
        unsafe { ogg_sys::ogg_stream_clear(&mut self.0); }
    }
}

impl Iterator for Stream {
    type Item = Page;

    fn next(&mut self) -> Option<Self::Item> {
        let (ret, p) = unsafe {
            let mut p: ogg_sys::ogg_page = std::mem::MaybeUninit::uninit().assume_init();
            let ret = ogg_sys::ogg_stream_pageout(&mut self.0, &mut p);
            (ret, p)
        };
        if ret == 0 {
            None
        } else {
            Some(Page(p))
        }
    }
}
