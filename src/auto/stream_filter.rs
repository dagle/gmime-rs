// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::{Filter, Stream};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMimeStreamFilter")]
    pub struct StreamFilter(Object<ffi::GMimeStreamFilter, ffi::GMimeStreamFilterClass>) @extends Stream;

    match fn {
        type_ => || ffi::g_mime_stream_filter_get_type(),
    }
}

impl StreamFilter {
    pub const NONE: Option<&'static StreamFilter> = None;

    #[doc(alias = "g_mime_stream_filter_new")]
    pub fn new(stream: &impl IsA<Stream>) -> StreamFilter {
        skip_assert_initialized!();
        unsafe {
            Stream::from_glib_full(ffi::g_mime_stream_filter_new(
                stream.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

pub trait StreamFilterExt: 'static {
    #[doc(alias = "g_mime_stream_filter_add")]
    fn add(&self, filter: &impl IsA<Filter>) -> i32;

    #[doc(alias = "g_mime_stream_filter_get_owner")]
    #[doc(alias = "get_owner")]
    fn is_owner(&self) -> bool;

    #[doc(alias = "g_mime_stream_filter_remove")]
    fn remove(&self, id: i32);

    #[doc(alias = "g_mime_stream_filter_set_owner")]
    fn set_owner(&self, owner: bool);
}

impl<O: IsA<StreamFilter>> StreamFilterExt for O {
    fn add(&self, filter: &impl IsA<Filter>) -> i32 {
        unsafe {
            ffi::g_mime_stream_filter_add(
                self.as_ref().to_glib_none().0,
                filter.as_ref().to_glib_none().0,
            )
        }
    }

    fn is_owner(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_stream_filter_get_owner(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove(&self, id: i32) {
        unsafe {
            ffi::g_mime_stream_filter_remove(self.as_ref().to_glib_none().0, id);
        }
    }

    fn set_owner(&self, owner: bool) {
        unsafe {
            ffi::g_mime_stream_filter_set_owner(self.as_ref().to_glib_none().0, owner.into_glib());
        }
    }
}

impl fmt::Display for StreamFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StreamFilter")
    }
}
