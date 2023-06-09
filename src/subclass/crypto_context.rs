use std::ffi::CStr;
use std::ptr;

use glib::Cast;

use crate::CryptoContext;
use crate::DecryptFlags;
use crate::DecryptResult;
use crate::DigestAlgo;
use crate::EncryptFlags;
use crate::SignatureList;
use crate::Stream;
use crate::VerifyFlags;
use glib::subclass::prelude::*;
use glib::translate::*;
extern crate libc;

macro_rules! maybe_str {
    ($acc:expr) => {
        if $acc.is_null() {
            None
        } else {
            let c_str = CStr::from_ptr($acc);
            let str = c_str.to_str().unwrap();
            Some(str)
        }
    };
}

pub trait CryptoContextImpl: CryptoContextExt + ObjectImpl {
    fn decrypt(
        &self,
        flags: DecryptFlags,
        session_key: Option<&str>,
        istream: &Stream,
        ostream: &Stream,
    ) -> Result<DecryptResult, glib::Error> {
        self.parent_decrypt(flags, session_key, istream, ostream)
    }
    fn digest_id(&self, name: &str) -> DigestAlgo {
        self.parent_digest_id(name)
    }
    fn digest_name(&self, digest: DigestAlgo) -> Option<String> {
        self.parent_digest_name(digest)
    }
    fn encrypt(
        &self,
        sign: bool,
        userid: Option<&str>,
        flags: EncryptFlags,
        recipients: &[&str],
        istream: &Stream,
        ostream: &Stream,
    ) -> Result<i32, glib::Error> {
        self.parent_encrypt(sign, userid, flags, recipients, istream, ostream)
    }

    fn encryption_protocol(&self) -> Option<String> {
        self.parent_encryption_protocol()
    }
    fn key_exchange_protocol(&self) -> Option<String> {
        self.parent_key_exchange_protocol()
    }
    fn signature_protocol(&self) -> Option<String> {
        self.parent_signature_protocol()
    }
    fn import_keys(&self, istream: &Stream) -> Result<i32, glib::Error> {
        self.parent_import_keys(istream)
    }
    fn export_keys(&self, keys: &[&str], ostream: &Stream) -> Result<i32, glib::Error> {
        self.parent_export_keys(keys, ostream)
    }
    fn verify(
        &self,
        flags: VerifyFlags,
        istream: &Stream,
        sigstream: Option<&Stream>,
        ostream: Option<&Stream>,
    ) -> Result<Option<SignatureList>, glib::Error> {
        self.parent_verify(flags, istream, sigstream, ostream)
    }

    fn sign(
        &self,
        detach: bool,
        userid: &str,
        istream: &Stream,
        ostream: &Stream,
    ) -> Result<i32, glib::Error> {
        self.parent_sign(detach, userid, istream, ostream)
    }
}

pub trait CryptoContextExt: ObjectSubclass {
    fn parent_decrypt(
        &self,
        flags: DecryptFlags,
        session_key: Option<&str>,
        istream: &Stream,
        ostream: &Stream,
    ) -> Result<DecryptResult, glib::Error>;

    fn parent_digest_id(&self, name: &str) -> DigestAlgo;
    fn parent_digest_name(&self, digest: DigestAlgo) -> Option<String>;
    fn parent_encrypt(
        &self,
        sign: bool,
        userid: Option<&str>,
        flags: EncryptFlags,
        recipients: &[&str],
        istream: &Stream,
        ostream: &Stream,
    ) -> Result<i32, glib::Error>;

    fn parent_encryption_protocol(&self) -> Option<String>;

    fn parent_key_exchange_protocol(&self) -> Option<String>;

    fn parent_signature_protocol(&self) -> Option<String>;

    fn parent_import_keys(&self, istream: &Stream) -> Result<i32, glib::Error>;

    fn parent_export_keys(&self, keys: &[&str], ostream: &Stream) -> Result<i32, glib::Error>;

    fn parent_sign(
        &self,
        detach: bool,
        userid: &str,
        istream: &Stream,
        ostream: &Stream,
    ) -> Result<i32, glib::Error>;

    fn parent_verify(
        &self,
        flags: VerifyFlags,
        istream: &Stream,
        sigstream: Option<&Stream>,
        ostream: Option<&Stream>,
    ) -> Result<Option<SignatureList>, glib::Error>;
}

