// This file was generated by gir (https://github.com/gtk-rs/gir @ ad40c01)
// from gir-files (https://github.com/gtk-rs/gir-files @ 5b96546)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use gmime_sys;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Filter(Object<gmime_sys::GMimeFilter, gmime_sys::GMimeFilterClass, FilterClass>);

    match fn {
        get_type => || gmime_sys::g_mime_filter_get_type(),
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
            gmime_sys::g_mime_filter_backup(self.as_ref().to_glib_none().0, data.to_glib_none().0, length);
        }
    }

    fn complete(&self, inbuf: &[u8], prespace: usize) -> (Vec<u8>, usize) {
        let inlen = inbuf.len() as usize;
        unsafe {
            let mut outbuf = ptr::null_mut();
            let mut outlen = mem::MaybeUninit::uninit();
            let mut outprespace = mem::MaybeUninit::uninit();
            gmime_sys::g_mime_filter_complete(self.as_ref().to_glib_none().0, inbuf.to_glib_none().0, inlen, prespace, &mut outbuf, outlen.as_mut_ptr(), outprespace.as_mut_ptr());
            let outprespace = outprespace.assume_init();
            (FromGlibContainer::from_glib_none_num(outbuf, outlen.assume_init() as usize), outprespace)
        }
    }

    fn copy(&self) -> Option<Filter> {
        unsafe {
            from_glib_full(gmime_sys::g_mime_filter_copy(self.as_ref().to_glib_none().0))
        }
    }

    fn filter(&self, inbuf: &[u8], prespace: usize) -> (Vec<u8>, usize) {
        let inlen = inbuf.len() as usize;
        unsafe {
            let mut outbuf = ptr::null_mut();
            let mut outlen = mem::MaybeUninit::uninit();
            let mut outprespace = mem::MaybeUninit::uninit();
            gmime_sys::g_mime_filter_filter(self.as_ref().to_glib_none().0, inbuf.to_glib_none().0, inlen, prespace, &mut outbuf, outlen.as_mut_ptr(), outprespace.as_mut_ptr());
            let outprespace = outprespace.assume_init();
            (FromGlibContainer::from_glib_none_num(outbuf, outlen.assume_init() as usize), outprespace)
        }
    }

    fn reset(&self) {
        unsafe {
            gmime_sys::g_mime_filter_reset(self.as_ref().to_glib_none().0);
        }
    }

    fn set_size(&self, size: usize, keep: bool) {
        unsafe {
            gmime_sys::g_mime_filter_set_size(self.as_ref().to_glib_none().0, size, keep.to_glib());
        }
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Filter")
    }
}
