// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bus;
use Caps;
use Clock;
use ClockTime;
use Context;
use ElementFactory;
use Error;
use Message;
use Object;
use Pad;
use PadLinkCheck;
use PadTemplate;
use Plugin;
use URIType;
use ffi;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Element(Object<ffi::GstElement, ffi::GstElementClass, ElementClass>) @extends Object;

    match fn {
        get_type => || ffi::gst_element_get_type(),
    }
}

impl Element {
    pub fn make_from_uri<'a, P: Into<Option<&'a str>>>(type_: URIType, uri: &str, elementname: P) -> Result<Element, Error> {
        assert_initialized_main_thread!();
        let elementname = elementname.into();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_element_make_from_uri(type_.to_glib(), uri.to_glib_none().0, elementname.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn register<'a, P: Into<Option<&'a Plugin>>>(plugin: P, name: &str, rank: u32, type_: glib::types::Type) -> Result<(), glib::error::BoolError> {
        assert_initialized_main_thread!();
        let plugin = plugin.into();
        unsafe {
            glib_result_from_gboolean!(ffi::gst_element_register(plugin.to_glib_none().0, name.to_glib_none().0, rank, type_.to_glib()), "Failed to register element factory")
        }
    }
}

unsafe impl Send for Element {}
unsafe impl Sync for Element {}

pub const NONE_ELEMENT: Option<&Element> = None;

pub trait ElementExt: 'static {
    fn abort_state(&self);

    fn add_pad<P: IsA<Pad>>(&self, pad: &P) -> Result<(), glib::error::BoolError>;

    fn create_all_pads(&self);

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn foreach_pad<P: FnMut(&Element, &Pad) -> bool>(&self, func: P) -> bool;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn foreach_sink_pad<P: FnMut(&Element, &Pad) -> bool>(&self, func: P) -> bool;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn foreach_src_pad<P: FnMut(&Element, &Pad) -> bool>(&self, func: P) -> bool;

    fn get_base_time(&self) -> ClockTime;

    fn get_bus(&self) -> Option<Bus>;

    fn get_clock(&self) -> Option<Clock>;

    fn get_compatible_pad<'a, P: IsA<Pad>, Q: Into<Option<&'a Caps>>>(&self, pad: &P, caps: Q) -> Option<Pad>;

    fn get_compatible_pad_template(&self, compattempl: &PadTemplate) -> Option<PadTemplate>;

    fn get_context(&self, context_type: &str) -> Option<Context>;

    fn get_contexts(&self) -> Vec<Context>;

    fn get_factory(&self) -> Option<ElementFactory>;

    fn get_request_pad(&self, name: &str) -> Option<Pad>;

    fn get_start_time(&self) -> ClockTime;

    fn get_static_pad(&self, name: &str) -> Option<Pad>;

    fn is_locked_state(&self) -> bool;

    //fn iterate_pads(&self) -> /*Ignored*/Option<Iterator>;

    //fn iterate_sink_pads(&self) -> /*Ignored*/Option<Iterator>;

    //fn iterate_src_pads(&self) -> /*Ignored*/Option<Iterator>;

    fn link<P: IsA<Element>>(&self, dest: &P) -> Result<(), glib::error::BoolError>;

    fn link_filtered<'a, P: IsA<Element>, Q: Into<Option<&'a Caps>>>(&self, dest: &P, filter: Q) -> Result<(), glib::error::BoolError>;

    //fn link_many<P: IsA<Element>>(&self, element_2: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool;

    fn link_pads<'a, 'b, P: Into<Option<&'a str>>, Q: IsA<Element>, R: Into<Option<&'b str>>>(&self, srcpadname: P, dest: &Q, destpadname: R) -> Result<(), glib::error::BoolError>;

    fn link_pads_filtered<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: IsA<Element>, R: Into<Option<&'b str>>, S: Into<Option<&'c Caps>>>(&self, srcpadname: P, dest: &Q, destpadname: R, filter: S) -> Result<(), glib::error::BoolError>;

    fn link_pads_full<'a, 'b, P: Into<Option<&'a str>>, Q: IsA<Element>, R: Into<Option<&'b str>>>(&self, srcpadname: P, dest: &Q, destpadname: R, flags: PadLinkCheck) -> Result<(), glib::error::BoolError>;

    fn lost_state(&self);

    //fn message_full<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, type_: /*Ignored*/MessageType, domain: /*Ignored*/glib::Quark, code: i32, text: P, debug: Q, file: &str, function: &str, line: i32);

    //#[cfg(any(feature = "v1_10", feature = "dox"))]
    //fn message_full_with_details<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, type_: /*Ignored*/MessageType, domain: /*Ignored*/glib::Quark, code: i32, text: P, debug: Q, file: &str, function: &str, line: i32, structure: &mut Structure);

    fn no_more_pads(&self);

    fn post_message(&self, message: &Message) -> Result<(), glib::error::BoolError>;

    fn provide_clock(&self) -> Option<Clock>;

    fn release_request_pad<P: IsA<Pad>>(&self, pad: &P);

    fn remove_pad<P: IsA<Pad>>(&self, pad: &P) -> Result<(), glib::error::BoolError>;

    fn request_pad<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b Caps>>>(&self, templ: &PadTemplate, name: P, caps: Q) -> Option<Pad>;

    fn set_base_time(&self, time: ClockTime);

    fn set_bus<'a, P: Into<Option<&'a Bus>>>(&self, bus: P);

    fn set_clock<'a, P: IsA<Clock> + 'a, Q: Into<Option<&'a P>>>(&self, clock: Q) -> Result<(), glib::error::BoolError>;

    fn set_context(&self, context: &Context);

    fn set_locked_state(&self, locked_state: bool) -> bool;

    fn set_start_time(&self, time: ClockTime);

    fn sync_state_with_parent(&self) -> Result<(), glib::error::BoolError>;

    fn unlink<P: IsA<Element>>(&self, dest: &P);

    //fn unlink_many<P: IsA<Element>>(&self, element_2: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn unlink_pads<P: IsA<Element>>(&self, srcpadname: &str, dest: &P, destpadname: &str);

    fn connect_no_more_pads<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_pad_added<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_pad_removed<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Element>> ElementExt for O {
    fn abort_state(&self) {
        unsafe {
            ffi::gst_element_abort_state(self.as_ref().to_glib_none().0);
        }
    }

    fn add_pad<P: IsA<Pad>>(&self, pad: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ffi::gst_element_add_pad(self.as_ref().to_glib_none().0, pad.as_ref().to_glib_none().0), "Failed to add pad")
        }
    }

