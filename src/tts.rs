use std::{ffi::CString, ptr};
use speech_dispatcher_sys as spd;
use crate::errors::error;
pub struct Speaker {
    con: *mut spd::SPDConnection,
    }
impl Speaker {
    pub fn init(app_name: &str) -> Result<Speaker, Error> {
        let name = CString::new(app_name).expect("name should not contain null bytes");
        let con = unsafe {
            spd::spd_open(
                name.as_ptr().cast(),
                ptr::null(),
                ptr::null(),
                spd::SPDConnectionMode::SPD_MODE_SINGLE,
            )
        };
        
        if con.is_null(){
            return Err(Error::InitError);
        }
        let speaker = Speaker { con };
        Ok(speaker)    
    }
}
impl Drop for Speaker {
    fn drop(&mut self){
        unsafe{
            spd::spd_close(self.con);
        }
    }
}