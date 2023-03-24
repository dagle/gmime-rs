// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::{FormatOptions, Param, ParserOptions};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMimeParamList")]
    pub struct ParamList(Object<ffi::GMimeParamList, ffi::GMimeParamListClass>);

    match fn {
        type_ => || ffi::g_mime_param_list_get_type(),
    }
}

impl ParamList {
    pub const NONE: Option<&'static ParamList> = None;

    #[doc(alias = "g_mime_param_list_new")]
    pub fn new() -> ParamList {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::g_mime_param_list_new()) }
    }

    #[doc(alias = "g_mime_param_list_parse")]
    pub fn parse(options: &mut ParserOptions, str: &str) -> Option<ParamList> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::g_mime_param_list_parse(
                options.to_glib_none_mut().0,
                str.to_glib_none().0,
            ))
        }
    }
}

impl Default for ParamList {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ParamListExt: 'static {
    #[doc(alias = "g_mime_param_list_clear")]
    fn clear(&self);

    #[doc(alias = "g_mime_param_list_encode")]
    fn encode(&self, options: &mut FormatOptions, fold: bool, str: &mut glib::String);

    #[doc(alias = "g_mime_param_list_get_parameter")]
    #[doc(alias = "get_parameter")]
    fn parameter(&self, name: &str) -> Option<Param>;

    #[doc(alias = "g_mime_param_list_get_parameter_at")]
    #[doc(alias = "get_parameter_at")]
    fn parameter_at(&self, index: i32) -> Option<Param>;

    #[doc(alias = "g_mime_param_list_length")]
    fn length(&self) -> i32;

    #[doc(alias = "g_mime_param_list_remove")]
    fn remove(&self, name: &str) -> bool;

    #[doc(alias = "g_mime_param_list_remove_at")]
    fn remove_at(&self, index: i32) -> bool;

    #[doc(alias = "g_mime_param_list_set_parameter")]
    fn set_parameter(&self, name: &str, value: &str);
}

impl<O: IsA<ParamList>> ParamListExt for O {
    fn clear(&self) {
        unsafe {
            ffi::g_mime_param_list_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn encode(&self, options: &mut FormatOptions, fold: bool, str: &mut glib::String) {
        unsafe {
            ffi::g_mime_param_list_encode(
                self.as_ref().to_glib_none().0,
                options.to_glib_none_mut().0,
                fold.into_glib(),
                str.to_glib_none_mut().0,
            );
        }
    }

    fn parameter(&self, name: &str) -> Option<Param> {
        unsafe {
            from_glib_none(ffi::g_mime_param_list_get_parameter(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn parameter_at(&self, index: i32) -> Option<Param> {
        unsafe {
            from_glib_none(ffi::g_mime_param_list_get_parameter_at(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    fn length(&self) -> i32 {
        unsafe { ffi::g_mime_param_list_length(self.as_ref().to_glib_none().0) }
    }

    fn remove(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_mime_param_list_remove(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn remove_at(&self, index: i32) -> bool {
        unsafe {
            from_glib(ffi::g_mime_param_list_remove_at(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    fn set_parameter(&self, name: &str, value: &str) {
        unsafe {
            ffi::g_mime_param_list_set_parameter(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for ParamList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ParamList")
    }
}
