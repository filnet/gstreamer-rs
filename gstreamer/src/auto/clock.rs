// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ClockTime;
use crate::Object;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstClock")]
    pub struct Clock(Object<ffi::GstClock, ffi::GstClockClass>) @extends Object;

    match fn {
        type_ => || ffi::gst_clock_get_type(),
    }
}

impl Clock {
    //#[doc(alias = "gst_clock_id_compare_func")]
    //pub fn id_compare_func(id1: /*Unimplemented*/Option<Fundamental: Pointer>, id2: /*Unimplemented*/Option<Fundamental: Pointer>) -> i32 {
    //    unsafe { TODO: call ffi:gst_clock_id_compare_func() }
    //}

    //#[cfg(any(feature = "v1_16", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    //#[doc(alias = "gst_clock_id_get_clock")]
    //pub fn id_get_clock(id: /*Unimplemented*/ClockID) -> Option<Clock> {
    //    unsafe { TODO: call ffi:gst_clock_id_get_clock() }
    //}

    //#[doc(alias = "gst_clock_id_get_time")]
    //pub fn id_get_time(id: /*Unimplemented*/ClockID) -> Option<ClockTime> {
    //    unsafe { TODO: call ffi:gst_clock_id_get_time() }
    //}

    //#[doc(alias = "gst_clock_id_ref")]
    //pub fn id_ref(id: /*Unimplemented*/ClockID) -> /*Unimplemented*/ClockID {
    //    unsafe { TODO: call ffi:gst_clock_id_ref() }
    //}

    //#[doc(alias = "gst_clock_id_unref")]
    //pub fn id_unref(id: /*Unimplemented*/ClockID) {
    //    unsafe { TODO: call ffi:gst_clock_id_unref() }
    //}

    //#[doc(alias = "gst_clock_id_unschedule")]
    //pub fn id_unschedule(id: /*Unimplemented*/ClockID) {
    //    unsafe { TODO: call ffi:gst_clock_id_unschedule() }
    //}

    //#[cfg(any(feature = "v1_16", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    //#[doc(alias = "gst_clock_id_uses_clock")]
    //pub fn id_uses_clock(id: /*Unimplemented*/ClockID, clock: &impl IsA<Clock>) -> bool {
    //    unsafe { TODO: call ffi:gst_clock_id_uses_clock() }
    //}

    //#[doc(alias = "gst_clock_id_wait")]
    //pub fn id_wait(id: /*Unimplemented*/ClockID) -> (Result<ClockSuccess, ClockError>, ClockTimeDiff) {
    //    unsafe { TODO: call ffi:gst_clock_id_wait() }
    //}

    //#[doc(alias = "gst_clock_id_wait_async")]
    //pub fn id_wait_async(id: /*Unimplemented*/ClockID, func: /*Unimplemented*/Fn(&Clock, impl Into<Option<ClockTime>>, /*Unimplemented*/ClockID) -> bool, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> Result<ClockSuccess, ClockError> {
    //    unsafe { TODO: call ffi:gst_clock_id_wait_async() }
    //}
}

unsafe impl Send for Clock {}
unsafe impl Sync for Clock {}

pub const NONE_CLOCK: Option<&Clock> = None;

