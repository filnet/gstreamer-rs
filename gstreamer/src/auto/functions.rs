// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Bin;
use crate::ClockTime;
use crate::DebugGraphDetails;
use crate::DebugLevel;
use crate::Element;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use crate::PluginAPIFlags;
#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
use crate::StackTraceFlags;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use crate::Tracer;
use glib::object::IsA;
use glib::translate::*;
use std::mem;
use std::ptr;

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
#[doc(alias = "gst_debug_add_ring_buffer_logger")]
pub fn debug_add_ring_buffer_logger(max_size_per_thread: u32, thread_timeout: u32) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_add_ring_buffer_logger(max_size_per_thread, thread_timeout);
    }
}

#[doc(alias = "gst_debug_bin_to_dot_data")]
pub fn debug_bin_to_dot_data<P: IsA<Bin>>(bin: &P, details: DebugGraphDetails) -> glib::GString {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gst_debug_bin_to_dot_data(
            bin.as_ref().to_glib_none().0,
            details.into_glib(),
        ))
    }
}

#[doc(alias = "gst_debug_bin_to_dot_file")]
pub fn debug_bin_to_dot_file<P: IsA<Bin>, Q: AsRef<std::path::Path>>(
    bin: &P,
    details: DebugGraphDetails,
    file_name: Q,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_bin_to_dot_file(
            bin.as_ref().to_glib_none().0,
            details.into_glib(),
            file_name.as_ref().to_glib_none().0,
        );
    }
}

#[doc(alias = "gst_debug_bin_to_dot_file_with_ts")]
pub fn debug_bin_to_dot_file_with_ts<P: IsA<Bin>, Q: AsRef<std::path::Path>>(
    bin: &P,
    details: DebugGraphDetails,
    file_name: Q,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_bin_to_dot_file_with_ts(
            bin.as_ref().to_glib_none().0,
            details.into_glib(),
            file_name.as_ref().to_glib_none().0,
        );
    }
}

#[doc(alias = "gst_debug_get_default_threshold")]
pub fn debug_get_default_threshold() -> DebugLevel {
    skip_assert_initialized!();
    unsafe { from_glib(ffi::gst_debug_get_default_threshold()) }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
#[doc(alias = "gst_debug_get_stack_trace")]
pub fn debug_get_stack_trace(flags: StackTraceFlags) -> Result<glib::GString, glib::BoolError> {
    skip_assert_initialized!();
    unsafe {
        Option::<_>::from_glib_full(ffi::gst_debug_get_stack_trace(flags.into_glib()))
            .ok_or_else(|| glib::bool_error!("Failed to get stack trace"))
    }
}

#[doc(alias = "gst_debug_is_active")]
pub fn debug_is_active() -> bool {
    skip_assert_initialized!();
    unsafe { from_glib(ffi::gst_debug_is_active()) }
}

#[doc(alias = "gst_debug_is_colored")]
pub fn debug_is_colored() -> bool {
    skip_assert_initialized!();
    unsafe { from_glib(ffi::gst_debug_is_colored()) }
}

#[doc(alias = "gst_debug_print_stack_trace")]
pub fn debug_print_stack_trace() {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_print_stack_trace();
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
#[doc(alias = "gst_debug_remove_ring_buffer_logger")]
pub fn debug_remove_ring_buffer_logger() {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_remove_ring_buffer_logger();
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
#[doc(alias = "gst_debug_ring_buffer_logger_get_logs")]
pub fn debug_ring_buffer_logger_get_logs() -> Vec<glib::GString> {
    skip_assert_initialized!();
    unsafe { FromGlibPtrContainer::from_glib_full(ffi::gst_debug_ring_buffer_logger_get_logs()) }
}

#[doc(alias = "gst_debug_set_active")]
pub fn debug_set_active(active: bool) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_set_active(active.into_glib());
    }
}

#[doc(alias = "gst_debug_set_colored")]
pub fn debug_set_colored(colored: bool) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_set_colored(colored.into_glib());
    }
}

