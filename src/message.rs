use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::mem;

use crate::Message;

pub trait MessageExtManual: 'static {
    #[doc(alias = "g_mime_message_partial_split_message")]
    fn split(&self, max_size: usize) -> Vec<Message>;
}

impl<O: IsA<Message>> MessageExtManual for O {
    fn split(&self, max_size: usize) -> Vec<Message> {
        unsafe {
            let mut n_parts = mem::MaybeUninit::uninit();
            let parts = ffi::g_mime_message_partial_split_message(
                self.as_ref().to_glib_none().0,
                max_size,
                n_parts.as_mut_ptr(),
            );
            let n_parts = n_parts.assume_init();
            FromGlibContainer::from_glib_full_num(parts, n_parts as usize)
        }
    }
}
