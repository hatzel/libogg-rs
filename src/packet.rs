use ogg_sys::ogg_packet;
use std::fmt;

pub struct Packet {
    pub(crate) inner: ogg_packet,
}

impl Packet {
    pub fn new<'a, T: 'a + AsMut<[u8]>>(data: &mut T) -> Self {
        Packet {
            inner: ogg_packet {
                packet: data.as_mut().as_mut_ptr(),
                bytes: data.as_mut().len() as i64,
                b_o_s: 0,
                e_o_s: 0,
                granulepos: 0,
                packetno: 0,
            },
        }
    }

    pub fn get_bos(&self) -> i64 {
        self.inner.b_o_s
    }

    pub fn set_bos(&mut self, bos: i64) {
        self.inner.b_o_s = bos;
    }

    pub fn get_eos(&self) -> i64 {
        self.inner.e_o_s
    }

    pub fn set_eos(&mut self, eos: i64) {
        self.inner.e_o_s = eos;
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
