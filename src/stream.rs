use crate::Stream;
use glib::object::IsA;
use glib::translate::*;

pub trait StreamExtManual: 'static {
    #[doc(alias = "g_mime_stream_write")]
    fn write(&self, buf: &[u8]) -> isize;
}

impl<O: IsA<Stream>> StreamExtManual for O {
    fn write(&self, buf: &[u8]) -> isize {
        let len = buf.len() as usize;
        unsafe {
            ffi::g_mime_stream_write(
                self.as_ref().to_glib_none().0,
                buf.to_glib_none().0 as *const u8,
                len,
            )
        }
    }
}
