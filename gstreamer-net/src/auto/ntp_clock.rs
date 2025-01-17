// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::NetClientClock;
use glib::object::Cast;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstNtpClock")]
    pub struct NtpClock(Object<ffi::GstNtpClock, ffi::GstNtpClockClass>) @extends NetClientClock, gst::Clock, gst::Object;

    match fn {
        type_ => || ffi::gst_ntp_clock_get_type(),
    }
}

impl NtpClock {
    #[doc(alias = "gst_ntp_clock_new")]
    pub fn new(
        name: &str,
        remote_address: &str,
        remote_port: i32,
        base_time: impl Into<Option<gst::ClockTime>>,
    ) -> NtpClock {
        assert_initialized_main_thread!();
        unsafe {
            gst::Clock::from_glib_full(ffi::gst_ntp_clock_new(
                name.to_glib_none().0,
                remote_address.to_glib_none().0,
                remote_port,
                base_time.into().into_glib(),
            ))
            .unsafe_cast()
        }
    }
}

unsafe impl Send for NtpClock {}
unsafe impl Sync for NtpClock {}
