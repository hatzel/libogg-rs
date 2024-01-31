use std::convert::TryInto;
use std::fmt;
use std::mem;

use ogg_sys::ogg_stream_state;

use crate::packet::Packet;
use crate::page::{InternalPage, Page};

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

    #[allow(clippy::useless_conversion)]
    pub fn get_lacing_storage(&self) -> i64 {
        self.0.lacing_storage.into()
    }

    #[allow(clippy::useless_conversion)]
    pub fn get_lacing_fill(&self) -> i64 {
        self.0.lacing_fill.into()
    }

    #[allow(clippy::useless_conversion)]
    pub fn get_lacing_packet(&self) -> i64 {
        self.0.lacing_packet.into()
    }

    #[allow(clippy::useless_conversion)]
    pub fn get_lacing_returned(&self) -> i64 {
        self.0.lacing_returned.into()
    }

    #[allow(clippy::useless_conversion)]
    pub fn get_body_storage(&self) -> i64 {
        self.0.body_storage.into()
    }

    #[allow(clippy::useless_conversion)]
    pub fn get_body_fill(&self) -> i64 {
        self.0.body_fill.into()
    }

    #[allow(clippy::useless_conversion)]
    pub fn get_body_returned(&self) -> i64 {
        self.0.body_returned.into()
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

    #[allow(clippy::useless_conversion)]
    pub fn get_serialno(&self) -> i64 {
        self.0.serialno.into()
    }

    #[allow(clippy::useless_conversion)]
    pub fn get_pageno(&self) -> i64 {
        self.0.pageno.into()
    }

    #[allow(clippy::useless_conversion)]
    pub fn set_pageno(&mut self, new_pageno: i64) -> Result<(), crate::Error> {
        self.0.pageno = new_pageno
            .try_into()
            .map_err(|_| crate::Error::TypeCastingError)?;
        Ok(())
    }

    pub fn get_packetno(&self) -> i64 {
        self.0.packetno
    }

    pub fn get_granulepos(&self) -> i64 {
        self.0.granulepos
    }

    pub fn packetin(&mut self, packet: &mut Packet) {
        // Internally the data is copied so this is safe
        unsafe {
            ogg_sys::ogg_stream_packetin(&mut self.0, &mut packet.inner);
        }
    }

    /// Return the next page in the stream
    ///
    /// May return None if not enough data was added to the stream.
    /// To ensure that the last page contains all data use `flush` or `fill_flush`.
    pub fn pageout(&mut self) -> Option<Page> {
        let (ret, page) = unsafe {
            let mut page: ogg_sys::ogg_page = std::mem::MaybeUninit::uninit().assume_init();
            let ret = ogg_sys::ogg_stream_pageout(&mut self.0, &mut page);
            (ret, page)
        };
        if ret == 0 {
            None
        } else {
            let internal_page = InternalPage(page);
            Some(Page {
                header: internal_page.get_header().to_owned(),
                body: internal_page.get_body().to_owned(),
            })
        }
    }

    /// Returns None if no packages are available for flushing
    pub fn flush(&mut self) -> Option<Page> {
        let (ret, page) = unsafe {
            let mut page: ogg_sys::ogg_page = std::mem::MaybeUninit::uninit().assume_init();
            let ret = ogg_sys::ogg_stream_flush(&mut self.0, &mut page);
            (ret, page)
        };
        if ret == 0 {
            None
        } else {
            let internal_page = InternalPage(page);
            Some(Page {
                header: internal_page.get_header().to_owned(),
                body: internal_page.get_body().to_owned(),
            })
        }
    }

    pub fn flush_fill(&mut self) -> Option<Page> {
        let (ret, page) = unsafe {
            let mut page: ogg_sys::ogg_page = std::mem::MaybeUninit::uninit().assume_init();
            let ret = ogg_sys::ogg_stream_flush_fill(&mut self.0, &mut page, -1);
            (ret, page)
        };
        if ret == 0 {
            None
        } else {
            let internal_page = InternalPage(page);
            Some(Page {
                header: internal_page.get_header().to_owned(),
                body: internal_page.get_body().to_owned(),
            })
        }
    }

    /// Reset the stream state
    pub fn reset(&mut self) -> Result<(), crate::Error> {
        unsafe {
            if ogg_sys::ogg_stream_reset(&mut self.0) == 0 {
                Ok(())
            } else {
                Err(crate::Error::InternalError)
            }
        }
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
        unsafe {
            ogg_sys::ogg_stream_clear(&mut self.0);
        }
    }
}