    fn create_all_pads(&self) {
        unsafe {
            ffi::gst_element_create_all_pads(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn foreach_pad<P: FnMut(&Element, &Pad) -> bool>(&self, func: P) -> bool {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&Element, &Pad) -> bool>(element: *mut ffi::GstElement, pad: *mut ffi::GstPad, user_data: glib_ffi::gpointer) -> glib_ffi::gboolean {
            let element = from_glib_borrow(element);
            let pad = from_glib_borrow(pad);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let res = (*callback)(&element, &pad);
            res.to_glib()
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            from_glib(ffi::gst_element_foreach_pad(self.as_ref().to_glib_none().0, func, super_callback0 as *const _ as usize as *mut _))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn foreach_sink_pad<P: FnMut(&Element, &Pad) -> bool>(&self, func: P) -> bool {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&Element, &Pad) -> bool>(element: *mut ffi::GstElement, pad: *mut ffi::GstPad, user_data: glib_ffi::gpointer) -> glib_ffi::gboolean {
            let element = from_glib_borrow(element);
            let pad = from_glib_borrow(pad);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let res = (*callback)(&element, &pad);
            res.to_glib()
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            from_glib(ffi::gst_element_foreach_sink_pad(self.as_ref().to_glib_none().0, func, super_callback0 as *const _ as usize as *mut _))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn foreach_src_pad<P: FnMut(&Element, &Pad) -> bool>(&self, func: P) -> bool {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&Element, &Pad) -> bool>(element: *mut ffi::GstElement, pad: *mut ffi::GstPad, user_data: glib_ffi::gpointer) -> glib_ffi::gboolean {
            let element = from_glib_borrow(element);
            let pad = from_glib_borrow(pad);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let res = (*callback)(&element, &pad);
            res.to_glib()
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            from_glib(ffi::gst_element_foreach_src_pad(self.as_ref().to_glib_none().0, func, super_callback0 as *const _ as usize as *mut _))
        }
    }

    fn get_base_time(&self) -> ClockTime {
        unsafe {
            from_glib(ffi::gst_element_get_base_time(self.as_ref().to_glib_none().0))
        }
    }

    fn get_bus(&self) -> Option<Bus> {
        unsafe {
            from_glib_full(ffi::gst_element_get_bus(self.as_ref().to_glib_none().0))
        }
    }

    fn get_clock(&self) -> Option<Clock> {
        unsafe {
            from_glib_full(ffi::gst_element_get_clock(self.as_ref().to_glib_none().0))
        }
    }

    fn get_compatible_pad<'a, P: IsA<Pad>, Q: Into<Option<&'a Caps>>>(&self, pad: &P, caps: Q) -> Option<Pad> {
        let caps = caps.into();
        unsafe {
            from_glib_full(ffi::gst_element_get_compatible_pad(self.as_ref().to_glib_none().0, pad.as_ref().to_glib_none().0, caps.to_glib_none().0))
        }
    }

    fn get_compatible_pad_template(&self, compattempl: &PadTemplate) -> Option<PadTemplate> {
        unsafe {
            from_glib_none(ffi::gst_element_get_compatible_pad_template(self.as_ref().to_glib_none().0, compattempl.to_glib_none().0))
        }
    }

    fn get_context(&self, context_type: &str) -> Option<Context> {
        unsafe {
            from_glib_full(ffi::gst_element_get_context(self.as_ref().to_glib_none().0, context_type.to_glib_none().0))
        }
    }

    fn get_contexts(&self) -> Vec<Context> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_element_get_contexts(self.as_ref().to_glib_none().0))
        }
    }

    fn get_factory(&self) -> Option<ElementFactory> {
        unsafe {
            from_glib_none(ffi::gst_element_get_factory(self.as_ref().to_glib_none().0))
        }
    }

    fn get_request_pad(&self, name: &str) -> Option<Pad> {
        unsafe {
            from_glib_full(ffi::gst_element_get_request_pad(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_start_time(&self) -> ClockTime {
        unsafe {
            from_glib(ffi::gst_element_get_start_time(self.as_ref().to_glib_none().0))
        }
    }

    fn get_static_pad(&self, name: &str) -> Option<Pad> {
        unsafe {
            from_glib_full(ffi::gst_element_get_static_pad(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn is_locked_state(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_element_is_locked_state(self.as_ref().to_glib_none().0))
        }
    }

    //fn iterate_pads(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi::gst_element_iterate_pads() }
    //}

    //fn iterate_sink_pads(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi::gst_element_iterate_sink_pads() }
    //}

    //fn iterate_src_pads(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi::gst_element_iterate_src_pads() }
    //}

    fn link<P: IsA<Element>>(&self, dest: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ffi::gst_element_link(self.as_ref().to_glib_none().0, dest.as_ref().to_glib_none().0), "Failed to link elements")
        }
    }

    fn link_filtered<'a, P: IsA<Element>, Q: Into<Option<&'a Caps>>>(&self, dest: &P, filter: Q) -> Result<(), glib::error::BoolError> {
        let filter = filter.into();
        unsafe {
            glib_result_from_gboolean!(ffi::gst_element_link_filtered(self.as_ref().to_glib_none().0, dest.as_ref().to_glib_none().0, filter.to_glib_none().0), "Failed to link elements")
        }
    }

    //fn link_many<P: IsA<Element>>(&self, element_2: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call ffi::gst_element_link_many() }
    //}

    fn link_pads<'a, 'b, P: Into<Option<&'a str>>, Q: IsA<Element>, R: Into<Option<&'b str>>>(&self, srcpadname: P, dest: &Q, destpadname: R) -> Result<(), glib::error::BoolError> {
        let srcpadname = srcpadname.into();
        let destpadname = destpadname.into();
        unsafe {
            glib_result_from_gboolean!(ffi::gst_element_link_pads(self.as_ref().to_glib_none().0, srcpadname.to_glib_none().0, dest.as_ref().to_glib_none().0, destpadname.to_glib_none().0), "Failed to link pads")
        }
    }

    fn link_pads_filtered<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: IsA<Element>, R: Into<Option<&'b str>>, S: Into<Option<&'c Caps>>>(&self, srcpadname: P, dest: &Q, destpadname: R, filter: S) -> Result<(), glib::error::BoolError> {
        let srcpadname = srcpadname.into();
        let destpadname = destpadname.into();
        let filter = filter.into();
        unsafe {
            glib_result_from_gboolean!(ffi::gst_element_link_pads_filtered(self.as_ref().to_glib_none().0, srcpadname.to_glib_none().0, dest.as_ref().to_glib_none().0, destpadname.to_glib_none().0, filter.to_glib_none().0), "Failed to link pads")
        }
    }

    fn link_pads_full<'a, 'b, P: Into<Option<&'a str>>, Q: IsA<Element>, R: Into<Option<&'b str>>>(&self, srcpadname: P, dest: &Q, destpadname: R, flags: PadLinkCheck) -> Result<(), glib::error::BoolError> {
        let srcpadname = srcpadname.into();
        let destpadname = destpadname.into();
        unsafe {
            glib_result_from_gboolean!(ffi::gst_element_link_pads_full(self.as_ref().to_glib_none().0, srcpadname.to_glib_none().0, dest.as_ref().to_glib_none().0, destpadname.to_glib_none().0, flags.to_glib()), "Failed to link pads")
        }
    }

    fn lost_state(&self) {
        unsafe {
            ffi::gst_element_lost_state(self.as_ref().to_glib_none().0);
        }
    }

    //fn message_full<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, type_: /*Ignored*/MessageType, domain: /*Ignored*/glib::Quark, code: i32, text: P, debug: Q, file: &str, function: &str, line: i32) {
    //    unsafe { TODO: call ffi::gst_element_message_full() }
    //}

    //#[cfg(any(feature = "v1_10", feature = "dox"))]
    //fn message_full_with_details<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, type_: /*Ignored*/MessageType, domain: /*Ignored*/glib::Quark, code: i32, text: P, debug: Q, file: &str, function: &str, line: i32, structure: &mut Structure) {
    //    unsafe { TODO: call ffi::gst_element_message_full_with_details() }
    //}

    fn no_more_pads(&self) {
        unsafe {
            ffi::gst_element_no_more_pads(self.as_ref().to_glib_none().0);
        }
    }

    fn post_message(&self, message: &Message) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ffi::gst_element_post_message(self.as_ref().to_glib_none().0, message.to_glib_full()), "Failed to post message")
        }
    }

    fn provide_clock(&self) -> Option<Clock> {
        unsafe {
            from_glib_full(ffi::gst_element_provide_clock(self.as_ref().to_glib_none().0))
        }
    }

    fn release_request_pad<P: IsA<Pad>>(&self, pad: &P) {
        unsafe {
            ffi::gst_element_release_request_pad(self.as_ref().to_glib_none().0, pad.as_ref().to_glib_none().0);
        }
    }

    fn remove_pad<P: IsA<Pad>>(&self, pad: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ffi::gst_element_remove_pad(self.as_ref().to_glib_none().0, pad.as_ref().to_glib_none().0), "Failed to remove pad")
        }
    }

