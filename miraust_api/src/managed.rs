use crate::RawPointerMut;

#[derive(Debug)]
#[repr(C)]
pub(crate) struct Managed {
    pub(crate) pointer: RawPointerMut,
    t: u8,
}

impl Managed {
    pub(crate) fn new(ptr: RawPointerMut, t: u8) -> Managed {
        Managed { pointer: ptr, t }
    }
}

impl Drop for Managed {
    fn drop(&mut self) {
        unsafe { drop_res(self.pointer, self.t) }
    }
}

#[link(name = "miraust_core")]
extern {
    fn drop_res(res: RawPointerMut, t: u8);
}