// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DiscovererInfo;
use Error;
use ffi;
use glib;
use glib::object::ObjectType;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gst;
use gst_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Discoverer(Object<ffi::GstDiscoverer, ffi::GstDiscovererClass, DiscovererClass>);

    match fn {
        get_type => || ffi::gst_discoverer_get_type(),
    }
}

impl Discoverer {
    pub fn new(timeout: gst::ClockTime) -> Result<Discoverer, Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_discoverer_new(timeout.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn discover_uri(&self, uri: &str) -> Result<DiscovererInfo, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_discoverer_discover_uri(self.to_glib_none().0, uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn discover_uri_async(&self, uri: &str) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ffi::gst_discoverer_discover_uri_async(self.to_glib_none().0, uri.to_glib_none().0), "Failed to add URI to list of discovers")
        }
    }

    pub fn start(&self) {
        unsafe {
            ffi::gst_discoverer_start(self.to_glib_none().0);
        }
    }

    pub fn stop(&self) {
        unsafe {
            ffi::gst_discoverer_stop(self.to_glib_none().0);
        }
    }

    pub fn connect_discovered<F: Fn(&Discoverer, &DiscovererInfo, &Option<Error>) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"discovered\0".as_ptr() as *const _,
                Some(transmute(discovered_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_finished<F: Fn(&Discoverer) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"finished\0".as_ptr() as *const _,
                Some(transmute(finished_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_source_setup<F: Fn(&Discoverer, &gst::Element) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"source-setup\0".as_ptr() as *const _,
                Some(transmute(source_setup_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_starting<F: Fn(&Discoverer) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"starting\0".as_ptr() as *const _,
                Some(transmute(starting_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe impl Send for Discoverer {}
unsafe impl Sync for Discoverer {}

unsafe extern "C" fn discovered_trampoline<F: Fn(&Discoverer, &DiscovererInfo, &Option<Error>) + Send + Sync + 'static>(this: *mut ffi::GstDiscoverer, info: *mut ffi::GstDiscovererInfo, error: *mut glib_ffi::GError, f: glib_ffi::gpointer) {
    let f: &F = transmute(f);
    f(&from_glib_borrow(this), &from_glib_borrow(info), &from_glib_borrow(error))
}

unsafe extern "C" fn finished_trampoline<F: Fn(&Discoverer) + Send + Sync + 'static>(this: *mut ffi::GstDiscoverer, f: glib_ffi::gpointer) {
    let f: &F = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn source_setup_trampoline<F: Fn(&Discoverer, &gst::Element) + Send + Sync + 'static>(this: *mut ffi::GstDiscoverer, source: *mut gst_ffi::GstElement, f: glib_ffi::gpointer) {
    let f: &F = transmute(f);
    f(&from_glib_borrow(this), &from_glib_borrow(source))
}

unsafe extern "C" fn starting_trampoline<F: Fn(&Discoverer) + Send + Sync + 'static>(this: *mut ffi::GstDiscoverer, f: glib_ffi::gpointer) {
    let f: &F = transmute(f);
    f(&from_glib_borrow(this))
}
