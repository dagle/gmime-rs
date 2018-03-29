// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff+)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8)
// DO NOT EDIT

use InternetAddressList;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct AutocryptHeaderList(Object<ffi::GMimeAutocryptHeaderList, ffi::GMimeAutocryptHeaderListClass>);

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

pub trait AutocryptHeaderListExt {
    //fn add(&self, header: /*Ignored*/&AutocryptHeader);

    fn add_missing_addresses(&self, addresses: &InternetAddressList) -> u32;

    fn get_count(&self) -> u32;

    //fn get_header_at(&self, index: u32) -> /*Ignored*/Option<AutocryptHeader>;

    //fn get_header_for_address(&self, mailbox: &InternetAddressMailbox) -> /*Ignored*/Option<AutocryptHeader>;

    fn remove_incomplete(&self);
}

impl<O: IsA<AutocryptHeaderList>> AutocryptHeaderListExt for O {
    //fn add(&self, header: /*Ignored*/&AutocryptHeader) {
    //    unsafe { TODO: call ffi::g_mime_autocrypt_header_list_add() }
    //}

    fn add_missing_addresses(&self, addresses: &InternetAddressList) -> u32 {
        unsafe {
            ffi::g_mime_autocrypt_header_list_add_missing_addresses(self.to_glib_none().0, addresses.to_glib_none().0)
        }
    }

    fn get_count(&self) -> u32 {
        unsafe {
            ffi::g_mime_autocrypt_header_list_get_count(self.to_glib_none().0)
        }
    }

    //fn get_header_at(&self, index: u32) -> /*Ignored*/Option<AutocryptHeader> {
    //    unsafe { TODO: call ffi::g_mime_autocrypt_header_list_get_header_at() }
    //}

    //fn get_header_for_address(&self, mailbox: &InternetAddressMailbox) -> /*Ignored*/Option<AutocryptHeader> {
    //    unsafe { TODO: call ffi::g_mime_autocrypt_header_list_get_header_for_address() }
    //}

    fn remove_incomplete(&self) {
        unsafe {
            ffi::g_mime_autocrypt_header_list_remove_incomplete(self.to_glib_none().0);
        }
    }
}