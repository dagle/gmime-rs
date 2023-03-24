// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::{Filter};
use glib::{prelude::*,translate::*};
use std::{fmt};

glib::wrapper! {
    #[doc(alias = "GMimeFilterCharset")]
    pub struct FilterCharset(Object<ffi::GMimeFilterCharset, ffi::GMimeFilterCharsetClass>) @extends Filter;

    match fn {
        type_ => || ffi::g_mime_filter_charset_get_type(),
    }
}

impl FilterCharset {
        pub const NONE: Option<&'static FilterCharset> = None;
    

    #[doc(alias = "g_mime_filter_charset_new")]
    pub fn new(from_charset: &str, to_charset: &str) -> FilterCharset {
        assert_initialized_main_thread!();
        unsafe {
            Filter::from_glib_full(ffi::g_mime_filter_charset_new(from_charset.to_glib_none().0, to_charset.to_glib_none().0)).unsafe_cast()
        }
    }
}

impl fmt::Display for FilterCharset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FilterCharset")
    }
}
