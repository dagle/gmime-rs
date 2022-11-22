use std::ffi::CStr;

use crate::{Stream, SeekWhence};
use glib::translate::*;
use glib::subclass::prelude::*;

pub trait StreamImpl: StreamImplExt + ObjectImpl {
    fn close(&self) -> i32 {
        self.parent_close()
	}

    fn eos(&self) -> bool {
        self.parent_eos()
	}

    fn flush(&self) -> i32 {
        self.parent_flush()
	}

    fn length(&self) -> i64 {
        self.parent_length()
	}

    fn read(&self, buf: &[u8]) -> isize {
        self.parent_read(buf)
	}

    fn reset(&self) -> i32 {
        self.parent_reset()
	}

    fn seek(&self, offset: i64, whence: SeekWhence) -> i64 {
        self.parent_seek(offset, whence)
	}

    fn substream(&self, start: i64, end: i64) -> Option<Stream> {
        self.parent_substream(start, end)
	}

    fn tell(&self) -> i64 {
        self.parent_tell()
	}

    fn write(&self, buf: &str) -> isize {
        self.parent_write(buf)
	}
}

pub trait StreamImplExt: ObjectSubclass {
    fn parent_close(&self) -> i32;

    fn parent_eos(&self) -> bool;

    fn parent_flush(&self) -> i32;

    fn parent_length(&self) -> i64;

    fn parent_read(&self, buf: &[u8]) -> isize;

    fn parent_reset(&self) -> i32;

    fn parent_seek(&self, offset: i64, whence: SeekWhence) -> i64;

    fn parent_substream(&self, start: i64, end: i64) -> Option<Stream>;

    fn parent_tell(&self) -> i64;

    fn parent_write(&self, buf: &str) -> isize;
}

unsafe impl<T: StreamImpl> IsSubclassable<T> for Stream {
    fn class_init(class: &mut glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.read = Some(read::<T>);
        klass.write = Some(write::<T>);
        klass.flush = Some(flush::<T>);
        klass.close = Some(close::<T>);
        klass.eos = Some(eos::<T>);
        klass.reset = Some(reset::<T>);
        klass.seek = Some(seek::<T>);
        klass.tell = Some(tell::<T>);
        klass.length = Some(length::<T>);
        klass.substream = Some(substream::<T>);
    }
    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <glib::Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn read <T: StreamImpl>(ptr: *mut ffi::GMimeStream, buf: *mut u8, len: libc::size_t) -> libc::ssize_t {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.read(std::slice::from_raw_parts(buf, len as usize))
}

    // pub write: Option<unsafe extern "C" fn(*mut GMimeStream, *const c_char, size_t) -> ssize_t>,
unsafe extern "C" fn write <T: StreamImpl>(ptr: *mut ffi::GMimeStream, buf: *const libc::c_char, len: libc::size_t) -> libc::ssize_t {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let c_str = CStr::from_ptr(buf);
    
    match c_str.to_str() {
        Ok(s) => imp.write(s),
        Err(_) => 0
    }
}

unsafe extern "C" fn flush <T: StreamImpl>(ptr: *mut ffi::GMimeStream) -> libc::c_int {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.flush()
}

unsafe extern "C" fn close <T: StreamImpl>(ptr: *mut ffi::GMimeStream) -> libc::c_int {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.close()
}

unsafe extern "C" fn eos <T: StreamImpl>(ptr: *mut ffi::GMimeStream) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.eos().into_glib()
}

unsafe extern "C" fn reset <T: StreamImpl>(ptr: *mut ffi::GMimeStream) -> libc::c_int {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.reset()
}

unsafe extern "C" fn seek <T: StreamImpl>(ptr: *mut ffi::GMimeStream, offset: i64, whence: ffi::GMimeSeekWhence) -> i64 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.seek(offset, from_glib(whence))
}

unsafe extern "C" fn tell <T: StreamImpl>(ptr: *mut ffi::GMimeStream) -> i64 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.tell()
}

unsafe extern "C" fn length <T: StreamImpl>(ptr: *mut ffi::GMimeStream) -> i64 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.length()
}

unsafe extern "C" fn substream <T: StreamImpl>(ptr: *mut ffi::GMimeStream, start: i64, stop: i64) -> *mut ffi::GMimeStream {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.substream(start, stop).to_glib_full()
}
