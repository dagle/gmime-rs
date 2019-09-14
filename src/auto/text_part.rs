// This file was generated by gir (https://github.com/gtk-rs/gir @ 9e3cb65)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8+)
// DO NOT EDIT

use Object;
use Part;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use gmime_sys;
use std::fmt;

glib_wrapper! {
    pub struct TextPart(Object<gmime_sys::GMimeTextPart, gmime_sys::GMimeTextPartClass, TextPartClass>) @extends Part, Object;

    match fn {
        get_type => || gmime_sys::g_mime_text_part_get_type(),
    }
}

impl TextPart {
    pub fn new() -> TextPart {
        unsafe {
            from_glib_full(gmime_sys::g_mime_text_part_new())
        }
    }

    pub fn new_with_subtype(subtype: &str) -> TextPart {
        unsafe {
            from_glib_full(gmime_sys::g_mime_text_part_new_with_subtype(subtype.to_glib_none().0))
        }
    }
}

impl Default for TextPart {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_TEXT_PART: Option<&TextPart> = None;

pub trait TextPartExt: 'static {
    fn get_charset(&self) -> Option<GString>;

    fn get_text(&self) -> Option<GString>;

    fn set_charset(&self, charset: &str);

    fn set_text(&self, text: &str);
}

impl<O: IsA<TextPart>> TextPartExt for O {
    fn get_charset(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gmime_sys::g_mime_text_part_get_charset(self.as_ref().to_glib_none().0))
        }
    }

    fn get_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gmime_sys::g_mime_text_part_get_text(self.as_ref().to_glib_none().0))
        }
    }

    fn set_charset(&self, charset: &str) {
        unsafe {
            gmime_sys::g_mime_text_part_set_charset(self.as_ref().to_glib_none().0, charset.to_glib_none().0);
        }
    }

    fn set_text(&self, text: &str) {
        unsafe {
            gmime_sys::g_mime_text_part_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        }
    }
}

impl fmt::Display for TextPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextPart")
    }
}
