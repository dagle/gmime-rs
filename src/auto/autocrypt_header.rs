// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8)
// DO NOT EDIT

use AutocryptPreferEncrypt;
use InternetAddressMailbox;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct AutocryptHeader(Object<ffi::GMimeAutocryptHeader, ffi::GMimeAutocryptHeaderClass>);

    match fn {
        get_type => || ffi::g_mime_autocrypt_header_get_type(),
    }
}

impl AutocryptHeader {
    pub fn new() -> AutocryptHeader {
        unsafe {
            from_glib_full(ffi::g_mime_autocrypt_header_new())
        }
    }

    pub fn new_from_string(string: &str) -> AutocryptHeader {
        unsafe {
            from_glib_full(ffi::g_mime_autocrypt_header_new_from_string(string.to_glib_none().0))
        }
    }
}

impl Default for AutocryptHeader {
    fn default() -> Self {
        Self::new()
    }
}

pub trait AutocryptHeaderExt {
    fn clone(&self, src: &AutocryptHeader);

    fn get_address(&self) -> Option<InternetAddressMailbox>;

    fn get_address_as_string(&self) -> Option<String>;

    fn get_effective_date(&self) -> Option<glib::DateTime>;

    fn get_keydata(&self) -> Option<glib::Bytes>;

    fn get_prefer_encrypt(&self) -> AutocryptPreferEncrypt;

    fn is_complete(&self) -> bool;

    fn set_address(&self, address: &InternetAddressMailbox);

    fn set_address_from_string(&self, address: &str);

    fn set_effective_date(&self, effective_date: &glib::DateTime);

    fn set_keydata(&self, data: &glib::Bytes);

    fn set_prefer_encrypt(&self, pref: AutocryptPreferEncrypt);

    fn to_string(&self, gossip: bool) -> String;
}

impl<O: IsA<AutocryptHeader>> AutocryptHeaderExt for O {
    fn clone(&self, src: &AutocryptHeader) {
        unsafe {
            ffi::g_mime_autocrypt_header_clone(self.to_glib_none().0, src.to_glib_none().0);
        }
    }

    fn get_address(&self) -> Option<InternetAddressMailbox> {
        unsafe {
            from_glib_none(ffi::g_mime_autocrypt_header_get_address(self.to_glib_none().0))
        }
    }

    fn get_address_as_string(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mime_autocrypt_header_get_address_as_string(self.to_glib_none().0))
        }
    }

    fn get_effective_date(&self) -> Option<glib::DateTime> {
        unsafe {
            from_glib_none(ffi::g_mime_autocrypt_header_get_effective_date(self.to_glib_none().0))
        }
    }

    fn get_keydata(&self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_none(ffi::g_mime_autocrypt_header_get_keydata(self.to_glib_none().0))
        }
    }

    fn get_prefer_encrypt(&self) -> AutocryptPreferEncrypt {
        unsafe {
            from_glib(ffi::g_mime_autocrypt_header_get_prefer_encrypt(self.to_glib_none().0))
        }
    }

    fn is_complete(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_autocrypt_header_is_complete(self.to_glib_none().0))
        }
    }

    fn set_address(&self, address: &InternetAddressMailbox) {
        unsafe {
            ffi::g_mime_autocrypt_header_set_address(self.to_glib_none().0, address.to_glib_none().0);
        }
    }

    fn set_address_from_string(&self, address: &str) {
        unsafe {
            ffi::g_mime_autocrypt_header_set_address_from_string(self.to_glib_none().0, address.to_glib_none().0);
        }
    }

    fn set_effective_date(&self, effective_date: &glib::DateTime) {
        unsafe {
            ffi::g_mime_autocrypt_header_set_effective_date(self.to_glib_none().0, effective_date.to_glib_none().0);
        }
    }

    fn set_keydata(&self, data: &glib::Bytes) {
        unsafe {
            ffi::g_mime_autocrypt_header_set_keydata(self.to_glib_none().0, data.to_glib_none().0);
        }
    }

    fn set_prefer_encrypt(&self, pref: AutocryptPreferEncrypt) {
        unsafe {
            ffi::g_mime_autocrypt_header_set_prefer_encrypt(self.to_glib_none().0, pref.to_glib());
        }
    }

    fn to_string(&self, gossip: bool) -> String {
        unsafe {
            from_glib_full(ffi::g_mime_autocrypt_header_to_string(self.to_glib_none().0, gossip.to_glib()))
        }
    }
}
