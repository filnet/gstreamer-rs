// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Bus;
use crate::Caps;
use crate::Clock;
use crate::ClockTime;
use crate::Context;
use crate::ElementFactory;
use crate::Object;
use crate::Pad;
use crate::PadLinkCheck;
use crate::PadTemplate;
use crate::State;
use crate::StateChange;
use crate::StateChangeError;
use crate::StateChangeReturn;
use crate::StateChangeSuccess;
use crate::URIType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GstElement")]
    pub struct Element(Object<ffi::GstElement, ffi::GstElementClass>) @extends Object;

    match fn {
        type_ => || ffi::gst_element_get_type(),
    }
}

impl Element {
    pub const NONE: Option<&'static Element> = None;

    #[doc(alias = "gst_element_make_from_uri")]
    pub fn make_from_uri(
        type_: URIType,
        uri: &str,
        elementname: Option<&str>,
    ) -> Result<Element, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_element_make_from_uri(
                type_.into_glib(),
                uri.to_glib_none().0,
                elementname.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

unsafe impl Send for Element {}
unsafe impl Sync for Element {}

pub trait ElementExt: 'static {
    #[doc(alias = "gst_element_abort_state")]
    fn abort_state(&self);

    #[doc(alias = "gst_element_add_pad")]
    fn add_pad(&self, pad: &impl IsA<Pad>) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_element_change_state")]
    fn change_state(&self, transition: StateChange)
        -> Result<StateChangeSuccess, StateChangeError>;

    #[doc(alias = "gst_element_continue_state")]
    fn continue_state(
        &self,
        ret: impl Into<StateChangeReturn>,
    ) -> Result<StateChangeSuccess, StateChangeError>;

    #[doc(alias = "gst_element_create_all_pads")]
    fn create_all_pads(&self);

    #[doc(alias = "gst_element_foreach_pad")]
    fn foreach_pad<P: FnMut(&Element, &Pad) -> bool>(&self, func: P) -> bool;

    #[doc(alias = "gst_element_foreach_sink_pad")]
    fn foreach_sink_pad<P: FnMut(&Element, &Pad) -> bool>(&self, func: P) -> bool;

    #[doc(alias = "gst_element_foreach_src_pad")]
    fn foreach_src_pad<P: FnMut(&Element, &Pad) -> bool>(&self, func: P) -> bool;

    #[doc(alias = "gst_element_get_base_time")]
    #[doc(alias = "get_base_time")]
    fn base_time(&self) -> Option<ClockTime>;

    #[doc(alias = "gst_element_get_bus")]
    #[doc(alias = "get_bus")]
    fn bus(&self) -> Option<Bus>;

    #[doc(alias = "gst_element_get_clock")]
    #[doc(alias = "get_clock")]
    fn clock(&self) -> Option<Clock>;

    #[doc(alias = "gst_element_get_compatible_pad")]
    #[doc(alias = "get_compatible_pad")]
    fn compatible_pad(&self, pad: &impl IsA<Pad>, caps: Option<&Caps>) -> Option<Pad>;

    #[doc(alias = "gst_element_get_compatible_pad_template")]
    #[doc(alias = "get_compatible_pad_template")]
    fn compatible_pad_template(&self, compattempl: &PadTemplate) -> Option<PadTemplate>;

    #[doc(alias = "gst_element_get_context")]
    #[doc(alias = "get_context")]
    fn context(&self, context_type: &str) -> Option<Context>;

    #[doc(alias = "gst_element_get_contexts")]
    #[doc(alias = "get_contexts")]
    fn contexts(&self) -> Vec<Context>;

    #[doc(alias = "gst_element_get_factory")]
    #[doc(alias = "get_factory")]
    fn factory(&self) -> Option<ElementFactory>;

    #[doc(alias = "gst_element_get_start_time")]
    #[doc(alias = "get_start_time")]
    fn start_time(&self) -> Option<ClockTime>;

    #[doc(alias = "gst_element_get_state")]
    #[doc(alias = "get_state")]
    fn state(
        &self,
        timeout: impl Into<Option<ClockTime>>,
    ) -> (Result<StateChangeSuccess, StateChangeError>, State, State);

    #[doc(alias = "gst_element_get_static_pad")]
    #[doc(alias = "get_static_pad")]
    fn static_pad(&self, name: &str) -> Option<Pad>;

    #[doc(alias = "gst_element_is_locked_state")]
    fn is_locked_state(&self) -> bool;

    #[doc(alias = "gst_element_link")]
    fn link(&self, dest: &impl IsA<Element>) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_element_link_filtered")]
    fn link_filtered(
        &self,
        dest: &impl IsA<Element>,
        filter: &Caps,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_element_link_pads")]
    fn link_pads(
        &self,
        srcpadname: Option<&str>,
        dest: &impl IsA<Element>,
        destpadname: Option<&str>,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_element_link_pads_filtered")]
    fn link_pads_filtered(
        &self,
        srcpadname: Option<&str>,
        dest: &impl IsA<Element>,
        destpadname: Option<&str>,
        filter: &Caps,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_element_link_pads_full")]
    fn link_pads_full(
        &self,
        srcpadname: Option<&str>,
        dest: &impl IsA<Element>,
        destpadname: Option<&str>,
        flags: PadLinkCheck,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_element_lost_state")]
    fn lost_state(&self);

    #[doc(alias = "gst_element_no_more_pads")]
    fn no_more_pads(&self);

    #[doc(alias = "gst_element_provide_clock")]
    fn provide_clock(&self) -> Option<Clock>;

    #[doc(alias = "gst_element_release_request_pad")]
    fn release_request_pad(&self, pad: &impl IsA<Pad>);

    #[doc(alias = "gst_element_remove_pad")]
    fn remove_pad(&self, pad: &impl IsA<Pad>) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_element_request_pad")]
    fn request_pad(
        &self,
        templ: &PadTemplate,
        name: Option<&str>,
        caps: Option<&Caps>,
    ) -> Option<Pad>;

    #[doc(alias = "gst_element_set_base_time")]
    fn set_base_time(&self, time: ClockTime);

    #[doc(alias = "gst_element_set_bus")]
    fn set_bus(&self, bus: Option<&Bus>);

    #[doc(alias = "gst_element_set_clock")]
    fn set_clock(&self, clock: Option<&impl IsA<Clock>>) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_element_set_context")]
    fn set_context(&self, context: &Context);

    #[doc(alias = "gst_element_set_locked_state")]
    fn set_locked_state(&self, locked_state: bool) -> bool;

    #[doc(alias = "gst_element_set_start_time")]
    fn set_start_time(&self, time: impl Into<Option<ClockTime>>);

    #[doc(alias = "gst_element_set_state")]
    fn set_state(&self, state: State) -> Result<StateChangeSuccess, StateChangeError>;

    #[doc(alias = "gst_element_sync_state_with_parent")]
    fn sync_state_with_parent(&self) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_element_unlink")]
    fn unlink(&self, dest: &impl IsA<Element>);

    #[doc(alias = "gst_element_unlink_pads")]
    fn unlink_pads(&self, srcpadname: &str, dest: &impl IsA<Element>, destpadname: &str);

    #[doc(alias = "no-more-pads")]
    fn connect_no_more_pads<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "pad-added")]
    fn connect_pad_added<F: Fn(&Self, &Pad) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "pad-removed")]
    fn connect_pad_removed<F: Fn(&Self, &Pad) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Element>> ElementExt for O {
    fn abort_state(&self) {
        unsafe {
            ffi::gst_element_abort_state(self.as_ref().to_glib_none().0);
        }
    }

    fn add_pad(&self, pad: &impl IsA<Pad>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_add_pad(
                    self.as_ref().to_glib_none().0,
                    pad.as_ref().to_glib_none().0
                ),
                "Failed to add pad"
            )
        }
    }

