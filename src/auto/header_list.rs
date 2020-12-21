// This file was generated by gir (https://github.com/gtk-rs/gir @ ad40c01)
// from gir-files (https://github.com/gtk-rs/gir-files @ 5b96546)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use gmime_sys;
use std::fmt;
use FormatOptions;
use Header;
use ParserOptions;
use Stream;

glib_wrapper! {
    pub struct HeaderList(Object<gmime_sys::GMimeHeaderList, gmime_sys::GMimeHeaderListClass, HeaderListClass>);

    match fn {
        get_type => || gmime_sys::g_mime_header_list_get_type(),
    }
}

impl HeaderList {
    pub fn new(options: Option<&ParserOptions>) -> HeaderList {
        unsafe {
            from_glib_full(gmime_sys::g_mime_header_list_new(mut_override(options.to_glib_none().0)))
        }
    }
}

pub const NONE_HEADER_LIST: Option<&HeaderList> = None;

pub trait HeaderListExt: 'static {
    fn append(&self, name: &str, value: &str, charset: &str);

    fn clear(&self);

    fn contains(&self, name: &str) -> bool;

    fn get_count(&self) -> i32;

    fn get_header(&self, name: &str) -> Option<Header>;

    fn get_header_at(&self, index: i32) -> Option<Header>;

    fn prepend(&self, name: &str, value: &str, charset: &str);

    fn remove(&self, name: &str) -> bool;

    fn remove_at(&self, index: i32);

    fn set(&self, name: &str, value: &str, charset: &str);

    fn to_string(&self, options: Option<&FormatOptions>) -> GString;

    fn write_to_stream<P: IsA<Stream>>(&self, options: Option<&FormatOptions>, stream: &P) -> isize;
}

impl<O: IsA<HeaderList>> HeaderListExt for O {
    fn append(&self, name: &str, value: &str, charset: &str) {
        unsafe {
            gmime_sys::g_mime_header_list_append(self.as_ref().to_glib_none().0, name.to_glib_none().0, value.to_glib_none().0, charset.to_glib_none().0);
        }
    }

    fn clear(&self) {
        unsafe {
            gmime_sys::g_mime_header_list_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn contains(&self, name: &str) -> bool {
        unsafe {
            from_glib(gmime_sys::g_mime_header_list_contains(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_count(&self) -> i32 {
        unsafe {
            gmime_sys::g_mime_header_list_get_count(self.as_ref().to_glib_none().0)
        }
    }

    fn get_header(&self, name: &str) -> Option<Header> {
        unsafe {
            from_glib_none(gmime_sys::g_mime_header_list_get_header(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_header_at(&self, index: i32) -> Option<Header> {
        unsafe {
            from_glib_none(gmime_sys::g_mime_header_list_get_header_at(self.as_ref().to_glib_none().0, index))
        }
    }

    fn prepend(&self, name: &str, value: &str, charset: &str) {
        unsafe {
            gmime_sys::g_mime_header_list_prepend(self.as_ref().to_glib_none().0, name.to_glib_none().0, value.to_glib_none().0, charset.to_glib_none().0);
        }
    }

    fn remove(&self, name: &str) -> bool {
        unsafe {
            from_glib(gmime_sys::g_mime_header_list_remove(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn remove_at(&self, index: i32) {
        unsafe {
            gmime_sys::g_mime_header_list_remove_at(self.as_ref().to_glib_none().0, index);
        }
    }

    fn set(&self, name: &str, value: &str, charset: &str) {
        unsafe {
            gmime_sys::g_mime_header_list_set(self.as_ref().to_glib_none().0, name.to_glib_none().0, value.to_glib_none().0, charset.to_glib_none().0);
        }
    }

    fn to_string(&self, options: Option<&FormatOptions>) -> GString {
        unsafe {
            from_glib_full(gmime_sys::g_mime_header_list_to_string(self.as_ref().to_glib_none().0, mut_override(options.to_glib_none().0)))
        }
    }

    fn write_to_stream<P: IsA<Stream>>(&self, options: Option<&FormatOptions>, stream: &P) -> isize {
        unsafe {
            gmime_sys::g_mime_header_list_write_to_stream(self.as_ref().to_glib_none().0, mut_override(options.to_glib_none().0), stream.as_ref().to_glib_none().0)
        }
    }
}

impl fmt::Display for HeaderList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HeaderList")
    }
}
