use crate::Error;
use speech_dispatcher_sys as spd;
use std::{ffi::CString, ptr};
//enums for tts and speech dispatcher specific things
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ConnectionMode {
    Single = spd::SPDConnectionMode::SPD_MODE_SINGLE,
    Threaded = spd::SPDConnectionMode::SPD_MODE_THREADED,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Priority {
    Important = spd::SPDPriority::SPD_IMPORTANT,
    Message = spd::SPDPriority::SPD_MESSAGE,
    Notification = spd::SPDPriority::SPD_NOTIFICATION,
    Progress = spd::SPDPriority::SPD_PROGRESS,
    Text = spd::SPDPriority::SPD_TEXT,
}

#[derive(Debug, PartialEq)]
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
                ConnectionMode::Single as u32,
            )
        };
        if con.is_null() {
            return Err(Error::InitError);
        }
        let speaker = Speaker { con };
        Ok(speaker)
    }
    pub fn speak_text(&self, text: &str) -> Result<(), Error> {
        let c_text=CString::new(text).expect("slice shouldn't contain null bytes");
        let result;
        unsafe{
            result=spd::spd_say(self.con, Priority::Text as u32, c_text.as_ptr().cast())
        };
        if result == 0{
            return Err(Error::SpeechSynthError);
        }
    Ok(())    
    }
}
impl Drop for Speaker {
    fn drop(&mut self) {
        unsafe {
            spd::spd_close(self.con);
        }
    }
}
