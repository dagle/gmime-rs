use std::convert::TryInto;
use std::io::ErrorKind::WriteZero;
use std::io::{Read, Error, Write};

use crate::Stream;
use crate::traits::StreamExt;
use glib::object::IsA;
use glib::translate::*;

pub trait StreamExtManual: 'static {
    #[doc(alias = "g_mime_stream_write")]
    fn write(&self, buf: &[u8]) -> isize;
}

impl !Send for Stream {}
impl !Sync for Stream {}

impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let size = StreamExtManual::write(self, buf);
        if size >= 0 {
            Ok(size.try_into().unwrap())
        } else {
            Err(Error::new(WriteZero, "Couldn't read from stream"))
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let size = StreamExt::flush(self);
        if size < 0 {
            Err(Error::new(WriteZero, "Couldn't flush stream"))
        } else {
            Ok(())
        }
    }
}

impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = StreamExt::read(self, buf);
        if size >= 0 {
            Ok(size.try_into().unwrap())
        } else {
            Err(Error::new(WriteZero, "Couldn't read from stream"))
        }
    }
}

impl<O: IsA<Stream>> StreamExtManual for O {
    fn write(&self, buf: &[u8]) -> isize {
        let len = buf.len() as usize;
        unsafe {
            ffi::g_mime_stream_write(
                self.as_ref().to_glib_none().0,
                buf.to_glib_none().0 as *const i8,
                len,
            )
        }
    }
}
