use crate::Filter;
use glib::translate::*;
use glib::subclass::prelude::*;

pub trait FilterImpl: FilterImplExt + ObjectImpl {
    fn complete(&self, inbuf: &[u8], prespace: usize) -> (Vec<u8>, usize) {
        self.parent_filter(inbuf, prespace)
    }

    fn copy(&self) -> Option<Filter> {
        self.parent_copy()
    }

    fn filter(&self, inbuf: &[u8], prespace: usize) -> (Vec<u8>, usize) {
        self.parent_filter(inbuf, prespace)
    }

    fn reset(&self) {
        self.parent_reset()
    }
}

pub trait FilterImplExt: ObjectSubclass {
    fn parent_complete(&self, inbuf: &[u8], prespace: usize) -> (Vec<u8>, usize);

    fn parent_copy(&self) -> Option<Filter>;

    fn parent_filter(&self, inbuf: &[u8], prespace: usize) -> (Vec<u8>, usize);

    fn parent_reset(&self);
}

// impl<T: FilterImpl> FilterImplExt for T {
//     fn parent_complete(&self, inbuf: &[u8], prespace: usize) -> (Vec<u8>, usize) {
//         todo!()
//     }
//
//     fn parent_copy(&self) -> Option<Filter> {
//         todo!()
//     }
//
//     fn parent_filter(&self, inbuf: &[u8], prespace: usize) -> (Vec<u8>, usize) {
//         todo!()
//     }
//
//     fn parent_reset(&self) {
//         unsafe {
//             let data = T::type_data();
//             let parent_class = data.as_ref().parent_class() as *mut ffi::GMimeFilterClass;
//             if let Some(f) = (*parent_class).reset {
//                 f(
//                     self.obj().unsafe_cast_ref::<Filter>().to_glib_none().0,
//                 )
//             }
//         }
//     }
// }

unsafe impl<T: FilterImpl> IsSubclassable<T> for Filter {
    fn class_init(class: &mut glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.complete = Some(complete::<T>);
        klass.copy = Some(copy::<T>);
        klass.filter = Some(filter::<T>);
        klass.reset = Some(reset::<T>);
    }
}

unsafe extern "C" fn complete<T: FilterImpl>(ptr: *mut ffi::GMimeFilter, inbuf: *mut u8, inlen: libc::size_t, prespace: libc::size_t, outbuf: *mut *mut u8, outlen: *mut libc::size_t, outprespace: *mut libc::size_t) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let ibuf = std::slice::from_raw_parts(inbuf, inlen as usize);
    let (out, outsize) = imp.complete(ibuf, prespace);
    *outbuf = out.to_glib_full();
    *outlen = outsize;
}

unsafe extern "C" fn copy<T: FilterImpl>(ptr: *mut ffi::GMimeFilter) -> *mut ffi::GMimeFilter {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.copy().to_glib_full()
}

unsafe extern "C" fn filter<T: FilterImpl>(ptr: *mut ffi::GMimeFilter, inbuf: *mut u8, inlen: libc::size_t, prespace: libc::size_t, outbuf: *mut *mut u8, outlen: *mut libc::size_t, outprespace: *mut libc::size_t) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let ibuf = std::slice::from_raw_parts(inbuf, inlen as usize);
    let (out, outsize) = imp.filter(ibuf, prespace);
    *outbuf = out.to_glib_full();
    *outlen = outsize;
}

unsafe extern "C" fn reset<T: FilterImpl>(ptr: *mut ffi::GMimeFilter) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.reset()
}
