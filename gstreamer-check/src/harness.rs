// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gst;
use gst::prelude::*;
use gst_check_sys;
use std::marker::PhantomData;
use std::mem;
use std::ops;
use std::path;
use std::ptr;
use TestClock;

#[derive(Debug)]
pub struct Harness(
    ptr::NonNull<gst_check_sys::GstHarness>,
    PhantomData<gst_check_sys::GstHarness>,
);

impl Drop for Harness {
    fn drop(&mut self) {
        unsafe {
            gst_check_sys::gst_harness_teardown(self.0.as_ptr());
        }
    }
}

unsafe impl Send for Harness {}
unsafe impl Sync for Harness {}

impl Harness {
    pub fn add_element_full<P: IsA<gst::Element>>(
        &mut self,
        element: &P,
        hsrc: Option<&gst::StaticPadTemplate>,
        element_sinkpad_name: Option<&str>,
        hsink: Option<&gst::StaticPadTemplate>,
        element_srcpad_name: Option<&str>,
    ) {
        let element_sinkpad_name = element_sinkpad_name.to_glib_none();
        let element_srcpad_name = element_srcpad_name.to_glib_none();
        unsafe {
            gst_check_sys::gst_harness_add_element_full(
                self.0.as_ptr(),
                element.as_ref().to_glib_none().0,
                hsrc.to_glib_none().0 as *mut _,
                element_sinkpad_name.0,
                hsink.to_glib_none().0 as *mut _,
                element_srcpad_name.0,
            );
        }
    }

