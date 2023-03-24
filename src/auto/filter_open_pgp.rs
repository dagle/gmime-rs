// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::{Filter,OpenPGPData};
use glib::{prelude::*,translate::*};
use std::{fmt};

glib::wrapper! {
    #[doc(alias = "GMimeFilterOpenPGP")]
    pub struct FilterOpenPGP(Object<ffi::GMimeFilterOpenPGP, ffi::GMimeFilterOpenPGPClass>) @extends Filter;

    match fn {
        type_ => || ffi::g_mime_filter_openpgp_get_type(),
    }
}

impl FilterOpenPGP {
        pub const NONE: Option<&'static FilterOpenPGP> = None;
    

    #[doc(alias = "g_mime_filter_openpgp_new")]
    pub fn new() -> FilterOpenPGP {
        assert_initialized_main_thread!();
        unsafe {
            Filter::from_glib_full(ffi::g_mime_filter_openpgp_new()).unsafe_cast()
        }
    }
}

#[cfg(any(feature = "v3_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_2")))]
impl Default for FilterOpenPGP {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

pub trait FilterOpenPGPExt: 'static {
    #[doc(alias = "g_mime_filter_openpgp_get_begin_offset")]
    #[doc(alias = "get_begin_offset")]
    fn begin_offset(&self) -> i64;

    #[doc(alias = "g_mime_filter_openpgp_get_data_type")]
    #[doc(alias = "get_data_type")]
    fn data_type(&self) -> OpenPGPData;

    #[doc(alias = "g_mime_filter_openpgp_get_end_offset")]
    #[doc(alias = "get_end_offset")]
    fn end_offset(&self) -> i64;
}

impl<O: IsA<FilterOpenPGP>> FilterOpenPGPExt for O {
    fn begin_offset(&self) -> i64 {
        unsafe {
            ffi::g_mime_filter_openpgp_get_begin_offset(self.as_ref().to_glib_none().0)
        }
    }

    fn data_type(&self) -> OpenPGPData {
        unsafe {
            from_glib(ffi::g_mime_filter_openpgp_get_data_type(self.as_ref().to_glib_none().0))
        }
    }

    fn end_offset(&self) -> i64 {
        unsafe {
            ffi::g_mime_filter_openpgp_get_end_offset(self.as_ref().to_glib_none().0)
        }
    }
}

impl fmt::Display for FilterOpenPGP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FilterOpenPGP")
    }
}
