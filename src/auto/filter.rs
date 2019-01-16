// This file was generated by gir (https://github.com/gtk-rs/gir @ 2f0a317)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Filter(Object<ffi::GMimeFilter, ffi::GMimeFilterClass, FilterClass>);

    match fn {
        get_type => || ffi::g_mime_filter_get_type(),
    }
}

pub const NONE_FILTER: Option<&Filter> = None;

pub trait FilterExt: 'static {
    fn backup(&self, data: &[u8]);

    fn complete(&self, inbuf: &[u8], prespace: usize) -> (Vec<u8>, usize);

    fn copy(&self) -> Option<Filter>;

    fn filter(&self, inbuf: &[u8], prespace: usize) -> (Vec<u8>, usize);

    fn reset(&self);

    fn set_size(&self, size: usize, keep: bool);
}

impl<O: IsA<Filter>> FilterExt for O {
    fn backup(&self, data: &[u8]) {
        let length = data.len() as usize;
        unsafe {
            ffi::g_mime_filter_backup(self.as_ref().to_glib_none().0, data.to_glib_none().0, length);
        }
    }

    fn complete(&self, inbuf: &[u8], prespace: usize) -> (Vec<u8>, usize) {
        let inlen = inbuf.len() as usize;
        unsafe {
            let mut outbuf = ptr::null_mut();
            let mut outlen = mem::uninitialized();
            let mut outprespace = mem::uninitialized();
            ffi::g_mime_filter_complete(self.as_ref().to_glib_none().0, inbuf.to_glib_none().0, inlen, prespace, &mut outbuf, &mut outlen, &mut outprespace);
            (FromGlibContainer::from_glib_none_num(outbuf, outlen as usize), outprespace)
        }
    }

    fn copy(&self) -> Option<Filter> {
        unsafe {
            from_glib_full(ffi::g_mime_filter_copy(self.as_ref().to_glib_none().0))
        }
    }

    fn filter(&self, inbuf: &[u8], prespace: usize) -> (Vec<u8>, usize) {
        let inlen = inbuf.len() as usize;
        unsafe {
            let mut outbuf = ptr::null_mut();
            let mut outlen = mem::uninitialized();
            let mut outprespace = mem::uninitialized();
            ffi::g_mime_filter_filter(self.as_ref().to_glib_none().0, inbuf.to_glib_none().0, inlen, prespace, &mut outbuf, &mut outlen, &mut outprespace);
            (FromGlibContainer::from_glib_none_num(outbuf, outlen as usize), outprespace)
        }
    }

    fn reset(&self) {
        unsafe {
            ffi::g_mime_filter_reset(self.as_ref().to_glib_none().0);
        }
    }

    fn set_size(&self, size: usize, keep: bool) {
        unsafe {
            ffi::g_mime_filter_set_size(self.as_ref().to_glib_none().0, size, keep.to_glib());
        }
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Filter")
    }
}
