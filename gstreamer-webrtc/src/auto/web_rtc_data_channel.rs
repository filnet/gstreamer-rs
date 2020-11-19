// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::ObjectExt;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gst_web_rtc_sys;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;
use WebRTCDataChannelState;
use WebRTCPriorityType;

glib_wrapper! {
    pub struct WebRTCDataChannel(Object<gst_web_rtc_sys::GstWebRTCDataChannel, gst_web_rtc_sys::GstWebRTCDataChannelClass>);

    match fn {
        get_type => || gst_web_rtc_sys::gst_webrtc_data_channel_get_type(),
    }
}

impl WebRTCDataChannel {
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn close(&self) {
        unsafe {
            gst_web_rtc_sys::gst_webrtc_data_channel_close(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn on_buffered_amount_low(&self) {
        unsafe {
            gst_web_rtc_sys::gst_webrtc_data_channel_on_buffered_amount_low(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn on_close(&self) {
        unsafe {
            gst_web_rtc_sys::gst_webrtc_data_channel_on_close(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn on_message_data(&self, data: Option<&glib::Bytes>) {
        unsafe {
            gst_web_rtc_sys::gst_webrtc_data_channel_on_message_data(
                self.to_glib_none().0,
                data.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn on_message_string(&self, str: Option<&str>) {
        unsafe {
            gst_web_rtc_sys::gst_webrtc_data_channel_on_message_string(
                self.to_glib_none().0,
                str.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn on_open(&self) {
        unsafe {
            gst_web_rtc_sys::gst_webrtc_data_channel_on_open(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn send_data(&self, data: Option<&glib::Bytes>) {
        unsafe {
            gst_web_rtc_sys::gst_webrtc_data_channel_send_data(
                self.to_glib_none().0,
                data.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn send_string(&self, str: Option<&str>) {
        unsafe {
            gst_web_rtc_sys::gst_webrtc_data_channel_send_string(
                self.to_glib_none().0,
                str.to_glib_none().0,
            );
        }
    }

    pub fn get_property_buffered_amount(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"buffered-amount\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `buffered-amount` getter")
                .unwrap()
        }
    }

    pub fn get_property_buffered_amount_low_threshold(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"buffered-amount-low-threshold\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `buffered-amount-low-threshold` getter")
                .unwrap()
        }
    }

    pub fn set_property_buffered_amount_low_threshold(&self, buffered_amount_low_threshold: u64) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"buffered-amount-low-threshold\0".as_ptr() as *const _,
                Value::from(&buffered_amount_low_threshold).to_glib_none().0,
            );
        }
    }

    pub fn get_property_id(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `id` getter")
                .unwrap()
        }
    }

    pub fn get_property_label(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"label\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `label` getter")
        }
    }

    pub fn get_property_max_packet_lifetime(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"max-packet-lifetime\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-packet-lifetime` getter")
                .unwrap()
        }
    }

    pub fn get_property_max_retransmits(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"max-retransmits\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-retransmits` getter")
                .unwrap()
        }
    }

    pub fn get_property_negotiated(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"negotiated\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `negotiated` getter")
                .unwrap()
        }
    }

    pub fn get_property_ordered(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"ordered\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `ordered` getter")
                .unwrap()
        }
    }

    pub fn get_property_priority(&self) -> WebRTCPriorityType {
        unsafe {
            let mut value = Value::from_type(<WebRTCPriorityType as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"priority\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `priority` getter")
                .unwrap()
        }
    }

    pub fn get_property_protocol(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"protocol\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `protocol` getter")
        }
    }

    pub fn get_property_ready_state(&self) -> WebRTCDataChannelState {
        unsafe {
            let mut value = Value::from_type(<WebRTCDataChannelState as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"ready-state\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `ready-state` getter")
                .unwrap()
        }
    }

    pub fn connect_close<F: Fn(&WebRTCDataChannel) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn close_trampoline<F: Fn(&WebRTCDataChannel) + Send + Sync + 'static>(
            this: *mut gst_web_rtc_sys::GstWebRTCDataChannel,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"close\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    close_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_close(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("close", &[])
                .unwrap()
        };
    }

    pub fn connect_on_buffered_amount_low<F: Fn(&WebRTCDataChannel) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn on_buffered_amount_low_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut gst_web_rtc_sys::GstWebRTCDataChannel,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-buffered-amount-low\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    on_buffered_amount_low_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_on_close<F: Fn(&WebRTCDataChannel) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn on_close_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut gst_web_rtc_sys::GstWebRTCDataChannel,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-close\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    on_close_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_on_error<F: Fn(&WebRTCDataChannel, &glib::Error) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn on_error_trampoline<
            F: Fn(&WebRTCDataChannel, &glib::Error) + Send + Sync + 'static,
        >(
            this: *mut gst_web_rtc_sys::GstWebRTCDataChannel,
            error: *mut glib_sys::GError,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(error))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-error\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    on_error_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_on_message_data<
        F: Fn(&WebRTCDataChannel, Option<&glib::Bytes>) + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn on_message_data_trampoline<
            F: Fn(&WebRTCDataChannel, Option<&glib::Bytes>) + Send + Sync + 'static,
        >(
            this: *mut gst_web_rtc_sys::GstWebRTCDataChannel,
            data: *mut glib_sys::GBytes,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<glib::Bytes>::from_glib_borrow(data)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-message-data\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    on_message_data_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_on_message_string<
        F: Fn(&WebRTCDataChannel, Option<&str>) + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn on_message_string_trampoline<
            F: Fn(&WebRTCDataChannel, Option<&str>) + Send + Sync + 'static,
        >(
            this: *mut gst_web_rtc_sys::GstWebRTCDataChannel,
            data: *mut libc::c_char,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<GString>::from_glib_borrow(data)
                    .as_ref()
                    .as_deref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-message-string\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    on_message_string_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_on_open<F: Fn(&WebRTCDataChannel) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn on_open_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut gst_web_rtc_sys::GstWebRTCDataChannel,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-open\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    on_open_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_send_data<
        F: Fn(&WebRTCDataChannel, Option<&glib::Bytes>) + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn send_data_trampoline<
            F: Fn(&WebRTCDataChannel, Option<&glib::Bytes>) + Send + Sync + 'static,
        >(
            this: *mut gst_web_rtc_sys::GstWebRTCDataChannel,
            data: *mut glib_sys::GBytes,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<glib::Bytes>::from_glib_borrow(data)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"send-data\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    send_data_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_send_data(&self, data: Option<&glib::Bytes>) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("send-data", &[&data])
                .unwrap()
        };
    }

    pub fn connect_send_string<F: Fn(&WebRTCDataChannel, Option<&str>) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn send_string_trampoline<
            F: Fn(&WebRTCDataChannel, Option<&str>) + Send + Sync + 'static,
        >(
            this: *mut gst_web_rtc_sys::GstWebRTCDataChannel,
            data: *mut libc::c_char,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<GString>::from_glib_borrow(data)
                    .as_ref()
                    .as_deref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"send-string\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    send_string_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_send_string(&self, data: Option<&str>) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("send-string", &[&data])
                .unwrap()
        };
    }

    pub fn connect_property_buffered_amount_notify<
        F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffered_amount_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut gst_web_rtc_sys::GstWebRTCDataChannel,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::buffered-amount\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_buffered_amount_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_buffered_amount_low_threshold_notify<
        F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffered_amount_low_threshold_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut gst_web_rtc_sys::GstWebRTCDataChannel,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::buffered-amount-low-threshold\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_buffered_amount_low_threshold_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_ready_state_notify<
        F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ready_state_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut gst_web_rtc_sys::GstWebRTCDataChannel,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ready-state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ready_state_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for WebRTCDataChannel {}
unsafe impl Sync for WebRTCDataChannel {}
