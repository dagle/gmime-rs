// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8)
// DO NOT EDIT

use Filter;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FilterCharset(Object<ffi::GMimeFilterCharset, ffi::GMimeFilterCharsetClass>): Filter;

    match fn {
        get_type => || ffi::g_mime_filter_charset_get_type(),
    }
}

impl FilterCharset {
    pub fn new(from_charset: &str, to_charset: &str) -> FilterCharset {
        unsafe {
            Filter::from_glib_full(ffi::g_mime_filter_charset_new(from_charset.to_glib_none().0, to_charset.to_glib_none().0)).downcast_unchecked()
        }
    }
}