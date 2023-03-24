// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::{Filter};
use glib::{prelude::*,translate::*};
use std::{fmt};

glib::wrapper! {
    #[doc(alias = "GMimeFilterHTML")]
    pub struct FilterHTML(Object<ffi::GMimeFilterHTML, ffi::GMimeFilterHTMLClass>) @extends Filter;

    match fn {
        type_ => || ffi::g_mime_filter_html_get_type(),
    }
}

impl FilterHTML {
        pub const NONE: Option<&'static FilterHTML> = None;
    

    #[doc(alias = "g_mime_filter_html_new")]
    pub fn new(flags: u32, colour: u32) -> FilterHTML {
        assert_initialized_main_thread!();
        unsafe {
            Filter::from_glib_full(ffi::g_mime_filter_html_new(flags, colour)).unsafe_cast()
        }
    }
}

impl fmt::Display for FilterHTML {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FilterHTML")
    }
}
