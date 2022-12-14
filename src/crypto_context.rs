use crate::CryptoContext;
use crate::Stream;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;

pub trait CryptoContextExtManual: 'static {
    #[doc(alias = "g_mime_crypto_context_export_keys")]
    fn export_keys(&self, keys: &[&str], ostream: &impl IsA<Stream>) -> Result<i32, glib::Error>;
}

impl<O: IsA<CryptoContext>> CryptoContextExtManual for O {
    fn export_keys(&self, keys: &[&str], ostream: &impl IsA<Stream>) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mime_crypto_context_export_keys(
                self.as_ref().to_glib_none().0,
                ToGlibContainerFromSlice::to_glib_none_from_slice(keys).0,
                ostream.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
