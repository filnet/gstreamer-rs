// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::GLContext;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use crate::GLSLProfile;
use crate::GLSLStage;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use crate::GLSLVersion;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    pub struct GLShader(Object<ffi::GstGLShader, ffi::GstGLShaderClass>) @extends gst::Object;

    match fn {
        type_ => || ffi::gst_gl_shader_get_type(),
    }
}

impl GLShader {
    #[doc(alias = "gst_gl_shader_new")]
    pub fn new<P: IsA<GLContext>>(context: &P) -> GLShader {
        skip_assert_initialized!();
        unsafe { from_glib_full(ffi::gst_gl_shader_new(context.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gst_gl_shader_new_default")]
    pub fn new_default<P: IsA<GLContext>>(context: &P) -> Result<GLShader, glib::Error> {
        skip_assert_initialized!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_gl_shader_new_default(context.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //#[doc(alias = "gst_gl_shader_new_link_with_stages")]
    //pub fn new_link_with_stages<P: IsA<GLContext>>(context: &P, error: &mut glib::Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> GLShader {
    //    unsafe { TODO: call ffi:gst_gl_shader_new_link_with_stages() }
    //}

    //#[doc(alias = "gst_gl_shader_new_with_stages")]
    //pub fn with_stages<P: IsA<GLContext>>(context: &P, error: &mut glib::Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> GLShader {
    //    unsafe { TODO: call ffi:gst_gl_shader_new_with_stages() }
    //}

    #[doc(alias = "gst_gl_shader_attach")]
    pub fn attach(&self, stage: &GLSLStage) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_gl_shader_attach(self.to_glib_none().0, stage.to_glib_none().0),
                "Failed to attach stage to shader"
            )
        }
    }

    #[doc(alias = "gst_gl_shader_attach_unlocked")]
    pub fn attach_unlocked(&self, stage: &GLSLStage) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_gl_shader_attach_unlocked(self.to_glib_none().0, stage.to_glib_none().0),
                "Failed to attach stage to shader"
            )
        }
    }

