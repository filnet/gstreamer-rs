// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ClockTime;
use crate::ControlBinding;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstObject")]
    pub struct Object(Object<ffi::GstObject, ffi::GstObjectClass>);

    match fn {
        type_ => || ffi::gst_object_get_type(),
    }
}

impl Object {
    #[doc(alias = "gst_object_check_uniqueness")]
    pub fn check_uniqueness(list: &[Object], name: &str) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::gst_object_check_uniqueness(
                list.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "gst_object_default_deep_notify")]
    //pub fn default_deep_notify(object: &impl IsA<glib::Object>, orig: &impl IsA<Object>, pspec: /*Ignored*/&glib::ParamSpec, excluded_props: &[&str]) {
    //    unsafe { TODO: call ffi:gst_object_default_deep_notify() }
    //}

    //#[doc(alias = "gst_object_replace")]
    //pub fn replace(oldobj: Option<impl IsA<Object>>, newobj: Option<&impl IsA<Object>>) -> bool {
    //    unsafe { TODO: call ffi:gst_object_replace() }
    //}
}

impl fmt::Display for Object {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&GstObjectExt::name(self))
    }
}

unsafe impl Send for Object {}
unsafe impl Sync for Object {}

pub const NONE_OBJECT: Option<&Object> = None;

pub trait GstObjectExt: 'static {
    #[doc(alias = "gst_object_add_control_binding")]
    fn add_control_binding(
        &self,
        binding: &impl IsA<ControlBinding>,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_object_default_error")]
    fn default_error(&self, error: &glib::Error, debug: Option<&str>);

    #[doc(alias = "gst_object_get_control_binding")]
    #[doc(alias = "get_control_binding")]
    fn control_binding(&self, property_name: &str) -> Option<ControlBinding>;

    #[doc(alias = "gst_object_get_control_rate")]
    #[doc(alias = "get_control_rate")]
    fn control_rate(&self) -> Option<ClockTime>;

    #[doc(alias = "gst_object_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> glib::GString;

    #[doc(alias = "gst_object_get_parent")]
    #[doc(alias = "get_parent")]
    fn parent(&self) -> Option<Object>;

    #[doc(alias = "gst_object_get_path_string")]
    #[doc(alias = "get_path_string")]
    fn path_string(&self) -> glib::GString;

    #[doc(alias = "gst_object_get_value")]
    #[doc(alias = "get_value")]
    fn value(
        &self,
        property_name: &str,
        timestamp: impl Into<Option<ClockTime>>,
    ) -> Option<glib::Value>;

    //#[doc(alias = "gst_object_get_value_array")]
    //#[doc(alias = "get_value_array")]
    //fn is_value_array(&self, property_name: &str, timestamp: impl Into<Option<ClockTime>>, interval: impl Into<Option<ClockTime>>, n_values: u32, values: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool;

    #[doc(alias = "gst_object_has_active_control_bindings")]
    fn has_active_control_bindings(&self) -> bool;

    #[doc(alias = "gst_object_has_ancestor")]
    fn has_ancestor(&self, ancestor: &impl IsA<Object>) -> bool;

    #[doc(alias = "gst_object_has_as_ancestor")]
    fn has_as_ancestor(&self, ancestor: &impl IsA<Object>) -> bool;

    #[doc(alias = "gst_object_has_as_parent")]
    fn has_as_parent(&self, parent: &impl IsA<Object>) -> bool;

    #[doc(alias = "gst_object_remove_control_binding")]
    fn remove_control_binding(&self, binding: &impl IsA<ControlBinding>) -> bool;

    #[doc(alias = "gst_object_set_control_binding_disabled")]
    fn set_control_binding_disabled(&self, property_name: &str, disabled: bool);

    #[doc(alias = "gst_object_set_control_bindings_disabled")]
    fn set_control_bindings_disabled(&self, disabled: bool);

    #[doc(alias = "gst_object_set_control_rate")]
    fn set_control_rate(&self, control_rate: impl Into<Option<ClockTime>>);

    #[doc(alias = "gst_object_set_parent")]
    fn set_parent(&self, parent: &impl IsA<Object>) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_object_suggest_next_sync")]
    fn suggest_next_sync(&self) -> Option<ClockTime>;

    #[doc(alias = "gst_object_sync_values")]
    fn sync_values(&self, timestamp: ClockTime) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_object_unparent")]
    fn unparent(&self);

    //#[doc(alias = "deep-notify")]
    //fn connect_deep_notify<Unsupported or ignored types>(&self, detail: Option<&str>, f: F) -> SignalHandlerId;

    #[doc(alias = "parent")]
    fn connect_parent_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Object>> GstObjectExt for O {
    fn add_control_binding(
        &self,
        binding: &impl IsA<ControlBinding>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_object_add_control_binding(
                    self.as_ref().to_glib_none().0,
                    binding.as_ref().to_glib_none().0
                ),
                "Failed to add control binding"
            )
        }
    }

    fn default_error(&self, error: &glib::Error, debug: Option<&str>) {
        unsafe {
            ffi::gst_object_default_error(
                self.as_ref().to_glib_none().0,
                error.to_glib_none().0,
                debug.to_glib_none().0,
            );
        }
    }

    fn control_binding(&self, property_name: &str) -> Option<ControlBinding> {
        unsafe {
            from_glib_full(ffi::gst_object_get_control_binding(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            ))
        }
    }

    fn control_rate(&self) -> Option<ClockTime> {
        unsafe {
            from_glib(ffi::gst_object_get_control_rate(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn name(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::gst_object_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn parent(&self) -> Option<Object> {
        unsafe { from_glib_full(ffi::gst_object_get_parent(self.as_ref().to_glib_none().0)) }
    }

    fn path_string(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gst_object_get_path_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn value(
        &self,
        property_name: &str,
        timestamp: impl Into<Option<ClockTime>>,
    ) -> Option<glib::Value> {
        unsafe {
            from_glib_full(ffi::gst_object_get_value(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                timestamp.into().into_glib(),
            ))
        }
    }

    //fn is_value_array(&self, property_name: &str, timestamp: impl Into<Option<ClockTime>>, interval: impl Into<Option<ClockTime>>, n_values: u32, values: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call ffi:gst_object_get_value_array() }
    //}

    fn has_active_control_bindings(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_object_has_active_control_bindings(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_ancestor(&self, ancestor: &impl IsA<Object>) -> bool {
        unsafe {
            from_glib(ffi::gst_object_has_ancestor(
                self.as_ref().to_glib_none().0,
                ancestor.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_as_ancestor(&self, ancestor: &impl IsA<Object>) -> bool {
        unsafe {
            from_glib(ffi::gst_object_has_as_ancestor(
                self.as_ref().to_glib_none().0,
                ancestor.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_as_parent(&self, parent: &impl IsA<Object>) -> bool {
        unsafe {
            from_glib(ffi::gst_object_has_as_parent(
                self.as_ref().to_glib_none().0,
                parent.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_control_binding(&self, binding: &impl IsA<ControlBinding>) -> bool {
        unsafe {
            from_glib(ffi::gst_object_remove_control_binding(
                self.as_ref().to_glib_none().0,
                binding.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_control_binding_disabled(&self, property_name: &str, disabled: bool) {
        unsafe {
            ffi::gst_object_set_control_binding_disabled(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                disabled.into_glib(),
            );
        }
    }

    fn set_control_bindings_disabled(&self, disabled: bool) {
        unsafe {
            ffi::gst_object_set_control_bindings_disabled(
                self.as_ref().to_glib_none().0,
                disabled.into_glib(),
            );
        }
    }

    fn set_control_rate(&self, control_rate: impl Into<Option<ClockTime>>) {
        unsafe {
            ffi::gst_object_set_control_rate(
                self.as_ref().to_glib_none().0,
                control_rate.into().into_glib(),
            );
        }
    }

    fn set_parent(&self, parent: &impl IsA<Object>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_object_set_parent(
                    self.as_ref().to_glib_none().0,
                    parent.as_ref().to_glib_none().0
                ),
                "Failed to set parent object"
            )
        }
    }

    fn suggest_next_sync(&self) -> Option<ClockTime> {
        unsafe {
            from_glib(ffi::gst_object_suggest_next_sync(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn sync_values(&self, timestamp: ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_object_sync_values(self.as_ref().to_glib_none().0, timestamp.into_glib()),
                "Failed to sync values"
            )
        }
    }

    fn unparent(&self) {
        unsafe {
            ffi::gst_object_unparent(self.as_ref().to_glib_none().0);
        }
    }

    //fn connect_deep_notify<Unsupported or ignored types>(&self, detail: Option<&str>, f: F) -> SignalHandlerId {
    //    Ignored prop: GObject.ParamSpec
    //}

    fn connect_parent_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
