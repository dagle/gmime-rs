// This file was generated by gir (https://github.com/gtk-rs/gir @ 2f0a317)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8)
// DO NOT EDIT

use FormatOptions;
use ParamList;
use ParserOptions;
use ffi;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct ContentType(Object<ffi::GMimeContentType, ffi::GMimeContentTypeClass, ContentTypeClass>);

    match fn {
        get_type => || ffi::g_mime_content_type_get_type(),
    }
}

impl ContentType {
    pub fn new(type_: &str, subtype: &str) -> ContentType {
        unsafe {
            from_glib_full(ffi::g_mime_content_type_new(type_.to_glib_none().0, subtype.to_glib_none().0))
        }
    }

    pub fn parse<'a, P: Into<Option<&'a ParserOptions>>>(options: P, str: &str) -> Option<ContentType> {
        let options = options.into();
        unsafe {
            from_glib_full(ffi::g_mime_content_type_parse(mut_override(options.to_glib_none().0), str.to_glib_none().0))
        }
    }
}

pub const NONE_CONTENT_TYPE: Option<&ContentType> = None;

pub trait ContentTypeExt: 'static {
    fn encode<'a, P: Into<Option<&'a FormatOptions>>>(&self, options: P) -> Option<GString>;

    fn get_media_subtype(&self) -> Option<GString>;

    fn get_media_type(&self) -> Option<GString>;

    fn get_mime_type(&self) -> Option<GString>;

    fn get_parameter(&self, name: &str) -> Option<GString>;

    fn get_parameters(&self) -> Option<ParamList>;

    fn is_type(&self, type_: &str, subtype: &str) -> bool;

    fn set_media_subtype(&self, subtype: &str);

    fn set_media_type(&self, type_: &str);

    fn set_parameter(&self, name: &str, value: &str);
}

impl<O: IsA<ContentType>> ContentTypeExt for O {
    fn encode<'a, P: Into<Option<&'a FormatOptions>>>(&self, options: P) -> Option<GString> {
        let options = options.into();
        unsafe {
            from_glib_full(ffi::g_mime_content_type_encode(self.as_ref().to_glib_none().0, mut_override(options.to_glib_none().0)))
        }
    }

    fn get_media_subtype(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::g_mime_content_type_get_media_subtype(self.as_ref().to_glib_none().0))
        }
    }

    fn get_media_type(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::g_mime_content_type_get_media_type(self.as_ref().to_glib_none().0))
        }
    }

    fn get_mime_type(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::g_mime_content_type_get_mime_type(self.as_ref().to_glib_none().0))
        }
    }

    fn get_parameter(&self, name: &str) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::g_mime_content_type_get_parameter(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_parameters(&self) -> Option<ParamList> {
        unsafe {
            from_glib_none(ffi::g_mime_content_type_get_parameters(self.as_ref().to_glib_none().0))
        }
    }

    fn is_type(&self, type_: &str, subtype: &str) -> bool {
        unsafe {
            from_glib(ffi::g_mime_content_type_is_type(self.as_ref().to_glib_none().0, type_.to_glib_none().0, subtype.to_glib_none().0))
        }
    }

    fn set_media_subtype(&self, subtype: &str) {
        unsafe {
            ffi::g_mime_content_type_set_media_subtype(self.as_ref().to_glib_none().0, subtype.to_glib_none().0);
        }
    }

    fn set_media_type(&self, type_: &str) {
        unsafe {
            ffi::g_mime_content_type_set_media_type(self.as_ref().to_glib_none().0, type_.to_glib_none().0);
        }
    }

    fn set_parameter(&self, name: &str, value: &str) {
        unsafe {
            ffi::g_mime_content_type_set_parameter(self.as_ref().to_glib_none().0, name.to_glib_none().0, value.to_glib_none().0);
        }
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ContentType")
    }
}