    fn change_state(
        &self,
        transition: StateChange,
    ) -> Result<StateChangeSuccess, StateChangeError> {
        unsafe {
            try_from_glib(ffi::gst_element_change_state(
                self.as_ref().to_glib_none().0,
                transition.into_glib(),
            ))
        }
    }

    fn continue_state(
        &self,
        ret: impl Into<StateChangeReturn>,
    ) -> Result<StateChangeSuccess, StateChangeError> {
        unsafe {
            try_from_glib(ffi::gst_element_continue_state(
                self.as_ref().to_glib_none().0,
                ret.into().into_glib(),
            ))
        }
    }

    fn create_all_pads(&self) {
        unsafe {
            ffi::gst_element_create_all_pads(self.as_ref().to_glib_none().0);
        }
    }

    fn foreach_pad<P: FnMut(&Element, &Pad) -> bool>(&self, func: P) -> bool {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&Element, &Pad) -> bool>(
            element: *mut ffi::GstElement,
            pad: *mut ffi::GstPad,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let element = from_glib_borrow(element);
            let pad = from_glib_borrow(pad);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let res = (*callback)(&element, &pad);
            res.into_glib()
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            from_glib(ffi::gst_element_foreach_pad(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            ))
        }
    }

    fn foreach_sink_pad<P: FnMut(&Element, &Pad) -> bool>(&self, func: P) -> bool {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&Element, &Pad) -> bool>(
            element: *mut ffi::GstElement,
            pad: *mut ffi::GstPad,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let element = from_glib_borrow(element);
            let pad = from_glib_borrow(pad);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let res = (*callback)(&element, &pad);
            res.into_glib()
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            from_glib(ffi::gst_element_foreach_sink_pad(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            ))
        }
    }

    fn foreach_src_pad<P: FnMut(&Element, &Pad) -> bool>(&self, func: P) -> bool {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&Element, &Pad) -> bool>(
            element: *mut ffi::GstElement,
            pad: *mut ffi::GstPad,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let element = from_glib_borrow(element);
            let pad = from_glib_borrow(pad);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let res = (*callback)(&element, &pad);
            res.into_glib()
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            from_glib(ffi::gst_element_foreach_src_pad(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            ))
        }
    }

    fn base_time(&self) -> Option<ClockTime> {
        unsafe {
            from_glib(ffi::gst_element_get_base_time(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn bus(&self) -> Option<Bus> {
        unsafe { from_glib_full(ffi::gst_element_get_bus(self.as_ref().to_glib_none().0)) }
    }

    fn clock(&self) -> Option<Clock> {
        unsafe { from_glib_full(ffi::gst_element_get_clock(self.as_ref().to_glib_none().0)) }
    }

    fn compatible_pad(&self, pad: &impl IsA<Pad>, caps: Option<&Caps>) -> Option<Pad> {
        unsafe {
            from_glib_full(ffi::gst_element_get_compatible_pad(
                self.as_ref().to_glib_none().0,
                pad.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
            ))
        }
    }

    fn compatible_pad_template(&self, compattempl: &PadTemplate) -> Option<PadTemplate> {
        unsafe {
            from_glib_none(ffi::gst_element_get_compatible_pad_template(
                self.as_ref().to_glib_none().0,
                compattempl.to_glib_none().0,
            ))
        }
    }

    fn context(&self, context_type: &str) -> Option<Context> {
        unsafe {
            from_glib_full(ffi::gst_element_get_context(
                self.as_ref().to_glib_none().0,
                context_type.to_glib_none().0,
            ))
        }
    }

    fn contexts(&self) -> Vec<Context> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_element_get_contexts(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn factory(&self) -> Option<ElementFactory> {
        unsafe { from_glib_none(ffi::gst_element_get_factory(self.as_ref().to_glib_none().0)) }
    }

    fn start_time(&self) -> Option<ClockTime> {
        unsafe {
            from_glib(ffi::gst_element_get_start_time(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn state(
        &self,
        timeout: impl Into<Option<ClockTime>>,
    ) -> (Result<StateChangeSuccess, StateChangeError>, State, State) {
        unsafe {
            let mut state = mem::MaybeUninit::uninit();
            let mut pending = mem::MaybeUninit::uninit();
            let ret = try_from_glib(ffi::gst_element_get_state(
                self.as_ref().to_glib_none().0,
                state.as_mut_ptr(),
                pending.as_mut_ptr(),
                timeout.into().into_glib(),
            ));
            let state = state.assume_init();
            let pending = pending.assume_init();
            (ret, from_glib(state), from_glib(pending))
        }
    }

    fn static_pad(&self, name: &str) -> Option<Pad> {
        unsafe {
            from_glib_full(ffi::gst_element_get_static_pad(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn is_locked_state(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_element_is_locked_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn link(&self, dest: &impl IsA<Element>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_link(
                    self.as_ref().to_glib_none().0,
                    dest.as_ref().to_glib_none().0
                ),
                "Failed to link elements"
            )
        }
    }

    fn link_filtered(
        &self,
        dest: &impl IsA<Element>,
        filter: &Caps,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_link_filtered(
                    self.as_ref().to_glib_none().0,
                    dest.as_ref().to_glib_none().0,
                    filter.to_glib_none().0
                ),
                "Failed to link elements"
            )
        }
    }

    fn link_pads(
        &self,
        srcpadname: Option<&str>,
        dest: &impl IsA<Element>,
        destpadname: Option<&str>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_link_pads(
                    self.as_ref().to_glib_none().0,
                    srcpadname.to_glib_none().0,
                    dest.as_ref().to_glib_none().0,
                    destpadname.to_glib_none().0
                ),
                "Failed to link pads"
            )
        }
    }

    fn link_pads_filtered(
        &self,
        srcpadname: Option<&str>,
        dest: &impl IsA<Element>,
        destpadname: Option<&str>,
        filter: &Caps,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_link_pads_filtered(
                    self.as_ref().to_glib_none().0,
                    srcpadname.to_glib_none().0,
                    dest.as_ref().to_glib_none().0,
                    destpadname.to_glib_none().0,
                    filter.to_glib_none().0
                ),
                "Failed to link pads"
            )
        }
    }

    fn link_pads_full(
        &self,
        srcpadname: Option<&str>,
        dest: &impl IsA<Element>,
        destpadname: Option<&str>,
        flags: PadLinkCheck,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_link_pads_full(
                    self.as_ref().to_glib_none().0,
                    srcpadname.to_glib_none().0,
                    dest.as_ref().to_glib_none().0,
                    destpadname.to_glib_none().0,
                    flags.into_glib()
                ),
                "Failed to link pads"
            )
        }
    }

    fn lost_state(&self) {
        unsafe {
            ffi::gst_element_lost_state(self.as_ref().to_glib_none().0);
        }
    }

    fn no_more_pads(&self) {
        unsafe {
            ffi::gst_element_no_more_pads(self.as_ref().to_glib_none().0);
        }
    }

    fn provide_clock(&self) -> Option<Clock> {
        unsafe {
            from_glib_full(ffi::gst_element_provide_clock(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn release_request_pad(&self, pad: &impl IsA<Pad>) {
        unsafe {
            ffi::gst_element_release_request_pad(
                self.as_ref().to_glib_none().0,
                pad.as_ref().to_glib_none().0,
            );
        }
    }

    fn remove_pad(&self, pad: &impl IsA<Pad>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_remove_pad(
                    self.as_ref().to_glib_none().0,
                    pad.as_ref().to_glib_none().0
                ),
                "Failed to remove pad"
            )
        }
    }

    fn request_pad(
        &self,
        templ: &PadTemplate,
        name: Option<&str>,
        caps: Option<&Caps>,
    ) -> Option<Pad> {
        unsafe {
            from_glib_full(ffi::gst_element_request_pad(
                self.as_ref().to_glib_none().0,
                templ.to_glib_none().0,
                name.to_glib_none().0,
                caps.to_glib_none().0,
            ))
        }
    }

    fn set_base_time(&self, time: ClockTime) {
        unsafe {
            ffi::gst_element_set_base_time(self.as_ref().to_glib_none().0, time.into_glib());
        }
    }

    fn set_bus(&self, bus: Option<&Bus>) {
        unsafe {
            ffi::gst_element_set_bus(self.as_ref().to_glib_none().0, bus.to_glib_none().0);
        }
    }

    fn set_clock(&self, clock: Option<&impl IsA<Clock>>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_set_clock(
                    self.as_ref().to_glib_none().0,
                    clock.map(|p| p.as_ref()).to_glib_none().0
                ),
                "Failed to set clock"
            )
        }
    }

    fn set_context(&self, context: &Context) {
        unsafe {
            ffi::gst_element_set_context(self.as_ref().to_glib_none().0, context.to_glib_none().0);
        }
    }

    fn set_locked_state(&self, locked_state: bool) -> bool {
        unsafe {
            from_glib(ffi::gst_element_set_locked_state(
                self.as_ref().to_glib_none().0,
                locked_state.into_glib(),
            ))
        }
    }

    fn set_start_time(&self, time: impl Into<Option<ClockTime>>) {
        unsafe {
            ffi::gst_element_set_start_time(
                self.as_ref().to_glib_none().0,
                time.into().into_glib(),
            );
        }
    }

    fn set_state(&self, state: State) -> Result<StateChangeSuccess, StateChangeError> {
        unsafe {
            try_from_glib(ffi::gst_element_set_state(
                self.as_ref().to_glib_none().0,
                state.into_glib(),
            ))
        }
    }

    fn sync_state_with_parent(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_sync_state_with_parent(self.as_ref().to_glib_none().0),
                "Failed to sync state with parent"
            )
        }
    }

    fn unlink(&self, dest: &impl IsA<Element>) {
        unsafe {
            ffi::gst_element_unlink(
                self.as_ref().to_glib_none().0,
                dest.as_ref().to_glib_none().0,
            );
        }
    }

    fn unlink_pads(&self, srcpadname: &str, dest: &impl IsA<Element>, destpadname: &str) {
        unsafe {
            ffi::gst_element_unlink_pads(
                self.as_ref().to_glib_none().0,
                srcpadname.to_glib_none().0,
                dest.as_ref().to_glib_none().0,
                destpadname.to_glib_none().0,
            );
        }
    }

    fn connect_no_more_pads<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn no_more_pads_trampoline<
            P: IsA<Element>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstElement,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Element::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"no-more-pads\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    no_more_pads_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_pad_added<F: Fn(&Self, &Pad) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pad_added_trampoline<
            P: IsA<Element>,
            F: Fn(&P, &Pad) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstElement,
            new_pad: *mut ffi::GstPad,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Element::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(new_pad),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pad-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    pad_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_pad_removed<F: Fn(&Self, &Pad) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pad_removed_trampoline<
            P: IsA<Element>,
            F: Fn(&P, &Pad) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstElement,
            old_pad: *mut ffi::GstPad,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Element::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(old_pad),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pad-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    pad_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
