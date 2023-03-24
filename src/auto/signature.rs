// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::{Certificate, SignatureStatus};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GMimeSignature")]
    pub struct Signature(Object<ffi::GMimeSignature, ffi::GMimeSignatureClass>);

    match fn {
        type_ => || ffi::g_mime_signature_get_type(),
    }
}

impl Signature {
    pub const NONE: Option<&'static Signature> = None;

    #[doc(alias = "g_mime_signature_new")]
    pub fn new() -> Signature {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::g_mime_signature_new()) }
    }
}

impl Default for Signature {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SignatureExt: 'static {
    #[doc(alias = "g_mime_signature_get_certificate")]
    #[doc(alias = "get_certificate")]
    fn certificate(&self) -> Option<Certificate>;

    #[doc(alias = "g_mime_signature_get_created")]
    #[doc(alias = "get_created")]
    fn created(&self) -> libc::c_long;

    #[doc(alias = "g_mime_signature_get_created64")]
    #[doc(alias = "get_created64")]
    fn created64(&self) -> i64;

    #[doc(alias = "g_mime_signature_get_expires")]
    #[doc(alias = "get_expires")]
    fn expires(&self) -> libc::c_long;

    #[doc(alias = "g_mime_signature_get_expires64")]
    #[doc(alias = "get_expires64")]
    fn expires64(&self) -> i64;

    #[doc(alias = "g_mime_signature_get_status")]
    #[doc(alias = "get_status")]
    fn status(&self) -> SignatureStatus;

    #[doc(alias = "g_mime_signature_set_certificate")]
    fn set_certificate(&self, cert: &impl IsA<Certificate>);

    #[doc(alias = "g_mime_signature_set_created")]
    fn set_created(&self, created: libc::c_long);

    #[doc(alias = "g_mime_signature_set_expires")]
    fn set_expires(&self, expires: libc::c_long);

    #[doc(alias = "g_mime_signature_set_status")]
    fn set_status(&self, status: SignatureStatus);
}

impl<O: IsA<Signature>> SignatureExt for O {
    fn certificate(&self) -> Option<Certificate> {
        unsafe {
            from_glib_none(ffi::g_mime_signature_get_certificate(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn created(&self) -> libc::c_long {
        unsafe { ffi::g_mime_signature_get_created(self.as_ref().to_glib_none().0) }
    }

    fn created64(&self) -> i64 {
        unsafe { ffi::g_mime_signature_get_created64(self.as_ref().to_glib_none().0) }
    }

    fn expires(&self) -> libc::c_long {
        unsafe { ffi::g_mime_signature_get_expires(self.as_ref().to_glib_none().0) }
    }

    fn expires64(&self) -> i64 {
        unsafe { ffi::g_mime_signature_get_expires64(self.as_ref().to_glib_none().0) }
    }

    fn status(&self) -> SignatureStatus {
        unsafe {
            from_glib(ffi::g_mime_signature_get_status(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_certificate(&self, cert: &impl IsA<Certificate>) {
        unsafe {
            ffi::g_mime_signature_set_certificate(
                self.as_ref().to_glib_none().0,
                cert.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_created(&self, created: libc::c_long) {
        unsafe {
            ffi::g_mime_signature_set_created(self.as_ref().to_glib_none().0, created);
        }
    }

    fn set_expires(&self, expires: libc::c_long) {
        unsafe {
            ffi::g_mime_signature_set_expires(self.as_ref().to_glib_none().0, expires);
        }
    }

    fn set_status(&self, status: SignatureStatus) {
        unsafe {
            ffi::g_mime_signature_set_status(self.as_ref().to_glib_none().0, status.into_glib());
        }
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Signature")
    }
}
