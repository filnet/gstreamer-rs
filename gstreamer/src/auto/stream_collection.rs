// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Object;
#[cfg(any(feature = "v1_10", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_10")))]
use crate::Stream;
use glib::translate::*;

glib::glib_wrapper! {
    pub struct StreamCollection(Object<ffi::GstStreamCollection, ffi::GstStreamCollectionClass>) @extends Object;

    match fn {
        get_type => || ffi::gst_stream_collection_get_type(),
    }
}

impl StreamCollection {
    #[cfg(any(feature = "v1_10", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_10")))]
    pub fn get_size(&self) -> u32 {
        unsafe { ffi::gst_stream_collection_get_size(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_10", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_10")))]
    pub fn get_stream(&self, index: u32) -> Option<Stream> {
        unsafe {
            from_glib_none(ffi::gst_stream_collection_get_stream(
                self.to_glib_none().0,
                index,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_10")))]
    pub fn get_upstream_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_stream_collection_get_upstream_id(
                self.to_glib_none().0,
            ))
        }
    }

    //pub fn connect_stream_notify<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored p0: GObject.ParamSpec
    //}
}

unsafe impl Send for StreamCollection {}
unsafe impl Sync for StreamCollection {}
