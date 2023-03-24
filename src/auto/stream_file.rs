// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::Stream;
use glib::{prelude::*, translate::*};
use std::{fmt, ptr};

glib::wrapper! {
    #[doc(alias = "GMimeStreamFile")]
    pub struct StreamFile(Object<ffi::GMimeStreamFile, ffi::GMimeStreamFileClass>) @extends Stream;

    match fn {
        type_ => || ffi::g_mime_stream_file_get_type(),
    }
}

impl StreamFile {
    pub const NONE: Option<&'static StreamFile> = None;

    //#[doc(alias = "g_mime_stream_file_new")]
    //pub fn new(fp: /*Unimplemented*/Option<Basic: Pointer>) -> StreamFile {
    //    unsafe { TODO: call ffi:g_mime_stream_file_new() }
    //}

    //#[doc(alias = "g_mime_stream_file_new_with_bounds")]
    //#[doc(alias = "new_with_bounds")]
    //pub fn with_bounds(fp: /*Unimplemented*/Option<Basic: Pointer>, start: i64, end: i64) -> StreamFile {
    //    unsafe { TODO: call ffi:g_mime_stream_file_new_with_bounds() }
    //}

    #[doc(alias = "g_mime_stream_file_open")]
    pub fn open(path: &str, mode: &str) -> Result<Stream, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_stream_file_open(
                path.to_glib_none().0,
                mode.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub trait StreamFileExt: 'static {
    #[doc(alias = "g_mime_stream_file_get_owner")]
    #[doc(alias = "get_owner")]
    fn is_owner(&self) -> bool;

    #[doc(alias = "g_mime_stream_file_set_owner")]
    fn set_owner(&self, owner: bool);
}

impl<O: IsA<StreamFile>> StreamFileExt for O {
    fn is_owner(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_stream_file_get_owner(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_owner(&self, owner: bool) {
        unsafe {
            ffi::g_mime_stream_file_set_owner(self.as_ref().to_glib_none().0, owner.into_glib());
        }
    }
}

impl fmt::Display for StreamFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StreamFile")
    }
}