    fn request_pad<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b Caps>>>(&self, templ: &PadTemplate, name: P, caps: Q) -> Option<Pad> {
        let name = name.into();
        let caps = caps.into();
        unsafe {
            from_glib_full(ffi::gst_element_request_pad(self.as_ref().to_glib_none().0, templ.to_glib_none().0, name.to_glib_none().0, caps.to_glib_none().0))
        }
    }

    fn set_base_time(&self, time: ClockTime) {
        unsafe {
            ffi::gst_element_set_base_time(self.as_ref().to_glib_none().0, time.to_glib());
        }
    }

    fn set_bus<'a, P: Into<Option<&'a Bus>>>(&self, bus: P) {
        let bus = bus.into();
        unsafe {
            ffi::gst_element_set_bus(self.as_ref().to_glib_none().0, bus.to_glib_none().0);
        }
    }

    fn set_clock<'a, P: IsA<Clock> + 'a, Q: Into<Option<&'a P>>>(&self, clock: Q) -> Result<(), glib::error::BoolError> {
        let clock = clock.into();
        unsafe {
            glib_result_from_gboolean!(ffi::gst_element_set_clock(self.as_ref().to_glib_none().0, clock.map(|p| p.as_ref()).to_glib_none().0), "Failed to set clock")
        }
    }

    fn set_context(&self, context: &Context) {
        unsafe {
            ffi::gst_element_set_context(self.as_ref().to_glib_none().0, context.to_glib_none().0);
        }
    }

    fn set_locked_state(&self, locked_state: bool) -> bool {
        unsafe {
            from_glib(ffi::gst_element_set_locked_state(self.as_ref().to_glib_none().0, locked_state.to_glib()))
        }
    }

    fn set_start_time(&self, time: ClockTime) {
        unsafe {
            ffi::gst_element_set_start_time(self.as_ref().to_glib_none().0, time.to_glib());
        }
    }

    fn sync_state_with_parent(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ffi::gst_element_sync_state_with_parent(self.as_ref().to_glib_none().0), "Failed to sync state with parent")
        }
    }

    fn unlink<P: IsA<Element>>(&self, dest: &P) {
        unsafe {
            ffi::gst_element_unlink(self.as_ref().to_glib_none().0, dest.as_ref().to_glib_none().0);
        }
    }

    //fn unlink_many<P: IsA<Element>>(&self, element_2: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gst_element_unlink_many() }
    //}

    fn unlink_pads<P: IsA<Element>>(&self, srcpadname: &str, dest: &P, destpadname: &str) {
        unsafe {
            ffi::gst_element_unlink_pads(self.as_ref().to_glib_none().0, srcpadname.to_glib_none().0, dest.as_ref().to_glib_none().0, destpadname.to_glib_none().0);
        }
    }

    fn connect_no_more_pads<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"no-more-pads\0".as_ptr() as *const _,
                Some(transmute(no_more_pads_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_pad_added<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pad-added\0".as_ptr() as *const _,
                Some(transmute(pad_added_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_pad_removed<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pad-removed\0".as_ptr() as *const _,
                Some(transmute(pad_removed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn no_more_pads_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut ffi::GstElement, f: glib_ffi::gpointer)
where P: IsA<Element> {
    let f: &F = transmute(f);
    f(&Element::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn pad_added_trampoline<P, F: Fn(&P, &Pad) + Send + Sync + 'static>(this: *mut ffi::GstElement, new_pad: *mut ffi::GstPad, f: glib_ffi::gpointer)
where P: IsA<Element> {
    let f: &F = transmute(f);
    f(&Element::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(new_pad))
}

unsafe extern "C" fn pad_removed_trampoline<P, F: Fn(&P, &Pad) + Send + Sync + 'static>(this: *mut ffi::GstElement, old_pad: *mut ffi::GstPad, f: glib_ffi::gpointer)
where P: IsA<Element> {
    let f: &F = transmute(f);
    f(&Element::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(old_pad))
}