#[doc(alias = "gst_debug_set_default_threshold")]
pub fn debug_set_default_threshold(level: DebugLevel) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_set_default_threshold(level.into_glib());
    }
}

#[doc(alias = "gst_debug_set_threshold_for_name")]
pub fn debug_set_threshold_for_name(name: &str, level: DebugLevel) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_set_threshold_for_name(name.to_glib_none().0, level.into_glib());
    }
}

#[doc(alias = "gst_debug_set_threshold_from_string")]
pub fn debug_set_threshold_from_string(list: &str, reset: bool) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_set_threshold_from_string(list.to_glib_none().0, reset.into_glib());
    }
}

#[doc(alias = "gst_debug_unset_threshold_for_name")]
pub fn debug_unset_threshold_for_name(name: &str) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_unset_threshold_for_name(name.to_glib_none().0);
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
#[doc(alias = "gst_get_main_executable_path")]
#[doc(alias = "get_main_executable_path")]
pub fn main_executable_path() -> Result<glib::GString, glib::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        Option::<_>::from_glib_none(ffi::gst_get_main_executable_path())
            .ok_or_else(|| glib::bool_error!("Failed to get main executable path"))
    }
}

#[doc(alias = "gst_parse_bin_from_description")]
pub fn parse_bin_from_description(
    bin_description: &str,
    ghost_unlinked_pads: bool,
) -> Result<Bin, glib::Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_bin_from_description(
            bin_description.to_glib_none().0,
            ghost_unlinked_pads.into_glib(),
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_none(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "gst_parse_launch")]
pub fn parse_launch(pipeline_description: &str) -> Result<Element, glib::Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_launch(pipeline_description.to_glib_none().0, &mut error);
        if error.is_null() {
            Ok(from_glib_none(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "gst_parse_launchv")]
pub fn parse_launchv(argv: &[&str]) -> Result<Element, glib::Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_launchv(argv.to_glib_none().0, &mut error);
        if error.is_null() {
            Ok(from_glib_none(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
#[doc(alias = "gst_tracing_get_active_tracers")]
pub fn tracing_get_active_tracers() -> Vec<Tracer> {
    assert_initialized_main_thread!();
    unsafe { FromGlibPtrContainer::from_glib_full(ffi::gst_tracing_get_active_tracers()) }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
#[doc(alias = "gst_type_mark_as_plugin_api")]
pub fn type_mark_as_plugin_api(type_: glib::types::Type, flags: PluginAPIFlags) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_type_mark_as_plugin_api(type_.into_glib(), flags.into_glib());
    }
}

#[doc(alias = "gst_update_registry")]
pub fn update_registry() -> Result<(), glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        glib::result_from_gboolean!(ffi::gst_update_registry(), "Failed to update the registry")
    }
}

#[doc(alias = "gst_util_get_timestamp")]
pub fn util_get_timestamp() -> ClockTime {
    skip_assert_initialized!();
    unsafe { try_from_glib(ffi::gst_util_get_timestamp()).expect("mandatory glib value is None") }
}

#[doc(alias = "gst_version")]
pub fn version() -> (u32, u32, u32, u32) {
    skip_assert_initialized!();
    unsafe {
        let mut major = mem::MaybeUninit::uninit();
        let mut minor = mem::MaybeUninit::uninit();
        let mut micro = mem::MaybeUninit::uninit();
        let mut nano = mem::MaybeUninit::uninit();
        ffi::gst_version(
            major.as_mut_ptr(),
            minor.as_mut_ptr(),
            micro.as_mut_ptr(),
            nano.as_mut_ptr(),
        );
        let major = major.assume_init();
        let minor = minor.assume_init();
        let micro = micro.assume_init();
        let nano = nano.assume_init();
        (major, minor, micro, nano)
    }
}

#[doc(alias = "gst_version_string")]
pub fn version_string() -> glib::GString {
    skip_assert_initialized!();
    unsafe { from_glib_full(ffi::gst_version_string()) }
}
