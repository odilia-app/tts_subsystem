use crate::Error;
use speech_dispatcher_sys as spd;
use std::{ffi::CString, fmt, ptr};
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
    pub fn new(app_name: &str) -> Result<Speaker, Error> {
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
    pub fn speak_text(&self, text: &str, priority: Priority) -> Result<(), Error> {
        let text = CString::new(text).expect("slice shouldn't contain null bytes");
        let priority = priority as u32;
        let result;
        unsafe {
            result = spd::spd_say(self.con, priority, text.as_ptr().cast());
        }
        if result == -1 {
            return Err(Error::SpeechSynthError);
        }
        Ok(())
    }
    pub fn stop(&self) -> Result<(), Error> {
        let res = unsafe { spd::spd_stop(self.con) };
        if res == -1 {
            return Err(Error::StopSpeechError);
        }
        Ok(())
    }
    pub fn pause(&self) -> Result<(), Error> {
        let res = unsafe { spd::spd_pause(self.con) };
        if res == -1 {
            return Err(Error::TTSPauseResumeError);
        }
        Ok(())
    }
    pub fn resume(&self) -> Result<(), Error> {
        let res = unsafe { spd::spd_resume(self.con) };
        if res == -1 {
            return Err(Error::TTSPauseResumeError);
        }
        Ok(())
    }
    pub fn cancel(&self) -> Result<(), Error> {
        let res = unsafe { spd::spd_cancel(self.con) };
        if res == -1 {
            return Err(Error::SpeechCancelationError);
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

impl fmt::Write for Speaker {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.speak_text(text, Priority::Text)
            .map_err(|_| fmt::Error)
    }
}
