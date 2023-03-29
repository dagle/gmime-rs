// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::{AutocryptPreferEncrypt,InternetAddressMailbox};
use glib::{prelude::*,translate::*};
use std::{fmt};

glib::wrapper! {
    #[doc(alias = "GMimeAutocryptHeader")]
    pub struct AutocryptHeader(Object<ffi::GMimeAutocryptHeader, ffi::GMimeAutocryptHeaderClass>);

    match fn {
        type_ => || ffi::g_mime_autocrypt_header_get_type(),
    }
}

impl AutocryptHeader {
        pub const NONE: Option<&'static AutocryptHeader> = None;
    

    #[doc(alias = "g_mime_autocrypt_header_new")]
    pub fn new() -> AutocryptHeader {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::g_mime_autocrypt_header_new())
        }
    }

    #[doc(alias = "g_mime_autocrypt_header_new_from_string")]
    #[doc(alias = "new_from_string")]
    pub fn from_string(string: &str) -> AutocryptHeader {
        assert_initialized_main_thread!();
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

pub trait AutocryptHeaderExt: 'static {
    #[doc(alias = "g_mime_autocrypt_header_clone")]
    fn clone(&self, src: &impl IsA<AutocryptHeader>);

    #[doc(alias = "g_mime_autocrypt_header_get_address")]
    #[doc(alias = "get_address")]
    fn address(&self) -> Option<InternetAddressMailbox>;

    #[doc(alias = "g_mime_autocrypt_header_get_address_as_string")]
    #[doc(alias = "get_address_as_string")]
    fn address_as_string(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mime_autocrypt_header_get_effective_date")]
    #[doc(alias = "get_effective_date")]
    fn effective_date(&self) -> Option<glib::DateTime>;

    #[doc(alias = "g_mime_autocrypt_header_get_keydata")]
    #[doc(alias = "get_keydata")]
    fn keydata(&self) -> Option<glib::Bytes>;

    #[doc(alias = "g_mime_autocrypt_header_get_prefer_encrypt")]
    #[doc(alias = "get_prefer_encrypt")]
    fn prefer_encrypt(&self) -> AutocryptPreferEncrypt;

    #[doc(alias = "g_mime_autocrypt_header_is_complete")]
    fn is_complete(&self) -> bool;

    #[doc(alias = "g_mime_autocrypt_header_set_address")]
    fn set_address(&self, address: &impl IsA<InternetAddressMailbox>);

    #[doc(alias = "g_mime_autocrypt_header_set_address_from_string")]
    fn set_address_from_string(&self, address: &str);

    #[doc(alias = "g_mime_autocrypt_header_set_effective_date")]
    fn set_effective_date(&self, effective_date: &glib::DateTime);

    #[doc(alias = "g_mime_autocrypt_header_set_keydata")]
    fn set_keydata(&self, keydata: &glib::Bytes);

    #[doc(alias = "g_mime_autocrypt_header_set_prefer_encrypt")]
    fn set_prefer_encrypt(&self, pref: AutocryptPreferEncrypt);

    #[doc(alias = "g_mime_autocrypt_header_to_string")]
    fn to_string(&self, gossip: bool) -> Option<glib::GString>;
}

impl<O: IsA<AutocryptHeader>> AutocryptHeaderExt for O {
    fn clone(&self, src: &impl IsA<AutocryptHeader>) {
        unsafe {
            ffi::g_mime_autocrypt_header_clone(self.as_ref().to_glib_none().0, src.as_ref().to_glib_none().0);
        }
    }

    fn address(&self) -> Option<InternetAddressMailbox> {
        unsafe {
            from_glib_none(ffi::g_mime_autocrypt_header_get_address(self.as_ref().to_glib_none().0))
        }
    }

    fn address_as_string(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_autocrypt_header_get_address_as_string(self.as_ref().to_glib_none().0))
        }
    }

    fn effective_date(&self) -> Option<glib::DateTime> {
        unsafe {
            from_glib_none(ffi::g_mime_autocrypt_header_get_effective_date(self.as_ref().to_glib_none().0))
        }
    }

    fn keydata(&self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_none(ffi::g_mime_autocrypt_header_get_keydata(self.as_ref().to_glib_none().0))
        }
    }

    fn prefer_encrypt(&self) -> AutocryptPreferEncrypt {
        unsafe {
            from_glib(ffi::g_mime_autocrypt_header_get_prefer_encrypt(self.as_ref().to_glib_none().0))
        }
    }

    fn is_complete(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mime_autocrypt_header_is_complete(self.as_ref().to_glib_none().0))
        }
    }

    fn set_address(&self, address: &impl IsA<InternetAddressMailbox>) {
        unsafe {
            ffi::g_mime_autocrypt_header_set_address(self.as_ref().to_glib_none().0, address.as_ref().to_glib_none().0);
        }
    }

    fn set_address_from_string(&self, address: &str) {
        unsafe {
            ffi::g_mime_autocrypt_header_set_address_from_string(self.as_ref().to_glib_none().0, address.to_glib_none().0);
        }
    }

    fn set_effective_date(&self, effective_date: &glib::DateTime) {
        unsafe {
            ffi::g_mime_autocrypt_header_set_effective_date(self.as_ref().to_glib_none().0, effective_date.to_glib_none().0);
        }
    }

    fn set_keydata(&self, keydata: &glib::Bytes) {
        unsafe {
            ffi::g_mime_autocrypt_header_set_keydata(self.as_ref().to_glib_none().0, keydata.to_glib_none().0);
        }
    }

    fn set_prefer_encrypt(&self, pref: AutocryptPreferEncrypt) {
        unsafe {
            ffi::g_mime_autocrypt_header_set_prefer_encrypt(self.as_ref().to_glib_none().0, pref.into_glib());
        }
    }

    fn to_string(&self, gossip: bool) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_mime_autocrypt_header_to_string(self.as_ref().to_glib_none().0, gossip.into_glib()))
        }
    }
}

impl fmt::Display for AutocryptHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AutocryptHeader")
    }
}
