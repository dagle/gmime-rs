// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::{DecryptFlags,DecryptResult,DigestAlgo,EncryptFlags,SignatureList,Stream,VerifyFlags};
use glib::{prelude::*,translate::*};
use std::{fmt,ptr};

glib::wrapper! {
    #[doc(alias = "GMimeCryptoContext")]
    pub struct CryptoContext(Object<ffi::GMimeCryptoContext, ffi::GMimeCryptoContextClass>);

    match fn {
        type_ => || ffi::g_mime_crypto_context_get_type(),
    }
}

impl CryptoContext {
        pub const NONE: Option<&'static CryptoContext> = None;
    

    #[doc(alias = "g_mime_crypto_context_new")]
    pub fn new(protocol: &str) -> Option<CryptoContext> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::g_mime_crypto_context_new(protocol.to_glib_none().0))
        }
    }

    //#[doc(alias = "g_mime_crypto_context_register")]
    //pub fn register<P: Fn() -> CryptoContext + 'static>(protocol: &str, callback: P) {
    //    unsafe { TODO: call ffi:g_mime_crypto_context_register() }
    //}
}

pub trait CryptoContextExt: 'static {
    #[doc(alias = "g_mime_crypto_context_decrypt")]
    fn decrypt(&self, flags: DecryptFlags, session_key: Option<&str>, istream: &impl IsA<Stream>, ostream: &impl IsA<Stream>) -> Result<DecryptResult, glib::Error>;

    #[doc(alias = "g_mime_crypto_context_digest_id")]
    fn digest_id(&self, name: &str) -> DigestAlgo;

    #[doc(alias = "g_mime_crypto_context_digest_name")]
    fn digest_name(&self, digest: DigestAlgo) -> Option<glib::GString>;

    #[doc(alias = "g_mime_crypto_context_encrypt")]
    fn encrypt(&self, sign: bool, userid: Option<&str>, flags: EncryptFlags, recipients: &[&str], istream: &impl IsA<Stream>, ostream: &impl IsA<Stream>) -> Result<i32, glib::Error>;

    #[doc(alias = "g_mime_crypto_context_get_encryption_protocol")]
    #[doc(alias = "get_encryption_protocol")]
    fn encryption_protocol(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mime_crypto_context_get_key_exchange_protocol")]
    #[doc(alias = "get_key_exchange_protocol")]
    fn key_exchange_protocol(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mime_crypto_context_get_signature_protocol")]
    #[doc(alias = "get_signature_protocol")]
    fn signature_protocol(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mime_crypto_context_import_keys")]
    fn import_keys(&self, istream: &impl IsA<Stream>) -> Result<i32, glib::Error>;

    //#[doc(alias = "g_mime_crypto_context_set_request_password")]
    //fn set_request_password<P: Fn(&CryptoContext, &str, &str, bool, Option<&glib::Error>) -> bool + 'static>(&self, request_passwd: P);

    #[doc(alias = "g_mime_crypto_context_sign")]
    fn sign(&self, detach: bool, userid: &str, istream: &impl IsA<Stream>, ostream: &impl IsA<Stream>) -> Result<i32, glib::Error>;

    #[doc(alias = "g_mime_crypto_context_verify")]
    fn verify(&self, flags: VerifyFlags, istream: &impl IsA<Stream>, sigstream: Option<&impl IsA<Stream>>, ostream: Option<&impl IsA<Stream>>) -> Result<Option<SignatureList>, glib::Error>;
}

impl<O: IsA<CryptoContext>> CryptoContextExt for O {
    fn decrypt(&self, flags: DecryptFlags, session_key: Option<&str>, istream: &impl IsA<Stream>, ostream: &impl IsA<Stream>) -> Result<DecryptResult, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_crypto_context_decrypt(self.as_ref().to_glib_none().0, flags.into_glib(), session_key.to_glib_none().0, istream.as_ref().to_glib_none().0, ostream.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn digest_id(&self, name: &str) -> DigestAlgo {
        unsafe {
            from_glib(ffi::g_mime_crypto_context_digest_id(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn digest_name(&self, digest: DigestAlgo) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_crypto_context_digest_name(self.as_ref().to_glib_none().0, digest.into_glib()))
        }
    }

    fn encrypt(&self, sign: bool, userid: Option<&str>, flags: EncryptFlags, recipients: &[&str], istream: &impl IsA<Stream>, ostream: &impl IsA<Stream>) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_crypto_context_encrypt(self.as_ref().to_glib_none().0, sign.into_glib(), userid.to_glib_none().0, flags.into_glib(), recipients.to_glib_none().0, istream.as_ref().to_glib_none().0, ostream.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn encryption_protocol(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_crypto_context_get_encryption_protocol(self.as_ref().to_glib_none().0))
        }
    }

    fn key_exchange_protocol(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_crypto_context_get_key_exchange_protocol(self.as_ref().to_glib_none().0))
        }
    }

    fn signature_protocol(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_mime_crypto_context_get_signature_protocol(self.as_ref().to_glib_none().0))
        }
    }

    fn import_keys(&self, istream: &impl IsA<Stream>) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_crypto_context_import_keys(self.as_ref().to_glib_none().0, istream.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    //fn set_request_password<P: Fn(&CryptoContext, &str, &str, bool, Option<&glib::Error>) -> bool + 'static>(&self, request_passwd: P) {
    //    unsafe { TODO: call ffi:g_mime_crypto_context_set_request_password() }
    //}

    fn sign(&self, detach: bool, userid: &str, istream: &impl IsA<Stream>, ostream: &impl IsA<Stream>) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_crypto_context_sign(self.as_ref().to_glib_none().0, detach.into_glib(), userid.to_glib_none().0, istream.as_ref().to_glib_none().0, ostream.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn verify(&self, flags: VerifyFlags, istream: &impl IsA<Stream>, sigstream: Option<&impl IsA<Stream>>, ostream: Option<&impl IsA<Stream>>) -> Result<Option<SignatureList>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_crypto_context_verify(self.as_ref().to_glib_none().0, flags.into_glib(), istream.as_ref().to_glib_none().0, sigstream.map(|p| p.as_ref()).to_glib_none().0, ostream.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for CryptoContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CryptoContext")
    }
}
