// This file was generated by gir (https://github.com/gtk-rs/gir @ 2f0a317)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8)
// DO NOT EDIT

use Error;
use Object;
use Part;
use SecureMimeType;
use SignatureList;
use VerifyFlags;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct ApplicationPkcs7Mime(Object<ffi::GMimeApplicationPkcs7Mime, ffi::GMimeApplicationPkcs7MimeClass, ApplicationPkcs7MimeClass>) @extends Part, Object;

    match fn {
        get_type => || ffi::g_mime_application_pkcs7_mime_get_type(),
    }
}

impl ApplicationPkcs7Mime {
    pub fn new(type_: SecureMimeType) -> ApplicationPkcs7Mime {
        unsafe {
            from_glib_full(ffi::g_mime_application_pkcs7_mime_new(type_.to_glib()))
        }
    }

    //pub fn encrypt<P: IsA<Object>>(entity: &P, flags: EncryptFlags, recipients: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 0, id: 28 }) -> Result<Option<ApplicationPkcs7Mime>, Error> {
    //    unsafe { TODO: call ffi::g_mime_application_pkcs7_mime_encrypt() }
    //}

    pub fn sign<P: IsA<Object>>(entity: &P, userid: &str) -> Result<Option<ApplicationPkcs7Mime>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_application_pkcs7_mime_sign(entity.as_ref().to_glib_none().0, userid.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

pub const NONE_APPLICATION_PKCS7_MIME: Option<&ApplicationPkcs7Mime> = None;

pub trait ApplicationPkcs7MimeExt: 'static {
    fn get_smime_type(&self) -> SecureMimeType;

    fn verify(&self, flags: VerifyFlags) -> Result<(Option<SignatureList>, Object), Error>;
}

impl<O: IsA<ApplicationPkcs7Mime>> ApplicationPkcs7MimeExt for O {
    fn get_smime_type(&self) -> SecureMimeType {
        unsafe {
            from_glib(ffi::g_mime_application_pkcs7_mime_get_smime_type(self.as_ref().to_glib_none().0))
        }
    }

    fn verify(&self, flags: VerifyFlags) -> Result<(Option<SignatureList>, Object), Error> {
        unsafe {
            let mut entity = ptr::null_mut();
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_application_pkcs7_mime_verify(self.as_ref().to_glib_none().0, flags.to_glib(), &mut entity, &mut error);
            if error.is_null() { Ok((from_glib_full(ret), from_glib_full(entity))) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for ApplicationPkcs7Mime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApplicationPkcs7Mime")
    }
}
