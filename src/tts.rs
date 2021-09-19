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

macro_rules! spd_return_err_if_fail {
    ($res: ident, $err: ident) => {
        if $res == -1 {
            Err(Error::$err)
        } else {
            Ok(())
        }
    };
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
            Err(Error::InitError)
        } else {
            Ok(Speaker { con })
        }
    }

    pub fn speak(&self, priority: Priority, text: &str) -> Result<(), Error> {
        let text = CString::new(text).expect("slice shouldn't contain null bytes");
        let priority = priority as u32;
        let res = unsafe { spd::spd_say(self.con, priority, text.as_ptr().cast()) };
        spd_return_err_if_fail!(res, SpeechSynthError)
    }

    pub fn stop(&self) -> Result<(), Error> {
        let res = unsafe { spd::spd_stop(self.con) };
        spd_return_err_if_fail!(res, StopSpeechError)
    }

    pub fn pause(&self) -> Result<(), Error> {
        let res = unsafe { spd::spd_pause(self.con) };
        spd_return_err_if_fail!(res, TTSPauseResumeError)
    }

    pub fn resume(&self) -> Result<(), Error> {
        let res = unsafe { spd::spd_resume(self.con) };
        spd_return_err_if_fail!(res, TTSPauseResumeError)
    }

    pub fn cancel(&self) -> Result<(), Error> {
        let res = unsafe { spd::spd_cancel(self.con) };
        spd_return_err_if_fail!(res, SpeechCancelationError)
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
        self.speak(Priority::Text, text).map_err(|_| fmt::Error)
    }
}
