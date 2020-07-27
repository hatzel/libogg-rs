use ogg_sys::ogg_packet;
use std::fmt;

pub struct Packet {
    pub(crate) inner: ogg_packet,
}

impl Packet {
    pub fn new<'a, T: 'a + AsRef<[u8]>>(data: &T) -> Self {
        Packet {
            inner: ogg_packet {
                packet: data.as_ref().as_ptr() as *mut u8,
                bytes: data.as_ref().len() as i64,
                b_o_s: 0,
                e_o_s: 0,
                granulepos: 0,
                packetno: 0,
            },
        }
    }

    pub fn get_bos(&self) -> bool {
        self.inner.b_o_s == 1
    }

    pub fn set_bos(&mut self, bos: bool) {
        self.inner.b_o_s = if bos { 1 } else { 0 };
    }

    pub fn get_eos(&self) -> bool {
        self.inner.e_o_s == 1
    }

    pub fn set_eos(&mut self, eos: bool) {
        self.inner.e_o_s = if eos { 1 } else { 0 };
    }

    pub fn get_granulepos(&self) -> i64 {
        self.inner.granulepos
    }

    pub fn set_granulepos(&mut self, granulepos: i64) {
        self.inner.granulepos = granulepos;
    }

    pub fn get_packetno(&self) -> i64 {
        self.inner.packetno
    }

    pub fn set_packetno(&mut self, packetno: i64) {
        self.inner.packetno = packetno;
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Packet")
            .field("eos", &self.get_eos())
            .field("bos", &self.get_bos())
            .field("packetno", &self.get_packetno())
            .field("granulepos", &self.get_granulepos())
            .finish()
    }
}
