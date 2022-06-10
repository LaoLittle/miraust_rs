use jni::objects::GlobalRef;

use crate::Listener;

#[derive(Debug, Clone)]
#[repr(C)]
pub(crate) struct Managed {
    pub(crate) pointer: *mut (),
    t: u8,
}

#[no_mangle]
extern fn drop_res(res: *mut (), t: u8) {
    unsafe {
        match t {
            0 => {
                Box::from_raw(res as *mut GlobalRef);
            }
            11 => {
                Box::from_raw(res as *mut Listener);
            }
            _ => {}
        };
    }
}