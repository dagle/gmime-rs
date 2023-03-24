use glib::Cast;

use crate::{SeekWhence, Stream};
use glib::subclass::prelude::*;
use glib::translate::*;

unsafe impl Send for Stream {}
unsafe impl Sync for Stream {}

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

    fn write(&self, buf: &[u8]) -> isize {
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

    fn parent_write(&self, buf: &[u8]) -> isize;
}

impl<T: StreamImpl> StreamImplExt for T {
    fn parent_close(&self) -> i32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeStreamClass;
            let f = (*parent_class)
                .close
                .expect("No parent class impl for \"close\"");
            f(self.obj().unsafe_cast_ref::<Stream>().to_glib_none().0)
        }
    }

    fn parent_eos(&self) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeStreamClass;
            let f = (*parent_class)
                .eos
                .expect("No parent class impl for \"eos\"");
            from_glib(f(self.obj().unsafe_cast_ref::<Stream>().to_glib_none().0))
        }
    }

    fn parent_flush(&self) -> i32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeStreamClass;
            let f = (*parent_class)
                .flush
                .expect("No parent class impl for \"flush\"");
            f(self.obj().unsafe_cast_ref::<Stream>().to_glib_none().0)
        }
    }

    fn parent_length(&self) -> i64 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeStreamClass;
            let f = (*parent_class)
                .length
                .expect("No parent class impl for \"length\"");
            f(self.obj().unsafe_cast_ref::<Stream>().to_glib_none().0)
        }
    }

    fn parent_read(&self, buf: &[u8]) -> isize {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeStreamClass;
            let f = (*parent_class)
                .read
                .expect("No parent class impl for \"close\"");
            f(
                self.obj().unsafe_cast_ref::<Stream>().to_glib_none().0,
                buf.to_glib_none().0,
                buf.len() as usize,
            )
        }
    }

    fn parent_reset(&self) -> i32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeStreamClass;
            let f = (*parent_class)
                .reset
                .expect("No parent class impl for \"reset\"");
            f(self.obj().unsafe_cast_ref::<Stream>().to_glib_none().0)
        }
    }

    fn parent_seek(&self, offset: i64, whence: SeekWhence) -> i64 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeStreamClass;
            let f = (*parent_class)
                .seek
                .expect("No parent class impl for \"reset\"");
            f(
                self.obj().unsafe_cast_ref::<Stream>().to_glib_none().0,
                offset,
                whence.into_glib(),
            )
        }
    }

    fn parent_substream(&self, start: i64, end: i64) -> Option<Stream> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeStreamClass;
            let f = (*parent_class)
                .substream
                .expect("No parent class impl for \"reset\"");
            from_glib_full(f(
                self.obj().unsafe_cast_ref::<Stream>().to_glib_none().0,
                start,
                end,
            ))
        }
    }

    fn parent_tell(&self) -> i64 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeStreamClass;
            let f = (*parent_class)
                .tell
                .expect("No parent class impl for \"reset\"");
            f(self.obj().unsafe_cast_ref::<Stream>().to_glib_none().0)
        }
    }

    fn parent_write(&self, buf: &[u8]) -> isize {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeStreamClass;
            let f = (*parent_class)
                .write
                .expect("No parent class impl for \"reset\"");
            f(
                self.obj().unsafe_cast_ref::<Stream>().to_glib_none().0,
                buf.to_glib_none().0 as *const libc::c_char,
                buf.len() as usize,
            )
        }
    }
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
}

unsafe extern "C" fn read<T: StreamImpl>(
    ptr: *mut ffi::GMimeStream,
    buf: *mut u8,
    len: libc::size_t,
) -> libc::ssize_t {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.read(std::slice::from_raw_parts(buf, len as usize))
}

unsafe extern "C" fn write<T: StreamImpl>(
    ptr: *mut ffi::GMimeStream,
    buf: *const libc::c_char,
    len: libc::size_t,
) -> libc::ssize_t {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let slice = std::slice::from_raw_parts(buf as *const u8, len as usize);
    imp.write(slice)
}

unsafe extern "C" fn flush<T: StreamImpl>(ptr: *mut ffi::GMimeStream) -> libc::c_int {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.flush()
}

unsafe extern "C" fn close<T: StreamImpl>(ptr: *mut ffi::GMimeStream) -> libc::c_int {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.close()
}

unsafe extern "C" fn eos<T: StreamImpl>(ptr: *mut ffi::GMimeStream) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.eos().into_glib()
}

unsafe extern "C" fn reset<T: StreamImpl>(ptr: *mut ffi::GMimeStream) -> libc::c_int {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.reset()
}

unsafe extern "C" fn seek<T: StreamImpl>(
    ptr: *mut ffi::GMimeStream,
    offset: i64,
    whence: ffi::GMimeSeekWhence,
) -> i64 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.seek(offset, from_glib(whence))
}

unsafe extern "C" fn tell<T: StreamImpl>(ptr: *mut ffi::GMimeStream) -> i64 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.tell()
}

unsafe extern "C" fn length<T: StreamImpl>(ptr: *mut ffi::GMimeStream) -> i64 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.length()
}

unsafe extern "C" fn substream<T: StreamImpl>(
    ptr: *mut ffi::GMimeStream,
    start: i64,
    stop: i64,
) -> *mut ffi::GMimeStream {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.substream(start, stop).to_glib_full()
}
