// This file was generated by gir (https://github.com/gtk-rs/gir @ 2f0a317)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8)
// DO NOT EDIT

use AutocryptHeader;
use InternetAddressList;
use InternetAddressMailbox;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct AutocryptHeaderList(Object<ffi::GMimeAutocryptHeaderList, ffi::GMimeAutocryptHeaderListClass, AutocryptHeaderListClass>);

    match fn {
        get_type => || ffi::g_mime_autocrypt_header_list_get_type(),
    }
}

impl AutocryptHeaderList {
    pub fn new() -> AutocryptHeaderList {
        unsafe {
            from_glib_full(ffi::g_mime_autocrypt_header_list_new())
        }
    }
}

impl Default for AutocryptHeaderList {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_AUTOCRYPT_HEADER_LIST: Option<&AutocryptHeaderList> = None;

pub trait AutocryptHeaderListExt: 'static {
    fn add<P: IsA<AutocryptHeader>>(&self, header: &P);

    fn add_missing_addresses<P: IsA<InternetAddressList>>(&self, addresses: &P) -> u32;

    fn get_count(&self) -> u32;

    fn get_header_at(&self, index: u32) -> Option<AutocryptHeader>;

    fn get_header_for_address<P: IsA<InternetAddressMailbox>>(&self, mailbox: &P) -> Option<AutocryptHeader>;

    fn remove_incomplete(&self);
}

impl<O: IsA<AutocryptHeaderList>> AutocryptHeaderListExt for O {
    fn add<P: IsA<AutocryptHeader>>(&self, header: &P) {
        unsafe {
            ffi::g_mime_autocrypt_header_list_add(self.as_ref().to_glib_none().0, header.as_ref().to_glib_none().0);
        }
    }

    fn add_missing_addresses<P: IsA<InternetAddressList>>(&self, addresses: &P) -> u32 {
        unsafe {
            ffi::g_mime_autocrypt_header_list_add_missing_addresses(self.as_ref().to_glib_none().0, addresses.as_ref().to_glib_none().0)
        }
    }

    fn get_count(&self) -> u32 {
        unsafe {
            ffi::g_mime_autocrypt_header_list_get_count(self.as_ref().to_glib_none().0)
        }
    }

    fn get_header_at(&self, index: u32) -> Option<AutocryptHeader> {
        unsafe {
            from_glib_none(ffi::g_mime_autocrypt_header_list_get_header_at(self.as_ref().to_glib_none().0, index))
        }
    }

    fn get_header_for_address<P: IsA<InternetAddressMailbox>>(&self, mailbox: &P) -> Option<AutocryptHeader> {
        unsafe {
            from_glib_none(ffi::g_mime_autocrypt_header_list_get_header_for_address(self.as_ref().to_glib_none().0, mailbox.as_ref().to_glib_none().0))
        }
    }

    fn remove_incomplete(&self) {
        unsafe {
            ffi::g_mime_autocrypt_header_list_remove_incomplete(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for AutocryptHeaderList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AutocryptHeaderList")
    }
}
