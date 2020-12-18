// This file was generated by gir (https://github.com/gtk-rs/gir @ 00040a2)
// from gir-files (https://github.com/gtk-rs/gir-files @ 8ff1a78d)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

use glib_sys as glib;
use gstreamer_gl_sys as gst_gl;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, time_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Records
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstGLDisplayX11Class {
    pub object_class: gst_gl::GstGLDisplayClass,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstGLDisplayX11Class {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstGLDisplayX11Class @ {:?}", self as *const _))
            .field("object_class", &self.object_class)
            .field("_padding", &self._padding)
            .finish()
    }
}

// Classes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstGLDisplayX11 {
    pub parent: gst_gl::GstGLDisplay,
    pub name: *mut c_char,
    pub display: gpointer,
    pub xcb_connection: gpointer,
    pub foreign_display: gboolean,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstGLDisplayX11 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstGLDisplayX11 @ {:?}", self as *const _))
            .finish()
    }
}

#[link(name = "gstgl-1.0")]
extern "C" {

    //=========================================================================
    // GstGLDisplayX11
    //=========================================================================
    pub fn gst_gl_display_x11_get_type() -> GType;
    pub fn gst_gl_display_x11_new(name: *const c_char) -> *mut GstGLDisplayX11;
    pub fn gst_gl_display_x11_new_with_display(display: gpointer) -> *mut GstGLDisplayX11;

}
