// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gst_sys;
use std::boxed::Box as Box_;
use Caps;
use Object;
use Pad;
use PadDirection;
use PadPresence;

glib_wrapper! {
    pub struct PadTemplate(Object<gst_sys::GstPadTemplate, gst_sys::GstPadTemplateClass, PadTemplateClass>) @extends Object;

    match fn {
        get_type => || gst_sys::gst_pad_template_get_type(),
    }
}

impl PadTemplate {
    pub fn new(
        name_template: &str,
        direction: PadDirection,
        presence: PadPresence,
        caps: &Caps,
    ) -> Result<PadTemplate, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_none(gst_sys::gst_pad_template_new(
                name_template.to_glib_none().0,
                direction.to_glib(),
                presence.to_glib(),
                caps.to_glib_none().0,
            ))
            .ok_or_else(|| glib_bool_error!("Failed to create pad template"))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn new_with_gtype(
        name_template: &str,
        direction: PadDirection,
        presence: PadPresence,
        caps: &Caps,
        pad_type: glib::types::Type,
    ) -> Result<PadTemplate, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_none(gst_sys::gst_pad_template_new_with_gtype(
                name_template.to_glib_none().0,
                direction.to_glib(),
                presence.to_glib(),
                caps.to_glib_none().0,
                pad_type.to_glib(),
            ))
            .ok_or_else(|| glib_bool_error!("Failed to create pad template"))
        }
    }

    pub fn get_caps(&self) -> Option<Caps> {
        unsafe { from_glib_full(gst_sys::gst_pad_template_get_caps(self.to_glib_none().0)) }
    }

    pub fn pad_created<P: IsA<Pad>>(&self, pad: &P) {
        unsafe {
            gst_sys::gst_pad_template_pad_created(
                self.to_glib_none().0,
                pad.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn get_property_direction(&self) -> PadDirection {
        unsafe {
            let mut value = Value::from_type(<PadDirection as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"direction\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `direction` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_property_gtype(&self) -> glib::types::Type {
        unsafe {
            let mut value = Value::from_type(<glib::types::Type as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"gtype\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `gtype` getter")
                .unwrap()
        }
    }

    pub fn get_property_name_template(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"name-template\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `name-template` getter")
        }
    }

    pub fn get_property_presence(&self) -> PadPresence {
        unsafe {
            let mut value = Value::from_type(<PadPresence as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"presence\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `presence` getter")
                .unwrap()
        }
    }

    pub fn connect_pad_created<F: Fn(&PadTemplate, &Pad) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pad_created_trampoline<
            F: Fn(&PadTemplate, &Pad) + Send + Sync + 'static,
        >(
            this: *mut gst_sys::GstPadTemplate,
            pad: *mut gst_sys::GstPad,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(pad))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pad-created\0".as_ptr() as *const _,
                Some(*(&pad_created_trampoline::<F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for PadTemplate {}
unsafe impl Sync for PadTemplate {}
