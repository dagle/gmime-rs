// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::{Object,Part};
use glib::{prelude::*,translate::*};
use std::{fmt};

glib::wrapper! {
    #[doc(alias = "GMimeTextPart")]
    pub struct TextPart(Object<ffi::GMimeTextPart, ffi::GMimeTextPartClass>) @extends Part, Object;

    match fn {
        type_ => || ffi::g_mime_text_part_get_type(),
    }
}

impl TextPart {
        pub const NONE: Option<&'static TextPart> = None;
    

    #[doc(alias = "g_mime_text_part_new")]
    pub fn new() -> TextPart {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::g_mime_text_part_new())
        }
    }

    #[doc(alias = "g_mime_text_part_new_with_subtype")]
    #[doc(alias = "new_with_subtype")]
    pub fn with_subtype(subtype: &str) -> TextPart {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::g_mime_text_part_new_with_subtype(subtype.to_glib_none().0))
        }
    }
}

impl Default for TextPart {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

pub trait TextPartExt: 'static {
    #[doc(alias = "g_mime_text_part_get_charset")]
    #[doc(alias = "get_charset")]
    fn charset(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mime_text_part_get_text")]
    #[doc(alias = "get_text")]
    fn text(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mime_text_part_set_charset")]
    fn set_charset(&self, charset: &str);

    #[doc(alias = "g_mime_text_part_set_text")]
    fn set_text(&self, text: &str);
}

impl<O: IsA<TextPart>> TextPartExt for O {
    fn charset(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_text_part_get_charset(self.as_ref().to_glib_none().0))
        }
    }

    fn text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_mime_text_part_get_text(self.as_ref().to_glib_none().0))
        }
    }

    fn set_charset(&self, charset: &str) {
        unsafe {
            ffi::g_mime_text_part_set_charset(self.as_ref().to_glib_none().0, charset.to_glib_none().0);
        }
    }

    fn set_text(&self, text: &str) {
        unsafe {
            ffi::g_mime_text_part_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        }
    }
}

impl fmt::Display for TextPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TextPart")
    }
}
