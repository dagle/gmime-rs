// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::Stream;
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMimeStreamMmap")]
    pub struct StreamMmap(Object<ffi::GMimeStreamMmap, ffi::GMimeStreamMmapClass>) @extends Stream;

    match fn {
        type_ => || ffi::g_mime_stream_mmap_get_type(),
    }
}

impl StreamMmap {
    pub const NONE: Option<&'static StreamMmap> = None;

    #[doc(alias = "g_mime_stream_mmap_new")]
    pub fn new(fd: i32, prot: i32, flags: i32) -> StreamMmap {
        assert_initialized_main_thread!();
        unsafe {
            Stream::from_glib_full(ffi::g_mime_stream_mmap_new(fd, prot, flags)).unsafe_cast()
        }
    }

    #[doc(alias = "g_mime_stream_mmap_new_with_bounds")]
    #[doc(alias = "new_with_bounds")]
    pub fn with_bounds(fd: i32, prot: i32, flags: i32, start: i64, end: i64) -> StreamMmap {
        assert_initialized_main_thread!();
        unsafe {
            Stream::from_glib_full(ffi::g_mime_stream_mmap_new_with_bounds(
                fd, prot, flags, start, end,
            ))
            .unsafe_cast()
        }
    }
}

pub trait StreamMmapExt: 'static {
    #[cfg(any(feature = "v3_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_2")))]
    #[doc(alias = "g_mime_stream_mmap_get_owner")]
    #[doc(alias = "get_owner")]
    fn is_owner(&self) -> bool;

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_2")))]
    #[doc(alias = "g_mime_stream_mmap_set_owner")]
    fn set_owner(&self, owner: bool);
}

impl<O: IsA<StreamMmap>> StreamMmapExt for O {
    #[cfg(any(feature = "v3_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_2")))]
    fn is_owner(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_stream_mmap_get_owner(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_2")))]
    fn set_owner(&self, owner: bool) {
        unsafe {
            ffi::g_mime_stream_mmap_set_owner(self.as_ref().to_glib_none().0, owner.into_glib());
        }
    }
}

impl fmt::Display for StreamMmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StreamMmap")
    }
}
