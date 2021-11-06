// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Clock;
use crate::ClockType;
use crate::Object;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstSystemClock")]
    pub struct SystemClock(Object<ffi::GstSystemClock, ffi::GstSystemClockClass>) @extends Clock, Object;

    match fn {
        type_ => || ffi::gst_system_clock_get_type(),
    }
}

impl SystemClock {
    #[doc(alias = "gst_system_clock_obtain")]
    pub fn obtain() -> Clock {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_system_clock_obtain()) }
    }

    #[doc(alias = "gst_system_clock_set_default")]
    pub fn set_default(new_clock: Option<&impl IsA<Clock>>) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_system_clock_set_default(new_clock.map(|p| p.as_ref()).to_glib_none().0);
        }
    }
}

unsafe impl Send for SystemClock {}
unsafe impl Sync for SystemClock {}

impl SystemClock {
    pub const NONE: Option<&'static SystemClock> = None;
}

pub trait SystemClockExt: 'static {
    #[doc(alias = "clock-type")]
    fn clock_type(&self) -> ClockType;

    #[doc(alias = "clock-type")]
    fn set_clock_type(&self, clock_type: ClockType);

    #[doc(alias = "clock-type")]
    fn connect_clock_type_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<SystemClock>> SystemClockExt for O {
    fn clock_type(&self) -> ClockType {
        glib::ObjectExt::property(self.as_ref(), "clock-type")
    }

    fn set_clock_type(&self, clock_type: ClockType) {
        glib::ObjectExt::set_property(self.as_ref(), "clock-type", &clock_type)
    }

    fn connect_clock_type_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_clock_type_trampoline<
            P: IsA<SystemClock>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstSystemClock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SystemClock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::clock-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_clock_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