    #[doc(alias = "gst_gl_shader_bind_attribute_location")]
    pub fn bind_attribute_location(&self, index: u32, name: &str) {
        unsafe {
            ffi::gst_gl_shader_bind_attribute_location(
                self.to_glib_none().0,
                index,
                name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_gl_shader_bind_frag_data_location")]
    pub fn bind_frag_data_location(&self, index: u32, name: &str) {
        unsafe {
            ffi::gst_gl_shader_bind_frag_data_location(
                self.to_glib_none().0,
                index,
                name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_gl_shader_compile_attach_stage")]
    pub fn compile_attach_stage(&self, stage: &GLSLStage) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gst_gl_shader_compile_attach_stage(
                self.to_glib_none().0,
                stage.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gst_gl_shader_detach")]
    pub fn detach(&self, stage: &GLSLStage) {
        unsafe {
            ffi::gst_gl_shader_detach(self.to_glib_none().0, stage.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_gl_shader_detach_unlocked")]
    pub fn detach_unlocked(&self, stage: &GLSLStage) {
        unsafe {
            ffi::gst_gl_shader_detach_unlocked(self.to_glib_none().0, stage.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_gl_shader_get_attribute_location")]
    pub fn attribute_location(&self, name: &str) -> i32 {
        unsafe {
            ffi::gst_gl_shader_get_attribute_location(self.to_glib_none().0, name.to_glib_none().0)
        }
    }

    #[doc(alias = "gst_gl_shader_get_program_handle")]
    pub fn program_handle(&self) -> i32 {
        unsafe { ffi::gst_gl_shader_get_program_handle(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_gl_shader_is_linked")]
    pub fn is_linked(&self) -> bool {
        unsafe { from_glib(ffi::gst_gl_shader_is_linked(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_gl_shader_link")]
    pub fn link(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gst_gl_shader_link(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gst_gl_shader_release")]
    pub fn release(&self) {
        unsafe {
            ffi::gst_gl_shader_release(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_gl_shader_release_unlocked")]
    pub fn release_unlocked(&self) {
        unsafe {
            ffi::gst_gl_shader_release_unlocked(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_gl_shader_set_uniform_1f")]
    pub fn set_uniform_1f(&self, name: &str, value: f32) {
        unsafe {
            ffi::gst_gl_shader_set_uniform_1f(self.to_glib_none().0, name.to_glib_none().0, value);
        }
    }

    #[doc(alias = "gst_gl_shader_set_uniform_1fv")]
    pub fn set_uniform_1fv(&self, name: &str, value: &[f32]) {
        let count = value.len() as u32;
        unsafe {
            ffi::gst_gl_shader_set_uniform_1fv(
                self.to_glib_none().0,
                name.to_glib_none().0,
                count,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_gl_shader_set_uniform_1i")]
    pub fn set_uniform_1i(&self, name: &str, value: i32) {
        unsafe {
            ffi::gst_gl_shader_set_uniform_1i(self.to_glib_none().0, name.to_glib_none().0, value);
        }
    }

    #[doc(alias = "gst_gl_shader_set_uniform_1iv")]
    pub fn set_uniform_1iv(&self, name: &str, value: &[i32]) {
        let count = value.len() as u32;
        unsafe {
            ffi::gst_gl_shader_set_uniform_1iv(
                self.to_glib_none().0,
                name.to_glib_none().0,
                count,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_gl_shader_set_uniform_2f")]
    pub fn set_uniform_2f(&self, name: &str, v0: f32, v1: f32) {
        unsafe {
            ffi::gst_gl_shader_set_uniform_2f(self.to_glib_none().0, name.to_glib_none().0, v0, v1);
        }
    }

    #[doc(alias = "gst_gl_shader_set_uniform_2fv")]
    pub fn set_uniform_2fv(&self, name: &str, value: &[f32]) {
        let count = value.len() as u32;
        unsafe {
            ffi::gst_gl_shader_set_uniform_2fv(
                self.to_glib_none().0,
                name.to_glib_none().0,
                count,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_gl_shader_set_uniform_2i")]
    pub fn set_uniform_2i(&self, name: &str, v0: i32, v1: i32) {
        unsafe {
            ffi::gst_gl_shader_set_uniform_2i(self.to_glib_none().0, name.to_glib_none().0, v0, v1);
        }
    }

    #[doc(alias = "gst_gl_shader_set_uniform_2iv")]
    pub fn set_uniform_2iv(&self, name: &str, value: &[i32]) {
        let count = value.len() as u32;
        unsafe {
            ffi::gst_gl_shader_set_uniform_2iv(
                self.to_glib_none().0,
                name.to_glib_none().0,
                count,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_gl_shader_set_uniform_3f")]
    pub fn set_uniform_3f(&self, name: &str, v0: f32, v1: f32, v2: f32) {
        unsafe {
            ffi::gst_gl_shader_set_uniform_3f(
                self.to_glib_none().0,
                name.to_glib_none().0,
                v0,
                v1,
                v2,
            );
        }
    }

    #[doc(alias = "gst_gl_shader_set_uniform_3fv")]
    pub fn set_uniform_3fv(&self, name: &str, value: &[f32]) {
        let count = value.len() as u32;
        unsafe {
            ffi::gst_gl_shader_set_uniform_3fv(
                self.to_glib_none().0,
                name.to_glib_none().0,
                count,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_gl_shader_set_uniform_3i")]
    pub fn set_uniform_3i(&self, name: &str, v0: i32, v1: i32, v2: i32) {
        unsafe {
            ffi::gst_gl_shader_set_uniform_3i(
                self.to_glib_none().0,
                name.to_glib_none().0,
                v0,
                v1,
                v2,
            );
        }
    }

    #[doc(alias = "gst_gl_shader_set_uniform_3iv")]
    pub fn set_uniform_3iv(&self, name: &str, value: &[i32]) {
        let count = value.len() as u32;
        unsafe {
            ffi::gst_gl_shader_set_uniform_3iv(
                self.to_glib_none().0,
                name.to_glib_none().0,
                count,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_gl_shader_set_uniform_4f")]
    pub fn set_uniform_4f(&self, name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        unsafe {
            ffi::gst_gl_shader_set_uniform_4f(
                self.to_glib_none().0,
                name.to_glib_none().0,
                v0,
                v1,
                v2,
                v3,
            );
        }
    }

    #[doc(alias = "gst_gl_shader_set_uniform_4fv")]
    pub fn set_uniform_4fv(&self, name: &str, value: &[f32]) {
        let count = value.len() as u32;
        unsafe {
            ffi::gst_gl_shader_set_uniform_4fv(
                self.to_glib_none().0,
                name.to_glib_none().0,
                count,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_gl_shader_set_uniform_4i")]
    pub fn set_uniform_4i(&self, name: &str, v0: i32, v1: i32, v2: i32, v3: i32) {
        unsafe {
            ffi::gst_gl_shader_set_uniform_4i(
                self.to_glib_none().0,
                name.to_glib_none().0,
                v0,
                v1,
                v2,
                v3,
            );
        }
    }

    #[doc(alias = "gst_gl_shader_set_uniform_4iv")]
    pub fn set_uniform_4iv(&self, name: &str, value: &[i32]) {
        let count = value.len() as u32;
        unsafe {
            ffi::gst_gl_shader_set_uniform_4iv(
                self.to_glib_none().0,
                name.to_glib_none().0,
                count,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_gl_shader_use")]
    pub fn use_(&self) {
        unsafe {
            ffi::gst_gl_shader_use(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_gl_shader_string_fragment_external_oes_get_default")]
    pub fn string_fragment_external_oes_get_default<P: IsA<GLContext>>(
        context: &P,
        version: GLSLVersion,
        profile: GLSLProfile,
    ) -> Option<glib::GString> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_gl_shader_string_fragment_external_oes_get_default(
                context.as_ref().to_glib_none().0,
                version.to_glib(),
                profile.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_gl_shader_string_fragment_get_default")]
    pub fn string_fragment_get_default<P: IsA<GLContext>>(
        context: &P,
        version: GLSLVersion,
        profile: GLSLProfile,
    ) -> Option<glib::GString> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_gl_shader_string_fragment_get_default(
                context.as_ref().to_glib_none().0,
                version.to_glib(),
                profile.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_gl_shader_string_get_highest_precision")]
    pub fn string_get_highest_precision<P: IsA<GLContext>>(
        context: &P,
        version: GLSLVersion,
        profile: GLSLProfile,
    ) -> Option<glib::GString> {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gst_gl_shader_string_get_highest_precision(
                context.as_ref().to_glib_none().0,
                version.to_glib(),
                profile.to_glib(),
            ))
        }
    }

    pub fn connect_property_linked_notify<F: Fn(&GLShader) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_linked_trampoline<F: Fn(&GLShader) + Send + Sync + 'static>(
            this: *mut ffi::GstGLShader,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::linked\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_linked_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for GLShader {}
unsafe impl Sync for GLShader {}
