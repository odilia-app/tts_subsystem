use std::{borrow::Cow, ffi::CStr, marker::PhantomData, os::raw::c_char, ptr::NonNull};

use speech_dispatcher_sys as spd;

use crate::Voice;

pub struct OutputModuleIter<'a> {
    current: NonNull<*mut c_char>,
    _marker: PhantomData<&'a crate::Speaker>,
}

impl OutputModuleIter<'_> {
    pub(crate) fn new(start: NonNull<*mut c_char>) -> Self {
        Self {
            current: start,
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for OutputModuleIter<'a> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = unsafe { *(self.current.as_ptr()) };
        if current.is_null() {
            None
        } else {
            let cstr;
            unsafe {
                // Safety: The `char **` will not be NULL, but the `char *` could be
                self.current = NonNull::new_unchecked(self.current.as_ptr().offset(1));
                // Safety: At this point, we know `current` is not NULL.
                cstr = CStr::from_ptr(current);
            }
            Some(cstr.to_string_lossy())
        }
    }
}

pub struct SynthesisVoiceIter<'a> {
    current: NonNull<*mut spd::SPDVoice>,
    _marker: PhantomData<&'a crate::Speaker>,
}

impl SynthesisVoiceIter<'_> {
    pub(crate) fn new(start: NonNull<*mut spd::SPDVoice>) -> Self {
        Self {
            current: start,
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for SynthesisVoiceIter<'a> {
    type Item = Voice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = unsafe { *(self.current.as_ptr()) };
        if current.is_null() {
            None
        } else {
            unsafe {
                // Safety: The `char **` will not be NULL, but the `char *` could be
                self.current = NonNull::new_unchecked(self.current.as_ptr().offset(1));
            }
            Some(Voice::from(unsafe { *current }))
        }
    }
}
