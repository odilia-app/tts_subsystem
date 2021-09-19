use std::{borrow::Cow, ffi::CStr, marker::PhantomData, os::raw::c_char};

pub struct OutputModuleIter<'a> {
    current: *mut *mut c_char,
    _marker: PhantomData<&'a crate::Speaker>,
}

impl OutputModuleIter<'_> {
    pub(crate) unsafe fn new(start: *mut *mut c_char) -> Self {
        Self {
            current: start,
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for OutputModuleIter<'a> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = unsafe { *self.current };
        if current.is_null() {
            None
        } else {
            let cstr;
            unsafe {
                self.current = self.current.offset(1);
                cstr = CStr::from_ptr(current);
            }
            Some(cstr.to_string_lossy())
        }
    }
}
