// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AppStreamType;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct AppSrc(Object<ffi::GstAppSrc, ffi::GstAppSrcClass>) @extends gst_base::BaseSrc, gst::Element, gst::Object, @implements gst::URIHandler;

    match fn {
        get_type => || ffi::gst_app_src_get_type(),
    }
}

impl AppSrc {
    pub fn get_caps(&self) -> Option<gst::Caps> {
        unsafe { from_glib_full(ffi::gst_app_src_get_caps(self.to_glib_none().0)) }
    }

    pub fn get_current_level_bytes(&self) -> u64 {
        unsafe { ffi::gst_app_src_get_current_level_bytes(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_10", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_10")))]
    pub fn get_duration(&self) -> gst::ClockTime {
        unsafe { from_glib(ffi::gst_app_src_get_duration(self.to_glib_none().0)) }
    }

    pub fn get_emit_signals(&self) -> bool {
        unsafe { from_glib(ffi::gst_app_src_get_emit_signals(self.to_glib_none().0)) }
    }

    pub fn get_max_bytes(&self) -> u64 {
        unsafe { ffi::gst_app_src_get_max_bytes(self.to_glib_none().0) }
    }

    pub fn get_size(&self) -> i64 {
        unsafe { ffi::gst_app_src_get_size(self.to_glib_none().0) }
    }

    pub fn get_stream_type(&self) -> AppStreamType {
        unsafe { from_glib(ffi::gst_app_src_get_stream_type(self.to_glib_none().0)) }
    }

    //pub fn set_callbacks(&self, callbacks: /*Ignored*/&mut AppSrcCallbacks, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:gst_app_src_set_callbacks() }
    //}

    pub fn set_caps(&self, caps: Option<&gst::Caps>) {
        unsafe {
            ffi::gst_app_src_set_caps(self.to_glib_none().0, caps.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_10", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_10")))]
    pub fn set_duration(&self, duration: gst::ClockTime) {
        unsafe {
            ffi::gst_app_src_set_duration(self.to_glib_none().0, duration.to_glib());
        }
    }

    pub fn set_emit_signals(&self, emit: bool) {
        unsafe {
            ffi::gst_app_src_set_emit_signals(self.to_glib_none().0, emit.to_glib());
        }
    }

    pub fn set_max_bytes(&self, max: u64) {
        unsafe {
            ffi::gst_app_src_set_max_bytes(self.to_glib_none().0, max);
        }
    }

    pub fn set_size(&self, size: i64) {
        unsafe {
            ffi::gst_app_src_set_size(self.to_glib_none().0, size);
        }
    }

    pub fn set_stream_type(&self, type_: AppStreamType) {
        unsafe {
            ffi::gst_app_src_set_stream_type(self.to_glib_none().0, type_.to_glib());
        }
    }

    pub fn get_property_block(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"block\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `block` getter")
                .unwrap()
        }
    }

    pub fn set_property_block(&self, block: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"block\0".as_ptr() as *const _,
                Value::from(&block).to_glib_none().0,
            );
        }
    }

    pub fn get_property_format(&self) -> gst::Format {
        unsafe {
            let mut value = Value::from_type(<gst::Format as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"format\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `format` getter")
                .unwrap()
        }
    }

    pub fn set_property_format(&self, format: gst::Format) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"format\0".as_ptr() as *const _,
                Value::from(&format).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
    pub fn get_property_handle_segment_change(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"handle-segment-change\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `handle-segment-change` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
    pub fn set_property_handle_segment_change(&self, handle_segment_change: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"handle-segment-change\0".as_ptr() as *const _,
                Value::from(&handle_segment_change).to_glib_none().0,
            );
        }
    }

    pub fn get_property_is_live(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"is-live\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `is-live` getter")
                .unwrap()
        }
    }

    pub fn set_property_is_live(&self, is_live: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"is-live\0".as_ptr() as *const _,
                Value::from(&is_live).to_glib_none().0,
            );
        }
    }

    pub fn get_property_max_latency(&self) -> i64 {
        unsafe {
            let mut value = Value::from_type(<i64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"max-latency\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-latency` getter")
                .unwrap()
        }
    }

    pub fn set_property_max_latency(&self, max_latency: i64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"max-latency\0".as_ptr() as *const _,
                Value::from(&max_latency).to_glib_none().0,
            );
        }
    }

    pub fn get_property_min_latency(&self) -> i64 {
        unsafe {
            let mut value = Value::from_type(<i64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"min-latency\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `min-latency` getter")
                .unwrap()
        }
    }

    pub fn set_property_min_latency(&self, min_latency: i64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"min-latency\0".as_ptr() as *const _,
                Value::from(&min_latency).to_glib_none().0,
            );
        }
    }

    pub fn get_property_min_percent(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"min-percent\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `min-percent` getter")
                .unwrap()
        }
    }

    pub fn set_property_min_percent(&self, min_percent: u32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"min-percent\0".as_ptr() as *const _,
                Value::from(&min_percent).to_glib_none().0,
            );
        }
    }

    pub fn connect_enough_data<F: Fn(&AppSrc) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn enough_data_trampoline<F: Fn(&AppSrc) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"enough-data\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    enough_data_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_need_data<F: Fn(&AppSrc, u32) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn need_data_trampoline<F: Fn(&AppSrc, u32) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            object: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"need-data\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    need_data_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_seek_data<F: Fn(&AppSrc, u64) -> bool + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn seek_data_trampoline<
            F: Fn(&AppSrc, u64) -> bool + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            object: u64,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"seek-data\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    seek_data_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_block_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_block_trampoline<F: Fn(&AppSrc) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::block\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_block_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_caps_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_caps_trampoline<F: Fn(&AppSrc) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::caps\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_caps_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_current_level_bytes_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_level_bytes_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::current-level-bytes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_current_level_bytes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_10", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_10")))]
    pub fn connect_property_duration_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_trampoline<F: Fn(&AppSrc) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_duration_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_emit_signals_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_emit_signals_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::emit-signals\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_emit_signals_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_format_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_format_trampoline<F: Fn(&AppSrc) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::format\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_format_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
    pub fn connect_property_handle_segment_change_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_handle_segment_change_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::handle-segment-change\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_handle_segment_change_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_is_live_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_live_trampoline<F: Fn(&AppSrc) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-live\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_live_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_max_bytes_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_bytes_trampoline<F: Fn(&AppSrc) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-bytes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_bytes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_max_latency_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_latency_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-latency\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_latency_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_min_latency_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_latency_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-latency\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_latency_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_min_percent_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_percent_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-percent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_percent_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_size_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_size_trampoline<F: Fn(&AppSrc) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_stream_type_notify<F: Fn(&AppSrc) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_stream_type_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stream-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_stream_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for AppSrc {}
unsafe impl Sync for AppSrc {}