pub trait ClockExt: 'static {
    #[doc(alias = "gst_clock_add_observation")]
    fn add_observation(&self, slave: ClockTime, master: ClockTime) -> Option<f64>;

    #[doc(alias = "gst_clock_add_observation_unapplied")]
    fn add_observation_unapplied(
        &self,
        slave: ClockTime,
        master: ClockTime,
    ) -> Option<(f64, ClockTime, ClockTime, ClockTime, ClockTime)>;

    #[doc(alias = "gst_clock_adjust_unlocked")]
    fn adjust_unlocked(&self, internal: ClockTime) -> Option<ClockTime>;

    #[doc(alias = "gst_clock_get_calibration")]
    #[doc(alias = "get_calibration")]
    fn calibration(&self) -> (ClockTime, ClockTime, ClockTime, ClockTime);

    #[doc(alias = "gst_clock_get_internal_time")]
    #[doc(alias = "get_internal_time")]
    fn internal_time(&self) -> ClockTime;

    #[doc(alias = "gst_clock_get_master")]
    #[doc(alias = "get_master")]
    fn master(&self) -> Option<Clock>;

    #[doc(alias = "gst_clock_get_resolution")]
    #[doc(alias = "get_resolution")]
    fn resolution(&self) -> ClockTime;

    #[doc(alias = "gst_clock_get_time")]
    #[doc(alias = "get_time")]
    fn time(&self) -> Option<ClockTime>;

    #[doc(alias = "gst_clock_get_timeout")]
    #[doc(alias = "get_timeout")]
    fn timeout(&self) -> Option<ClockTime>;

    #[doc(alias = "gst_clock_is_synced")]
    fn is_synced(&self) -> bool;

    #[doc(alias = "gst_clock_set_calibration")]
    fn set_calibration(
        &self,
        internal: ClockTime,
        external: ClockTime,
        rate_num: ClockTime,
        rate_denom: ClockTime,
    );

    #[doc(alias = "gst_clock_set_master")]
    fn set_master(&self, master: Option<&impl IsA<Clock>>) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_clock_set_resolution")]
    fn set_resolution(&self, resolution: ClockTime) -> ClockTime;

    #[doc(alias = "gst_clock_set_synced")]
    fn set_synced(&self, synced: bool);

    #[doc(alias = "gst_clock_set_timeout")]
    fn set_timeout(&self, timeout: impl Into<Option<ClockTime>>);

    #[doc(alias = "gst_clock_unadjust_unlocked")]
    fn unadjust_unlocked(&self, external: ClockTime) -> Option<ClockTime>;

    #[doc(alias = "gst_clock_wait_for_sync")]
    fn wait_for_sync(
        &self,
        timeout: impl Into<Option<ClockTime>>,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "window-size")]
    fn window_size(&self) -> i32;

    #[doc(alias = "window-size")]
    fn set_window_size(&self, window_size: i32);

    #[doc(alias = "window-threshold")]
    fn window_threshold(&self) -> i32;

    #[doc(alias = "window-threshold")]
    fn set_window_threshold(&self, window_threshold: i32);

    #[doc(alias = "synced")]
    fn connect_synced<F: Fn(&Self, bool) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "timeout")]
    fn connect_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F)
        -> SignalHandlerId;

    #[doc(alias = "window-size")]
    fn connect_window_size_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "window-threshold")]
    fn connect_window_threshold_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Clock>> ClockExt for O {
    fn add_observation(&self, slave: ClockTime, master: ClockTime) -> Option<f64> {
        unsafe {
            let mut r_squared = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_clock_add_observation(
                self.as_ref().to_glib_none().0,
                slave.into_glib(),
                master.into_glib(),
                r_squared.as_mut_ptr(),
            ));
            let r_squared = r_squared.assume_init();
            if ret {
                Some(r_squared)
            } else {
                None
            }
        }
    }

    fn add_observation_unapplied(
        &self,
        slave: ClockTime,
        master: ClockTime,
    ) -> Option<(f64, ClockTime, ClockTime, ClockTime, ClockTime)> {
        unsafe {
            let mut r_squared = mem::MaybeUninit::uninit();
            let mut internal = mem::MaybeUninit::uninit();
            let mut external = mem::MaybeUninit::uninit();
            let mut rate_num = mem::MaybeUninit::uninit();
            let mut rate_denom = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_clock_add_observation_unapplied(
                self.as_ref().to_glib_none().0,
                slave.into_glib(),
                master.into_glib(),
                r_squared.as_mut_ptr(),
                internal.as_mut_ptr(),
                external.as_mut_ptr(),
                rate_num.as_mut_ptr(),
                rate_denom.as_mut_ptr(),
            ));
            let r_squared = r_squared.assume_init();
            let internal = internal.assume_init();
            let external = external.assume_init();
            let rate_num = rate_num.assume_init();
            let rate_denom = rate_denom.assume_init();
            if ret {
                Some((
                    r_squared,
                    try_from_glib(internal).expect("mandatory glib value is None"),
                    try_from_glib(external).expect("mandatory glib value is None"),
                    try_from_glib(rate_num).expect("mandatory glib value is None"),
                    try_from_glib(rate_denom).expect("mandatory glib value is None"),
                ))
            } else {
                None
            }
        }
    }

    fn adjust_unlocked(&self, internal: ClockTime) -> Option<ClockTime> {
        unsafe {
            from_glib(ffi::gst_clock_adjust_unlocked(
                self.as_ref().to_glib_none().0,
                internal.into_glib(),
            ))
        }
    }

    fn calibration(&self) -> (ClockTime, ClockTime, ClockTime, ClockTime) {
        unsafe {
            let mut internal = mem::MaybeUninit::uninit();
            let mut external = mem::MaybeUninit::uninit();
            let mut rate_num = mem::MaybeUninit::uninit();
            let mut rate_denom = mem::MaybeUninit::uninit();
            ffi::gst_clock_get_calibration(
                self.as_ref().to_glib_none().0,
                internal.as_mut_ptr(),
                external.as_mut_ptr(),
                rate_num.as_mut_ptr(),
                rate_denom.as_mut_ptr(),
            );
            let internal = internal.assume_init();
            let external = external.assume_init();
            let rate_num = rate_num.assume_init();
            let rate_denom = rate_denom.assume_init();
            (
                try_from_glib(internal).expect("mandatory glib value is None"),
                try_from_glib(external).expect("mandatory glib value is None"),
                try_from_glib(rate_num).expect("mandatory glib value is None"),
                try_from_glib(rate_denom).expect("mandatory glib value is None"),
            )
        }
    }

    fn internal_time(&self) -> ClockTime {
        unsafe {
            try_from_glib(ffi::gst_clock_get_internal_time(
                self.as_ref().to_glib_none().0,
            ))
            .expect("mandatory glib value is None")
        }
    }

    fn master(&self) -> Option<Clock> {
        unsafe { from_glib_full(ffi::gst_clock_get_master(self.as_ref().to_glib_none().0)) }
    }

    fn resolution(&self) -> ClockTime {
        unsafe {
            try_from_glib(ffi::gst_clock_get_resolution(
                self.as_ref().to_glib_none().0,
            ))
            .expect("mandatory glib value is None")
        }
    }

    fn time(&self) -> Option<ClockTime> {
        unsafe { from_glib(ffi::gst_clock_get_time(self.as_ref().to_glib_none().0)) }
    }

    fn timeout(&self) -> Option<ClockTime> {
        unsafe { from_glib(ffi::gst_clock_get_timeout(self.as_ref().to_glib_none().0)) }
    }

    fn is_synced(&self) -> bool {
        unsafe { from_glib(ffi::gst_clock_is_synced(self.as_ref().to_glib_none().0)) }
    }

    fn set_calibration(
        &self,
        internal: ClockTime,
        external: ClockTime,
        rate_num: ClockTime,
        rate_denom: ClockTime,
    ) {
        unsafe {
            ffi::gst_clock_set_calibration(
                self.as_ref().to_glib_none().0,
                internal.into_glib(),
                external.into_glib(),
                rate_num.into_glib(),
                rate_denom.into_glib(),
            );
        }
    }

    fn set_master(&self, master: Option<&impl IsA<Clock>>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_clock_set_master(
                    self.as_ref().to_glib_none().0,
                    master.map(|p| p.as_ref()).to_glib_none().0
                ),
                "Failed to set master clock"
            )
        }
    }

    fn set_resolution(&self, resolution: ClockTime) -> ClockTime {
        unsafe {
            try_from_glib(ffi::gst_clock_set_resolution(
                self.as_ref().to_glib_none().0,
                resolution.into_glib(),
            ))
            .expect("mandatory glib value is None")
        }
    }

    fn set_synced(&self, synced: bool) {
        unsafe {
            ffi::gst_clock_set_synced(self.as_ref().to_glib_none().0, synced.into_glib());
        }
    }

    fn set_timeout(&self, timeout: impl Into<Option<ClockTime>>) {
        unsafe {
            ffi::gst_clock_set_timeout(self.as_ref().to_glib_none().0, timeout.into().into_glib());
        }
    }

    fn unadjust_unlocked(&self, external: ClockTime) -> Option<ClockTime> {
        unsafe {
            from_glib(ffi::gst_clock_unadjust_unlocked(
                self.as_ref().to_glib_none().0,
                external.into_glib(),
            ))
        }
    }

    fn wait_for_sync(
        &self,
        timeout: impl Into<Option<ClockTime>>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_clock_wait_for_sync(
                    self.as_ref().to_glib_none().0,
                    timeout.into().into_glib()
                ),
                "Timed out waiting for sync"
            )
        }
    }

    fn window_size(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"window-size\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `window-size` getter")
        }
    }

    fn set_window_size(&self, window_size: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"window-size\0".as_ptr() as *const _,
                window_size.to_value().to_glib_none().0,
            );
        }
    }

    fn window_threshold(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"window-threshold\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `window-threshold` getter")
        }
    }

    fn set_window_threshold(&self, window_threshold: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"window-threshold\0".as_ptr() as *const _,
                window_threshold.to_value().to_glib_none().0,
            );
        }
    }

    fn connect_synced<F: Fn(&Self, bool) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn synced_trampoline<
            P: IsA<Clock>,
            F: Fn(&P, bool) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstClock,
            synced: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Clock::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(synced),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"synced\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    synced_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeout_trampoline<
            P: IsA<Clock>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstClock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Clock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_window_size_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_window_size_trampoline<
            P: IsA<Clock>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstClock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Clock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::window-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_window_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_window_threshold_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_window_threshold_trampoline<
            P: IsA<Clock>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstClock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Clock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::window-threshold\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_window_threshold_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
