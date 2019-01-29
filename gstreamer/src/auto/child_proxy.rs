// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct ChildProxy(Interface<ffi::GstChildProxy>);

    match fn {
        get_type => || ffi::gst_child_proxy_get_type(),
    }
}

unsafe impl Send for ChildProxy {}
unsafe impl Sync for ChildProxy {}

pub const NONE_CHILD_PROXY: Option<&ChildProxy> = None;

pub trait ChildProxyExt: 'static {
    fn child_added<P: IsA<glib::Object>>(&self, child: &P, name: &str);

    fn child_removed<P: IsA<glib::Object>>(&self, child: &P, name: &str);

    //fn get(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn get_child_by_index(&self, index: u32) -> Option<glib::Object>;

    fn get_child_by_name(&self, name: &str) -> Option<glib::Object>;

    fn get_children_count(&self) -> u32;

    //fn get_property(&self, name: &str, value: /*Ignored*/glib::Value);

    //fn get_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    //fn lookup(&self, name: &str, pspec: /*Ignored*/glib::ParamSpec) -> Option<glib::Object>;

    //fn set(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn set_property(&self, name: &str, value: /*Ignored*/&glib::Value);

    //fn set_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    fn connect_child_added<F: Fn(&Self, &glib::Object, &str) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_child_removed<F: Fn(&Self, &glib::Object, &str) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ChildProxy>> ChildProxyExt for O {
    fn child_added<P: IsA<glib::Object>>(&self, child: &P, name: &str) {
        unsafe {
            ffi::gst_child_proxy_child_added(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn child_removed<P: IsA<glib::Object>>(&self, child: &P, name: &str) {
        unsafe {
            ffi::gst_child_proxy_child_removed(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }

    //fn get(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gst_child_proxy_get() }
    //}

    fn get_child_by_index(&self, index: u32) -> Option<glib::Object> {
        unsafe {
            from_glib_full(ffi::gst_child_proxy_get_child_by_index(self.as_ref().to_glib_none().0, index))
        }
    }

    fn get_child_by_name(&self, name: &str) -> Option<glib::Object> {
        unsafe {
            from_glib_full(ffi::gst_child_proxy_get_child_by_name(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_children_count(&self) -> u32 {
        unsafe {
            ffi::gst_child_proxy_get_children_count(self.as_ref().to_glib_none().0)
        }
    }

    //fn get_property(&self, name: &str, value: /*Ignored*/glib::Value) {
    //    unsafe { TODO: call ffi::gst_child_proxy_get_property() }
    //}

    //fn get_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gst_child_proxy_get_valist() }
    //}

    //fn lookup(&self, name: &str, pspec: /*Ignored*/glib::ParamSpec) -> Option<glib::Object> {
    //    unsafe { TODO: call ffi::gst_child_proxy_lookup() }
    //}

    //fn set(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gst_child_proxy_set() }
    //}

    //fn set_property(&self, name: &str, value: /*Ignored*/&glib::Value) {
    //    unsafe { TODO: call ffi::gst_child_proxy_set_property() }
    //}

    //fn set_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gst_child_proxy_set_valist() }
    //}

    fn connect_child_added<F: Fn(&Self, &glib::Object, &str) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"child-added\0".as_ptr() as *const _,
                Some(transmute(child_added_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_child_removed<F: Fn(&Self, &glib::Object, &str) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"child-removed\0".as_ptr() as *const _,
                Some(transmute(child_removed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn child_added_trampoline<P, F: Fn(&P, &glib::Object, &str) + Send + Sync + 'static>(this: *mut ffi::GstChildProxy, object: *mut gobject_ffi::GObject, name: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<ChildProxy> {
    let f: &F = transmute(f);
    f(&ChildProxy::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(object), &GString::from_glib_borrow(name))
}

unsafe extern "C" fn child_removed_trampoline<P, F: Fn(&P, &glib::Object, &str) + Send + Sync + 'static>(this: *mut ffi::GstChildProxy, object: *mut gobject_ffi::GObject, name: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<ChildProxy> {
    let f: &F = transmute(f);
    f(&ChildProxy::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(object), &GString::from_glib_borrow(name))
}
