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
    pub struct StreamPipe(Object<gmime_sys::GMimeStreamPipe, gmime_sys::GMimeStreamPipeClass, StreamPipeClass>) @extends Stream;

    match fn {
        get_type => || gmime_sys::g_mime_stream_pipe_get_type(),
    }
}

impl StreamPipe {
    pub fn new(fd: i32) -> StreamPipe {
        unsafe {
            Stream::from_glib_full(gmime_sys::g_mime_stream_pipe_new(fd)).unsafe_cast()
        }
    }
}

pub const NONE_STREAM_PIPE: Option<&StreamPipe> = None;

pub trait StreamPipeExt: 'static {
    fn get_owner(&self) -> bool;

    fn set_owner(&self, owner: bool);
}

impl<O: IsA<StreamPipe>> StreamPipeExt for O {
    fn get_owner(&self) -> bool {
        unsafe {
            from_glib(gmime_sys::g_mime_stream_pipe_get_owner(self.as_ref().to_glib_none().0))
        }
    }

    fn set_owner(&self, owner: bool) {
        unsafe {
            gmime_sys::g_mime_stream_pipe_set_owner(self.as_ref().to_glib_none().0, owner.to_glib());
        }
    }
}

impl fmt::Display for StreamPipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StreamPipe")
    }
}
