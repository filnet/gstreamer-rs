// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Asset;
use Error;
use UriSourceAsset;
use ffi;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_pbutils;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct UriClipAsset(Object<ffi::GESUriClipAsset, ffi::GESUriClipAssetClass, UriClipAssetClass>) @extends Asset;

    match fn {
        get_type => || ffi::ges_uri_clip_asset_get_type(),
    }
}

impl UriClipAsset {
    //pub fn new<'a, P: IsA<gio::Cancellable> + 'a, Q: Into<Option<&'a P>>, R: FnOnce(Result<(), Error>) + 'static>(uri: &str, cancellable: Q, callback: R) {
    //    unsafe { TODO: call ffi::ges_uri_clip_asset_new() }
    //}

    pub fn request_sync(uri: &str) -> Result<Option<UriClipAsset>, Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ges_uri_clip_asset_request_sync(uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

pub const NONE_URI_CLIP_ASSET: Option<&UriClipAsset> = None;

pub trait UriClipAssetExt: 'static {
    fn get_duration(&self) -> gst::ClockTime;

    fn get_info(&self) -> Option<gst_pbutils::DiscovererInfo>;

    fn get_stream_assets(&self) -> Vec<UriSourceAsset>;

    fn is_image(&self) -> bool;

    fn set_property_duration(&self, duration: u64);

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<UriClipAsset>> UriClipAssetExt for O {
    fn get_duration(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::ges_uri_clip_asset_get_duration(self.as_ref().to_glib_none().0))
        }
    }

    fn get_info(&self) -> Option<gst_pbutils::DiscovererInfo> {
        unsafe {
            from_glib_none(ffi::ges_uri_clip_asset_get_info(const_override(self.as_ref().to_glib_none().0)))
        }
    }

    fn get_stream_assets(&self) -> Vec<UriSourceAsset> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::ges_uri_clip_asset_get_stream_assets(self.as_ref().to_glib_none().0))
        }
    }

    fn is_image(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_uri_clip_asset_is_image(self.as_ref().to_glib_none().0))
        }
    }

    fn set_property_duration(&self, duration: u64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"duration\0".as_ptr() as *const _, Value::from(&duration).to_glib_none().0);
        }
    }

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::duration\0".as_ptr() as *const _,
                Some(transmute(notify_duration_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_duration_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GESUriClipAsset, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<UriClipAsset> {
    let f: &F = transmute(f);
    f(&UriClipAsset::from_glib_borrow(this).unsafe_cast())
}
