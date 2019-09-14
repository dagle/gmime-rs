// This file was generated by gir (https://github.com/gtk-rs/gir @ 9e3cb65)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8+)
// DO NOT EDIT

use Stream;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use gmime_sys;
use std::fmt;

glib_wrapper! {
    pub struct StreamCat(Object<gmime_sys::GMimeStreamCat, gmime_sys::GMimeStreamCatClass, StreamCatClass>) @extends Stream;

    match fn {
        get_type => || gmime_sys::g_mime_stream_cat_get_type(),
    }
}

impl StreamCat {
    pub fn new() -> StreamCat {
        unsafe {
            Stream::from_glib_full(gmime_sys::g_mime_stream_cat_new()).unsafe_cast()
        }
    }
}

impl Default for StreamCat {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_STREAM_CAT: Option<&StreamCat> = None;

pub trait StreamCatExt: 'static {
    fn add_source<P: IsA<Stream>>(&self, source: &P) -> i32;
}

impl<O: IsA<StreamCat>> StreamCatExt for O {
    fn add_source<P: IsA<Stream>>(&self, source: &P) -> i32 {
        unsafe {
            gmime_sys::g_mime_stream_cat_add_source(self.as_ref().to_glib_none().0, source.as_ref().to_glib_none().0)
        }
    }
}

impl fmt::Display for StreamCat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StreamCat")
    }
}