    pub fn add_element_sink_pad<P: IsA<gst::Pad>>(&mut self, sinkpad: &P) {
        unsafe {
            gst_check_sys::gst_harness_add_element_sink_pad(
                self.0.as_ptr(),
                sinkpad.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn add_element_src_pad<P: IsA<gst::Pad>>(&mut self, srcpad: &P) {
        unsafe {
            gst_check_sys::gst_harness_add_element_src_pad(
                self.0.as_ptr(),
                srcpad.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn add_parse(&mut self, launchline: &str) {
        unsafe {
            gst_check_sys::gst_harness_add_parse(self.0.as_ptr(), launchline.to_glib_none().0);
        }
    }

    pub fn add_probe<F>(
        &mut self,
        element_name: &str,
        pad_name: &str,
        mask: gst::PadProbeType,
        func: F,
    ) where
        F: Fn(&gst::Pad, &mut gst::PadProbeInfo) -> gst::PadProbeReturn + Send + Sync + 'static,
    {
        // Reimplementation of the C code so we don't have to duplicate all the callback code

        let element = self.find_element(element_name).expect("Element not found");
        let pad = element.get_static_pad(pad_name).expect("Pad not found");
        pad.add_probe(mask, func);
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn add_propose_allocation_meta(
        &mut self,
        api: glib::types::Type,
        params: Option<&gst::StructureRef>,
    ) {
        unsafe {
            let params = params.map(|p| p.as_ptr()).unwrap_or(ptr::null_mut());
            gst_check_sys::gst_harness_add_propose_allocation_meta(
                self.0.as_ptr(),
                api.to_glib(),
                params,
            );
        }
    }

    pub fn add_sink(&mut self, sink_element_name: &str) {
        unsafe {
            gst_check_sys::gst_harness_add_sink(
                self.0.as_ptr(),
                sink_element_name.to_glib_none().0,
            );
        }
    }

    pub fn add_sink_harness(&mut self, sink_harness: Harness) {
        unsafe {
            let sink_harness = mem::ManuallyDrop::new(sink_harness);
            gst_check_sys::gst_harness_add_sink_harness(self.0.as_ptr(), sink_harness.0.as_ptr());
        }
    }

    pub fn add_sink_parse(&mut self, launchline: &str) {
        unsafe {
            gst_check_sys::gst_harness_add_sink_parse(self.0.as_ptr(), launchline.to_glib_none().0);
        }
    }

    pub fn add_src(&mut self, src_element_name: &str, has_clock_wait: bool) {
        unsafe {
            gst_check_sys::gst_harness_add_src(
                self.0.as_ptr(),
                src_element_name.to_glib_none().0,
                has_clock_wait.to_glib(),
            );
        }
    }

    pub fn add_src_harness(&mut self, src_harness: Harness, has_clock_wait: bool) {
        unsafe {
            let src_harness = mem::ManuallyDrop::new(src_harness);
            gst_check_sys::gst_harness_add_src_harness(
                self.0.as_ptr(),
                src_harness.0.as_ptr(),
                has_clock_wait.to_glib(),
            );
        }
    }

    pub fn add_src_parse(&mut self, launchline: &str, has_clock_wait: bool) {
        unsafe {
            gst_check_sys::gst_harness_add_src_parse(
                self.0.as_ptr(),
                launchline.to_glib_none().0,
                has_clock_wait.to_glib(),
            );
        }
    }

    pub fn buffers_in_queue(&self) -> u32 {
        unsafe { gst_check_sys::gst_harness_buffers_in_queue(self.0.as_ptr()) }
    }

    pub fn buffers_received(&self) -> u32 {
        unsafe { gst_check_sys::gst_harness_buffers_received(self.0.as_ptr()) }
    }

    pub fn crank_multiple_clock_waits(&mut self, waits: u32) -> Result<(), glib::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_check_sys::gst_harness_crank_multiple_clock_waits(self.0.as_ptr(), waits),
                "Failed to crank multiple clock waits",
            )
        }
    }

    pub fn crank_single_clock_wait(&mut self) -> Result<(), glib::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_check_sys::gst_harness_crank_single_clock_wait(self.0.as_ptr()),
                "Failed to crank single clock wait",
            )
        }
    }

    pub fn create_buffer(&mut self, size: usize) -> Result<gst::Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(gst_check_sys::gst_harness_create_buffer(
                self.0.as_ptr(),
                size,
            ))
            .ok_or_else(|| glib_bool_error!("Failed to create new buffer"))
        }
    }

    pub fn dump_to_file<P: AsRef<path::Path>>(&mut self, filename: P) {
        let filename = filename.as_ref();
        unsafe {
            gst_check_sys::gst_harness_dump_to_file(self.0.as_ptr(), filename.to_glib_none().0);
        }
    }

    pub fn events_in_queue(&self) -> u32 {
        unsafe { gst_check_sys::gst_harness_events_in_queue(self.0.as_ptr()) }
    }

    pub fn events_received(&self) -> u32 {
        unsafe { gst_check_sys::gst_harness_events_received(self.0.as_ptr()) }
    }

    pub fn find_element(&mut self, element_name: &str) -> Option<gst::Element> {
        unsafe {
            // Work around https://gitlab.freedesktop.org/gstreamer/gstreamer/merge_requests/31
            let ptr = gst_check_sys::gst_harness_find_element(
                self.0.as_ptr(),
                element_name.to_glib_none().0,
            );

            if ptr.is_null() {
                return None;
            }

            // Clear floating flag if it is set
            if gobject_sys::g_object_is_floating(ptr as *mut _) != glib_sys::GFALSE {
                gobject_sys::g_object_ref_sink(ptr as *mut _);
            }

            from_glib_full(ptr)
        }
    }

    //pub fn get(&mut self, element_name: &str, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gst_check_sys::gst_harness_get() }
    //}

    //pub fn get_allocator(&mut self, allocator: /*Ignored*/gst::Allocator, params: /*Ignored*/gst::AllocationParams) {
    //    unsafe { TODO: call gst_check_sys::gst_harness_get_allocator() }
    //}

    pub fn get_last_pushed_timestamp(&self) -> gst::ClockTime {
        unsafe {
            from_glib(gst_check_sys::gst_harness_get_last_pushed_timestamp(
                self.0.as_ptr(),
            ))
        }
    }

    pub fn get_testclock(&self) -> Option<TestClock> {
        unsafe { from_glib_full(gst_check_sys::gst_harness_get_testclock(self.0.as_ptr())) }
    }

    pub fn play(&mut self) {
        unsafe {
            gst_check_sys::gst_harness_play(self.0.as_ptr());
        }
    }

    pub fn pull(&mut self) -> Result<gst::Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(gst_check_sys::gst_harness_pull(self.0.as_ptr()))
                .ok_or_else(|| glib_bool_error!("Failed to pull buffer"))
        }
    }

    pub fn pull_event(&mut self) -> Result<gst::Event, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(gst_check_sys::gst_harness_pull_event(self.0.as_ptr()))
                .ok_or_else(|| glib_bool_error!("Failed to pull event"))
        }
    }

    pub fn pull_upstream_event(&mut self) -> Result<gst::Event, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(gst_check_sys::gst_harness_pull_upstream_event(
                self.0.as_ptr(),
            ))
            .ok_or_else(|| glib_bool_error!("Failed to pull event"))
        }
    }

    pub fn push(&mut self, buffer: gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_check_sys::gst_harness_push(
                self.0.as_ptr(),
                buffer.into_ptr(),
            ))
        };
        ret.into_result()
    }

    pub fn push_and_pull(&mut self, buffer: gst::Buffer) -> Result<gst::Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(gst_check_sys::gst_harness_push_and_pull(
                self.0.as_ptr(),
                buffer.into_ptr(),
            ))
            .ok_or_else(|| glib_bool_error!("Failed to push and pull buffer"))
        }
    }

    pub fn push_event(&mut self, event: gst::Event) -> bool {
        unsafe {
            from_glib(gst_check_sys::gst_harness_push_event(
                self.0.as_ptr(),
                event.into_ptr(),
            ))
        }
    }

    pub fn push_from_src(&mut self) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn =
            unsafe { from_glib(gst_check_sys::gst_harness_push_from_src(self.0.as_ptr())) };
        ret.into_result()
    }

    pub fn push_to_sink(&mut self) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn =
            unsafe { from_glib(gst_check_sys::gst_harness_push_to_sink(self.0.as_ptr())) };
        ret.into_result()
    }

    pub fn push_upstream_event(&mut self, event: gst::Event) -> bool {
        unsafe {
            from_glib(gst_check_sys::gst_harness_push_upstream_event(
                self.0.as_ptr(),
                event.into_ptr(),
            ))
        }
    }

    pub fn query_latency(&self) -> gst::ClockTime {
        unsafe { from_glib(gst_check_sys::gst_harness_query_latency(self.0.as_ptr())) }
    }

    //pub fn set(&mut self, element_name: &str, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gst_check_sys::gst_harness_set() }
    //}

    pub fn set_blocking_push_mode(&mut self) {
        unsafe {
            gst_check_sys::gst_harness_set_blocking_push_mode(self.0.as_ptr());
        }
    }

    pub fn set_caps(&mut self, in_: gst::Caps, out: gst::Caps) {
        unsafe {
            gst_check_sys::gst_harness_set_caps(self.0.as_ptr(), in_.into_ptr(), out.into_ptr());
        }
    }

    pub fn set_caps_str(&mut self, in_: &str, out: &str) {
        unsafe {
            gst_check_sys::gst_harness_set_caps_str(
                self.0.as_ptr(),
                in_.to_glib_none().0,
                out.to_glib_none().0,
            );
        }
    }

    pub fn set_drop_buffers(&mut self, drop_buffers: bool) {
        unsafe {
            gst_check_sys::gst_harness_set_drop_buffers(self.0.as_ptr(), drop_buffers.to_glib());
        }
    }

    pub fn set_forwarding(&mut self, forwarding: bool) {
        unsafe {
            gst_check_sys::gst_harness_set_forwarding(self.0.as_ptr(), forwarding.to_glib());
        }
    }

    //pub fn set_propose_allocator<P: IsA<gst::Allocator>>(&mut self, allocator: Option<&P>, params: Option<&gst::AllocationParams>) {
    //    unsafe { TODO: call gst_check_sys::gst_harness_set_propose_allocator() }
    //}

    pub fn set_sink_caps(&mut self, caps: gst::Caps) {
        unsafe {
            gst_check_sys::gst_harness_set_sink_caps(self.0.as_ptr(), caps.into_ptr());
        }
    }

    pub fn set_sink_caps_str(&mut self, str: &str) {
        unsafe {
            gst_check_sys::gst_harness_set_sink_caps_str(self.0.as_ptr(), str.to_glib_none().0);
        }
    }

    pub fn set_src_caps(&mut self, caps: gst::Caps) {
        unsafe {
            gst_check_sys::gst_harness_set_src_caps(self.0.as_ptr(), caps.into_ptr());
        }
    }

    pub fn set_src_caps_str(&mut self, str: &str) {
        unsafe {
            gst_check_sys::gst_harness_set_src_caps_str(self.0.as_ptr(), str.to_glib_none().0);
        }
    }

    pub fn set_time(&mut self, time: gst::ClockTime) -> Result<(), glib::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_check_sys::gst_harness_set_time(self.0.as_ptr(), time.to_glib()),
                "Failed to set time",
            )
        }
    }

    pub fn set_upstream_latency(&mut self, latency: gst::ClockTime) {
        unsafe {
            gst_check_sys::gst_harness_set_upstream_latency(self.0.as_ptr(), latency.to_glib());
        }
    }

    pub fn sink_push_many(&mut self, pushes: u32) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_check_sys::gst_harness_sink_push_many(
                self.0.as_ptr(),
                pushes as i32,
            ))
        };
        ret.into_result()
    }

    pub fn src_crank_and_push_many(
        &mut self,
        cranks: u32,
        pushes: u32,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_check_sys::gst_harness_src_crank_and_push_many(
                self.0.as_ptr(),
                cranks as i32,
                pushes as i32,
            ))
        };
        ret.into_result()
    }

    pub fn src_push_event(&mut self) -> bool {
        unsafe { from_glib(gst_check_sys::gst_harness_src_push_event(self.0.as_ptr())) }
    }

    //pub fn stress_custom_start<'a, P: Into<Option<&'a /*Ignored*/glib::Func>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&mut self, init: P, callback: /*Unknown conversion*//*Unimplemented*/Func, data: Q, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call gst_check_sys::gst_harness_stress_custom_start() }
    //}

    //pub fn stress_property_start_full(&mut self, name: &str, value: /*Ignored*/&glib::Value, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call gst_check_sys::gst_harness_stress_property_start_full() }
    //}

    //pub fn stress_push_buffer_start_full(&mut self, caps: &mut gst::Caps, segment: /*Ignored*/&gst::Segment, buf: &mut gst::Buffer, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call gst_check_sys::gst_harness_stress_push_buffer_start_full() }
    //}

    //pub fn stress_push_buffer_with_cb_start_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&mut self, caps: &mut gst::Caps, segment: /*Ignored*/&gst::Segment, func: /*Unknown conversion*//*Unimplemented*/HarnessPrepareBufferFunc, data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call gst_check_sys::gst_harness_stress_push_buffer_with_cb_start_full() }
    //}

    //pub fn stress_push_event_start_full(&mut self, event: &mut gst::Event, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call gst_check_sys::gst_harness_stress_push_event_start_full() }
    //}

    //pub fn stress_push_event_with_cb_start_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&mut self, func: /*Unknown conversion*//*Unimplemented*/HarnessPrepareEventFunc, data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call gst_check_sys::gst_harness_stress_push_event_with_cb_start_full() }
    //}

    //pub fn stress_push_upstream_event_start_full(&mut self, event: &mut gst::Event, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call gst_check_sys::gst_harness_stress_push_upstream_event_start_full() }
    //}

    //pub fn stress_push_upstream_event_with_cb_start_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&mut self, func: /*Unknown conversion*//*Unimplemented*/HarnessPrepareEventFunc, data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call gst_check_sys::gst_harness_stress_push_upstream_event_with_cb_start_full() }
    //}

    //pub fn stress_requestpad_start_full(&mut self, templ: /*Ignored*/&gst::PadTemplate, name: &str, caps: &mut gst::Caps, release: bool, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call gst_check_sys::gst_harness_stress_requestpad_start_full() }
    //}

    //pub fn stress_statechange_start_full(&mut self, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call gst_check_sys::gst_harness_stress_statechange_start_full() }
    //}

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn take_all_data_as_buffer(&mut self) -> Result<gst::Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(gst_check_sys::gst_harness_take_all_data_as_buffer(
                self.0.as_ptr(),
            ))
            .ok_or_else(|| glib_bool_error!("Failed to take all data as buffer"))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn take_all_data_as_bytes(&mut self) -> Result<glib::Bytes, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(gst_check_sys::gst_harness_take_all_data_as_bytes(
                self.0.as_ptr(),
            ))
            .ok_or_else(|| glib_bool_error!("Failed to take all data as bytes"))
        }
    }

    pub fn try_pull(&mut self) -> Option<gst::Buffer> {
        unsafe { from_glib_full(gst_check_sys::gst_harness_try_pull(self.0.as_ptr())) }
    }

    pub fn try_pull_event(&mut self) -> Option<gst::Event> {
        unsafe { from_glib_full(gst_check_sys::gst_harness_try_pull_event(self.0.as_ptr())) }
    }

    pub fn try_pull_upstream_event(&mut self) -> Option<gst::Event> {
        unsafe {
            from_glib_full(gst_check_sys::gst_harness_try_pull_upstream_event(
                self.0.as_ptr(),
            ))
        }
    }

    pub fn upstream_events_in_queue(&self) -> u32 {
        unsafe { gst_check_sys::gst_harness_upstream_events_in_queue(self.0.as_ptr()) }
    }

    pub fn upstream_events_received(&self) -> u32 {
        unsafe { gst_check_sys::gst_harness_upstream_events_received(self.0.as_ptr()) }
    }

    pub fn use_systemclock(&mut self) {
        unsafe {
            gst_check_sys::gst_harness_use_systemclock(self.0.as_ptr());
        }
    }

    pub fn use_testclock(&mut self) {
        unsafe {
            gst_check_sys::gst_harness_use_testclock(self.0.as_ptr());
        }
    }

    pub fn wait_for_clock_id_waits(
        &mut self,
        waits: u32,
        timeout: u32,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_check_sys::gst_harness_wait_for_clock_id_waits(self.0.as_ptr(), waits, timeout),
                "Failed to wait for clock id waits",
            )
        }
    }

    unsafe fn from_glib_full(ptr: *mut gst_check_sys::GstHarness) -> Harness {
        assert!(!ptr.is_null());

        Harness(ptr::NonNull::new_unchecked(ptr), PhantomData)
    }

    pub fn new(element_name: &str) -> Harness {
        assert_initialized_main_thread!();
        unsafe {
            Self::from_glib_full(gst_check_sys::gst_harness_new(
                element_name.to_glib_none().0,
            ))
        }
    }

    pub fn new_empty() -> Harness {
        assert_initialized_main_thread!();
        unsafe { Self::from_glib_full(gst_check_sys::gst_harness_new_empty()) }
    }

    pub fn new_full<P: IsA<gst::Element>>(
        element: &P,
        hsrc: Option<&gst::StaticPadTemplate>,
        element_sinkpad_name: Option<&str>,
        hsink: Option<&gst::StaticPadTemplate>,
        element_srcpad_name: Option<&str>,
    ) -> Harness {
        assert_initialized_main_thread!();
        let element_sinkpad_name = element_sinkpad_name.to_glib_none();
        let element_srcpad_name = element_srcpad_name.to_glib_none();
        unsafe {
            Self::from_glib_full(gst_check_sys::gst_harness_new_full(
                element.as_ref().to_glib_none().0,
                hsrc.to_glib_none().0 as *mut _,
                element_sinkpad_name.0,
                hsink.to_glib_none().0 as *mut _,
                element_srcpad_name.0,
            ))
        }
    }

    pub fn new_parse(launchline: &str) -> Harness {
        assert_initialized_main_thread!();
        unsafe {
            Self::from_glib_full(gst_check_sys::gst_harness_new_parse(
                launchline.to_glib_none().0,
            ))
        }
    }

    pub fn new_with_element<P: IsA<gst::Element>>(
        element: &P,
        element_sinkpad_name: Option<&str>,
        element_srcpad_name: Option<&str>,
    ) -> Harness {
        assert_initialized_main_thread!();
        let element_sinkpad_name = element_sinkpad_name.to_glib_none();
        let element_srcpad_name = element_srcpad_name.to_glib_none();
        unsafe {
            Self::from_glib_full(gst_check_sys::gst_harness_new_with_element(
                element.as_ref().to_glib_none().0,
                element_sinkpad_name.0,
                element_srcpad_name.0,
            ))
        }
    }

    pub fn new_with_padnames(
        element_name: &str,
        element_sinkpad_name: Option<&str>,
        element_srcpad_name: Option<&str>,
    ) -> Harness {
        assert_initialized_main_thread!();
        let element_sinkpad_name = element_sinkpad_name.to_glib_none();
        let element_srcpad_name = element_srcpad_name.to_glib_none();
        unsafe {
            Self::from_glib_full(gst_check_sys::gst_harness_new_with_padnames(
                element_name.to_glib_none().0,
                element_sinkpad_name.0,
                element_srcpad_name.0,
            ))
        }
    }

    pub fn new_with_templates(
        element_name: &str,
        hsrc: Option<&gst::StaticPadTemplate>,
        hsink: Option<&gst::StaticPadTemplate>,
    ) -> Harness {
        assert_initialized_main_thread!();
        unsafe {
            Self::from_glib_full(gst_check_sys::gst_harness_new_with_templates(
                element_name.to_glib_none().0,
                hsrc.to_glib_none().0 as *mut _,
                hsink.to_glib_none().0 as *mut _,
            ))
        }
    }

    //pub fn stress_thread_stop(t: /*Ignored*/&mut HarnessThread) -> u32 {
    //    unsafe { TODO: call gst_check_sys::gst_harness_stress_thread_stop() }
    //}

    pub fn get_element(&self) -> Option<gst::Element> {
        unsafe {
            // Work around https://gitlab.freedesktop.org/gstreamer/gstreamer/merge_requests/31
            let ptr = (*self.0.as_ptr()).element;

            if ptr.is_null() {
                return None;
            }

            // Clear floating flag if it is set
            if gobject_sys::g_object_is_floating(ptr as *mut _) != glib_sys::GFALSE {
                gobject_sys::g_object_ref_sink(ptr as *mut _);
            }

            from_glib_none(ptr)
        }
    }

    pub fn get_sinkpad(&self) -> Option<gst::Pad> {
        unsafe {
            // Work around https://gitlab.freedesktop.org/gstreamer/gstreamer/merge_requests/31
            let ptr = (*self.0.as_ptr()).sinkpad;

            if ptr.is_null() {
                return None;
            }

            // Clear floating flag if it is set
            if gobject_sys::g_object_is_floating(ptr as *mut _) != glib_sys::GFALSE {
                gobject_sys::g_object_ref_sink(ptr as *mut _);
            }

            from_glib_none(ptr)
        }
    }

    pub fn get_srcpad(&self) -> Option<gst::Pad> {
        unsafe {
            // Work around https://gitlab.freedesktop.org/gstreamer/gstreamer/merge_requests/31
            let ptr = (*self.0.as_ptr()).srcpad;

            if ptr.is_null() {
                return None;
            }

            // Clear floating flag if it is set
            if gobject_sys::g_object_is_floating(ptr as *mut _) != glib_sys::GFALSE {
                gobject_sys::g_object_ref_sink(ptr as *mut _);
            }

            from_glib_none(ptr)
        }
    }

    pub fn get_sink_harness(&self) -> Option<Ref> {
        unsafe {
            let sink_harness = (*self.0.as_ptr()).sink_harness;
            if sink_harness.is_null() {
                None
            } else {
                Some(Ref(
                    mem::ManuallyDrop::new(Harness(
                        ptr::NonNull::new_unchecked(sink_harness),
                        PhantomData,
                    )),
                    PhantomData,
                ))
            }
        }
    }

    pub fn get_src_harness(&self) -> Option<Ref> {
        unsafe {
            let src_harness = (*self.0.as_ptr()).src_harness;
            if src_harness.is_null() {
                None
            } else {
                Some(Ref(
                    mem::ManuallyDrop::new(Harness(
                        ptr::NonNull::new_unchecked(src_harness),
                        PhantomData,
                    )),
                    PhantomData,
                ))
            }
        }
    }

    pub fn get_mut_sink_harness(&mut self) -> Option<RefMut> {
        unsafe {
            let sink_harness = (*self.0.as_ptr()).sink_harness;
            if sink_harness.is_null() {
                None
            } else {
                Some(RefMut(
                    mem::ManuallyDrop::new(Harness(
                        ptr::NonNull::new_unchecked(sink_harness),
                        PhantomData,
                    )),
                    PhantomData,
                ))
            }
        }
    }

    pub fn get_mut_src_harness(&mut self) -> Option<RefMut> {
        unsafe {
            let src_harness = (*self.0.as_ptr()).src_harness;
            if src_harness.is_null() {
                None
            } else {
                Some(RefMut(
                    mem::ManuallyDrop::new(Harness(
                        ptr::NonNull::new_unchecked(src_harness),
                        PhantomData,
                    )),
                    PhantomData,
                ))
            }
        }
    }
}

#[derive(Debug)]
pub struct Ref<'a>(mem::ManuallyDrop<Harness>, PhantomData<&'a Harness>);

impl<'a> ops::Deref for Ref<'a> {
    type Target = Harness;

    fn deref(&self) -> &Harness {
        &*self.0
    }
}

#[derive(Debug)]
pub struct RefMut<'a>(mem::ManuallyDrop<Harness>, PhantomData<&'a mut Harness>);

impl<'a> ops::Deref for RefMut<'a> {
    type Target = Harness;

    fn deref(&self) -> &Harness {
        &*self.0
    }
}

impl<'a> ops::DerefMut for RefMut<'a> {
    fn deref_mut(&mut self) -> &mut Harness {
        &mut *self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_push_pull() {
        gst::init().unwrap();

        let mut h = Harness::new("identity");
        h.set_src_caps_str("application/test");
        let buf = gst::Buffer::new();
        let buf = h.push_and_pull(buf);
        assert!(buf.is_ok());
    }
}