impl<T: CryptoContextImpl> CryptoContextExt for T {
    fn parent_decrypt(
        &self,
        flags: DecryptFlags,
        session_key: Option<&str>,
        istream: &Stream,
        ostream: &Stream,
    ) -> Result<DecryptResult, glib::Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeCryptoContextClass;
            let f = (*parent_class)
                .decrypt
                .expect("No parent class impl for \"decrypt\"");
            let mut error = std::ptr::null_mut();
            let ret = f(
                self.obj()
                    .unsafe_cast_ref::<CryptoContext>()
                    .to_glib_none()
                    .0,
                flags.into_glib(),
                session_key.to_glib_none().0,
                istream.to_glib_none().0,
                ostream.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_digest_id(&self, name: &str) -> DigestAlgo {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeCryptoContextClass;
            let f = (*parent_class)
                .digest_id
                .expect("No parent class impl for \"digest_id\"");
            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<CryptoContext>()
                    .to_glib_none()
                    .0,
                name.to_glib_none().0,
            ))
        }
    }

    fn parent_digest_name(&self, digest: DigestAlgo) -> Option<String> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeCryptoContextClass;
            let f = (*parent_class)
                .digest_name
                .expect("No parent class impl for \"digest_name\"");
            from_glib_none(f(
                self.obj()
                    .unsafe_cast_ref::<CryptoContext>()
                    .to_glib_none()
                    .0,
                digest.into_glib(),
            ))
        }
    }

    fn parent_encrypt(
        &self,
        sign: bool,
        userid: Option<&str>,
        flags: EncryptFlags,
        recipients: &[&str],
        istream: &Stream,
        ostream: &Stream,
    ) -> Result<i32, glib::Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeCryptoContextClass;
            let f = (*parent_class)
                .encrypt
                .expect("No parent class impl for \"encrypt\"");
            let mut error = std::ptr::null_mut();
            let ret = f(
                self.obj()
                    .unsafe_cast_ref::<CryptoContext>()
                    .to_glib_none()
                    .0,
                sign.into_glib(),
                userid.to_glib_none().0,
                flags.into_glib(),
                recipients.to_glib_none().0,
                istream.to_glib_none().0,
                ostream.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_encryption_protocol(&self) -> Option<String> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeCryptoContextClass;
            let f = (*parent_class)
                .get_encryption_protocol
                .expect("No parent class impl for \"encrypt\"");
            from_glib_none(f(self
                .obj()
                .unsafe_cast_ref::<CryptoContext>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_key_exchange_protocol(&self) -> Option<String> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeCryptoContextClass;
            let f = (*parent_class)
                .get_key_exchange_protocol
                .expect("No parent class impl for \"encrypt\"");
            from_glib_none(f(self
                .obj()
                .unsafe_cast_ref::<CryptoContext>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_signature_protocol(&self) -> Option<String> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeCryptoContextClass;
            let f = (*parent_class)
                .get_signature_protocol
                .expect("No parent class impl for \"encrypt\"");
            from_glib_none(f(self
                .obj()
                .unsafe_cast_ref::<CryptoContext>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_import_keys(&self, istream: &Stream) -> Result<i32, glib::Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeCryptoContextClass;
            let f = (*parent_class)
                .import_keys
                .expect("No parent class impl for \"import_keys\"");
            let mut error = std::ptr::null_mut();
            let ret = f(
                self.obj()
                    .unsafe_cast_ref::<CryptoContext>()
                    .to_glib_none()
                    .0,
                istream.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_export_keys(&self, keys: &[&str], ostream: &Stream) -> Result<i32, glib::Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeCryptoContextClass;
            let f = (*parent_class)
                .export_keys
                .expect("No parent class impl for \"export_keys\"");
            let mut error = std::ptr::null_mut();

            let ret = f(
                self.obj()
                    .unsafe_cast_ref::<CryptoContext>()
                    .to_glib_none()
                    .0,
                ToGlibContainerFromSlice::to_glib_none_from_slice(keys).0,
                ostream.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_sign(
        &self,
        detach: bool,
        userid: &str,
        istream: &Stream,
        ostream: &Stream,
    ) -> Result<i32, glib::Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeCryptoContextClass;
            let f = (*parent_class)
                .sign
                .expect("No parent class impl for \"sign\"");
            let mut error = std::ptr::null_mut();
            let ret = f(
                self.obj()
                    .unsafe_cast_ref::<CryptoContext>()
                    .to_glib_none()
                    .0,
                detach.into_glib(),
                userid.to_glib_none().0,
                istream.to_glib_none().0,
                ostream.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_verify(
        &self,
        flags: VerifyFlags,
        istream: &Stream,
        sigstream: Option<&Stream>,
        ostream: Option<&Stream>,
    ) -> Result<Option<SignatureList>, glib::Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeCryptoContextClass;
            let f = (*parent_class)
                .verify
                .expect("No parent class impl for \"verify\"");
            let mut error = std::ptr::null_mut();
            let ret = f(
                self.obj()
                    .unsafe_cast_ref::<CryptoContext>()
                    .to_glib_none()
                    .0,
                flags.into_glib(),
                istream.to_glib_none().0,
                sigstream.to_glib_none().0,
                ostream.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

unsafe impl<T: CryptoContextImpl> IsSubclassable<T> for CryptoContext {
    fn class_init(class: &mut glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.decrypt = Some(decrypt::<T>);
        klass.digest_id = Some(digest_id::<T>);
        klass.digest_name = Some(digest_name::<T>);
        klass.encrypt = Some(encrypt::<T>);
        klass.get_encryption_protocol = Some(get_encryption_protocol::<T>);
        klass.get_key_exchange_protocol = Some(get_key_exchange_protocol::<T>);
        klass.get_signature_protocol = Some(get_signature_protocol::<T>);
        klass.import_keys = Some(import_keys::<T>);
        klass.export_keys = Some(export_keys::<T>);
        klass.sign = Some(sign::<T>);
        klass.verify = Some(verify::<T>);
    }
}

unsafe extern "C" fn decrypt<T: CryptoContextImpl>(
    ptr: *mut ffi::GMimeCryptoContext,
    flags: ffi::GMimeDecryptFlags,
    key: *const libc::c_char,
    istream: *mut ffi::GMimeStream,
    ostream: *mut ffi::GMimeStream,
    error: *mut *mut glib::ffi::GError,
) -> *mut ffi::GMimeDecryptResult {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let key = maybe_str!(key);
    let instream: Borrowed<Stream> = from_glib_borrow(istream);
    let outstream: Borrowed<Stream> = from_glib_borrow(ostream);

    let result = imp.decrypt(
        from_glib(flags),
        key,
        &*instream,
        &*outstream,
    );

    match result {
        Ok(num) => {
            *error = ptr::null_mut();
            num.to_glib_full()
        }
        Err(e) => {
            if !error.is_null() {
                *error = e.into_glib_ptr();
            }
            std::ptr::null_mut()
        }
    }
}

unsafe extern "C" fn digest_id<T: CryptoContextImpl>(
    ptr: *mut ffi::GMimeCryptoContext,
    name: *const libc::c_char,
) -> ffi::GMimeDigestAlgo {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let c_str = CStr::from_ptr(name);
    let str = c_str.to_str().unwrap();
    return imp.digest_id(str).into_glib()
}

unsafe extern "C" fn digest_name<T: CryptoContextImpl>(
    ptr: *mut ffi::GMimeCryptoContext,
    digest: ffi::GMimeDigestAlgo,
) -> *const libc::c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.digest_name(from_glib(digest)).to_glib_none().0
}

unsafe extern "C" fn get_signature_protocol<T: CryptoContextImpl>(
    ptr: *mut ffi::GMimeCryptoContext,
) -> *const libc::c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.signature_protocol().to_glib_none().0
}

unsafe extern "C" fn get_encryption_protocol<T: CryptoContextImpl>(
    ptr: *mut ffi::GMimeCryptoContext,
) -> *const libc::c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.signature_protocol().to_glib_none().0
}

unsafe extern "C" fn get_key_exchange_protocol<T: CryptoContextImpl>(
    ptr: *mut ffi::GMimeCryptoContext,
) -> *const libc::c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.signature_protocol().to_glib_none().0
}

unsafe extern "C" fn verify<T: CryptoContextImpl>(
    ptr: *mut ffi::GMimeCryptoContext,
    flags: ffi::GMimeVerifyFlags,
    istream: *mut ffi::GMimeStream,
    sigstream: *mut ffi::GMimeStream,
    ostream: *mut ffi::GMimeStream,
    error: *mut *mut glib::ffi::GError,
) -> *mut ffi::GMimeSignatureList {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let instream: Borrowed<Stream> = from_glib_borrow(istream);
    let outstream: Borrowed<Option<Stream>> = from_glib_borrow(ostream);
    let sstream: Borrowed<Option<Stream>> = from_glib_borrow(sigstream);

    let result = imp.verify(
        from_glib(flags),
        &*instream,
        sstream.as_ref().as_ref(),
        outstream.as_ref().as_ref(),
    );

    match result {
        Ok(list) => {
            if !error.is_null() {
                *error = ptr::null_mut();
            }
            list.to_glib_full()
        }
        Err(e) => {
            if !error.is_null() {
                *error = e.into_glib_ptr();
            }
            std::ptr::null_mut()
        }
    }
}

unsafe extern "C" fn encrypt<T: CryptoContextImpl>(
    ptr: *mut ffi::GMimeCryptoContext,
    sign: glib::ffi::gboolean,
    uid: *const libc::c_char,
    flags: ffi::GMimeEncryptFlags,
    recipients: *mut glib::ffi::GPtrArray,
    istream: *mut ffi::GMimeStream,
    ostream: *mut ffi::GMimeStream,
    error: *mut *mut glib::ffi::GError,
) -> libc::c_int {

    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let uid = maybe_str!(uid);

    let instream: Borrowed<Stream> = from_glib_borrow(istream);
    let outstream: Borrowed<Stream> = from_glib_borrow(ostream);

    let num = (*recipients).len as usize;
    let mut recip: Vec<&str> = Vec::with_capacity(num);
    let pdata = (*recipients).pdata;
    for n in 0..num {
        let item_ptr = pdata.add(n);
        let c_str = CStr::from_ptr(*item_ptr as *const libc::c_char);
        recip.push(c_str.to_str().unwrap());
    }

    let result = imp.encrypt(
        from_glib(sign),
        uid,
        from_glib(flags),
        &*recip,
        &*instream,
        &*outstream,
    );

    match result {
        Ok(num) => {
            if !error.is_null() {
                *error = ptr::null_mut();
            }
            num
        }
        Err(e) => {
            if !error.is_null() {
                *error = e.into_glib_ptr();
            }
            -1
        }
    }
}

unsafe extern "C" fn import_keys<T: CryptoContextImpl>(
    ptr: *mut ffi::GMimeCryptoContext,
    istream: *mut ffi::GMimeStream,
    error: *mut *mut glib::ffi::GError,
) -> libc::c_int {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let instream: Borrowed<Stream> = from_glib_borrow(istream);
    match imp.import_keys(&*instream) {
        Ok(num) => {
            if !error.is_null() {
                *error = ptr::null_mut();
            }
            num
        }
        Err(e) => {
            if !error.is_null() {
                *error = e.into_glib_ptr();
            }
            -1
        }
    }
}

unsafe extern "C" fn export_keys<T: CryptoContextImpl>(
    ptr: *mut ffi::GMimeCryptoContext,
    keys: *mut *const libc::c_char,
    ostream: *mut ffi::GMimeStream,
    error: *mut *mut glib::ffi::GError,
) -> libc::c_int {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let outstream: Borrowed<Stream> = from_glib_borrow(ostream);

    let mut k: Vec<&str> = Vec::new();
    if !keys.is_null() {
        let mut len = 0;
        loop {
            let item = *keys.add(len);
            if item.is_null() {
                break;
            }
            let c_str = CStr::from_ptr(item);
            k.push(c_str.to_str().unwrap());
            len += 1;
        }
    }

    match imp.export_keys(&k, &*outstream) {
        Ok(num) => {
            if !error.is_null() {
                *error = ptr::null_mut();
            }
            num
        }
        Err(e) => {
            if !error.is_null() {
                *error = e.into_glib_ptr();
            }
            -1
        }
    }
}

unsafe extern "C" fn sign<T: CryptoContextImpl>(
    ptr: *mut ffi::GMimeCryptoContext,
    detach: glib::ffi::gboolean,
    uid: *const libc::c_char,
    istream: *mut ffi::GMimeStream,
    ostream: *mut ffi::GMimeStream,
    error: *mut *mut glib::ffi::GError,
) -> libc::c_int {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let uid = CStr::from_ptr(uid);
    let uid = uid.to_str().unwrap();
    let instream: Borrowed<Stream> = from_glib_borrow(istream);
    let outstream: Borrowed<Stream> = from_glib_borrow(ostream);
    let result = imp.sign(from_glib(detach), uid, &*instream, &*outstream);

    match result {
        Ok(num) => {
            if !error.is_null() {
                *error = ptr::null_mut();
            }
            num
        }
        Err(e) => {
            if !error.is_null() {
                *error = e.into_glib_ptr();
            }
            -1
        }
    }
}
