// This file was generated by gir (https://github.com/gtk-rs/gir @ ad40c01)
// from gir-files (https://github.com/gtk-rs/gir-files @ 5b96546)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use gmime_sys;
use std::fmt;
use Stream;

glib_wrapper! {
    pub struct StreamNull(Object<gmime_sys::GMimeStreamNull, gmime_sys::GMimeStreamNullClass, StreamNullClass>) @extends Stream;

    match fn {
        get_type => || gmime_sys::g_mime_stream_null_get_type(),
    }
}

impl StreamNull {
    pub fn new() -> StreamNull {
        unsafe {
            Stream::from_glib_full(gmime_sys::g_mime_stream_null_new()).unsafe_cast()
        }
    }
}

impl Default for StreamNull {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_STREAM_NULL: Option<&StreamNull> = None;

pub trait StreamNullExt: 'static {
    fn get_count_newlines(&self) -> bool;

    fn set_count_newlines(&self, count: bool);
}

impl<O: IsA<StreamNull>> StreamNullExt for O {
    fn get_count_newlines(&self) -> bool {
        unsafe {
            from_glib(gmime_sys::g_mime_stream_null_get_count_newlines(self.as_ref().to_glib_none().0))
        }
    }

    fn set_count_newlines(&self, count: bool) {
        unsafe {
            gmime_sys::g_mime_stream_null_set_count_newlines(self.as_ref().to_glib_none().0, count.to_glib());
        }
    }
}

impl fmt::Display for StreamNull {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StreamNull")
    }
}
