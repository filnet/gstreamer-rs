// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::PipelineFlags;
use crate::Timeline;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GESPipeline")]
    pub struct Pipeline(Object<ffi::GESPipeline, ffi::GESPipelineClass>) @extends gst::Pipeline, gst::Bin, gst::Element, gst::Object, @implements gst::ChildProxy;

    match fn {
        type_ => || ffi::ges_pipeline_get_type(),
    }
}

impl Pipeline {
    #[doc(alias = "ges_pipeline_new")]
    pub fn new() -> Pipeline {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_pipeline_new()) }
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Pipeline {
    pub const NONE: Option<&'static Pipeline> = None;
}

pub trait GESPipelineExt: 'static {
    #[doc(alias = "ges_pipeline_get_mode")]
    #[doc(alias = "get_mode")]
    fn mode(&self) -> PipelineFlags;

    #[doc(alias = "ges_pipeline_get_thumbnail")]
    #[doc(alias = "get_thumbnail")]
    fn thumbnail(&self, caps: &gst::Caps) -> Option<gst::Sample>;

    #[doc(alias = "ges_pipeline_get_thumbnail_rgb24")]
    #[doc(alias = "get_thumbnail_rgb24")]
    fn thumbnail_rgb24(&self, width: i32, height: i32) -> Option<gst::Sample>;

    #[doc(alias = "ges_pipeline_preview_get_audio_sink")]
    fn preview_get_audio_sink(&self) -> Option<gst::Element>;

    #[doc(alias = "ges_pipeline_preview_get_video_sink")]
    fn preview_get_video_sink(&self) -> Option<gst::Element>;

    #[doc(alias = "ges_pipeline_preview_set_audio_sink")]
    fn preview_set_audio_sink(&self, sink: &impl IsA<gst::Element>);

    #[doc(alias = "ges_pipeline_preview_set_video_sink")]
    fn preview_set_video_sink(&self, sink: &impl IsA<gst::Element>);

    #[doc(alias = "ges_pipeline_save_thumbnail")]
    fn save_thumbnail(
        &self,
        width: i32,
        height: i32,
        format: &str,
        location: &str,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "ges_pipeline_set_mode")]
    fn set_mode(&self, mode: PipelineFlags) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "ges_pipeline_set_render_settings")]
    fn set_render_settings(
        &self,
        output_uri: &str,
        profile: &impl IsA<gst_pbutils::EncodingProfile>,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "ges_pipeline_set_timeline")]
    fn set_timeline(&self, timeline: &impl IsA<Timeline>) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "audio-filter")]
    fn audio_filter(&self) -> Option<gst::Element>;

    #[doc(alias = "audio-filter")]
    fn set_audio_filter<P: IsA<gst::Element>>(&self, audio_filter: Option<&P>);

    #[doc(alias = "audio-sink")]
    fn audio_sink(&self) -> Option<gst::Element>;

    #[doc(alias = "audio-sink")]
    fn set_audio_sink<P: IsA<gst::Element>>(&self, audio_sink: Option<&P>);

    fn timeline(&self) -> Option<Timeline>;

    #[doc(alias = "video-filter")]
    fn video_filter(&self) -> Option<gst::Element>;

    #[doc(alias = "video-filter")]
    fn set_video_filter<P: IsA<gst::Element>>(&self, video_filter: Option<&P>);

    #[doc(alias = "video-sink")]
    fn video_sink(&self) -> Option<gst::Element>;

    #[doc(alias = "video-sink")]
    fn set_video_sink<P: IsA<gst::Element>>(&self, video_sink: Option<&P>);

    #[doc(alias = "audio-filter")]
    fn connect_audio_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "audio-sink")]
    fn connect_audio_sink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "mode")]
    fn connect_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "timeline")]
    fn connect_timeline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "video-filter")]
    fn connect_video_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "video-sink")]
    fn connect_video_sink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Pipeline>> GESPipelineExt for O {
    fn mode(&self) -> PipelineFlags {
        unsafe { from_glib(ffi::ges_pipeline_get_mode(self.as_ref().to_glib_none().0)) }
    }

    fn thumbnail(&self, caps: &gst::Caps) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(ffi::ges_pipeline_get_thumbnail(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
            ))
        }
    }

    fn thumbnail_rgb24(&self, width: i32, height: i32) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(ffi::ges_pipeline_get_thumbnail_rgb24(
                self.as_ref().to_glib_none().0,
                width,
                height,
            ))
        }
    }

    fn preview_get_audio_sink(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::ges_pipeline_preview_get_audio_sink(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn preview_get_video_sink(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::ges_pipeline_preview_get_video_sink(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn preview_set_audio_sink(&self, sink: &impl IsA<gst::Element>) {
        unsafe {
            ffi::ges_pipeline_preview_set_audio_sink(
                self.as_ref().to_glib_none().0,
                sink.as_ref().to_glib_none().0,
            );
        }
    }

    fn preview_set_video_sink(&self, sink: &impl IsA<gst::Element>) {
        unsafe {
            ffi::ges_pipeline_preview_set_video_sink(
                self.as_ref().to_glib_none().0,
                sink.as_ref().to_glib_none().0,
            );
        }
    }

    fn save_thumbnail(
        &self,
        width: i32,
        height: i32,
        format: &str,
        location: &str,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ges_pipeline_save_thumbnail(
                self.as_ref().to_glib_none().0,
                width,
                height,
                format.to_glib_none().0,
                location.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_mode(&self, mode: PipelineFlags) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_pipeline_set_mode(self.as_ref().to_glib_none().0, mode.into_glib()),
                "Failed to set mode"
            )
        }
    }

    fn set_render_settings(
        &self,
        output_uri: &str,
        profile: &impl IsA<gst_pbutils::EncodingProfile>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_pipeline_set_render_settings(
                    self.as_ref().to_glib_none().0,
                    output_uri.to_glib_none().0,
                    profile.as_ref().to_glib_none().0
                ),
                "Failed to set render settings"
            )
        }
    }

    fn set_timeline(&self, timeline: &impl IsA<Timeline>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_pipeline_set_timeline(
                    self.as_ref().to_glib_none().0,
                    timeline.as_ref().to_glib_full()
                ),
                "Failed to set timeline"
            )
        }
    }

    fn audio_filter(&self) -> Option<gst::Element> {
        glib::ObjectExt::property(self.as_ref(), "audio-filter")
    }

    fn set_audio_filter<P: IsA<gst::Element>>(&self, audio_filter: Option<&P>) {
        glib::ObjectExt::set_property(self.as_ref(), "audio-filter", &audio_filter)
    }

    fn audio_sink(&self) -> Option<gst::Element> {
        glib::ObjectExt::property(self.as_ref(), "audio-sink")
    }

    fn set_audio_sink<P: IsA<gst::Element>>(&self, audio_sink: Option<&P>) {
        glib::ObjectExt::set_property(self.as_ref(), "audio-sink", &audio_sink)
    }

    fn timeline(&self) -> Option<Timeline> {
        glib::ObjectExt::property(self.as_ref(), "timeline")
    }

    fn video_filter(&self) -> Option<gst::Element> {
        glib::ObjectExt::property(self.as_ref(), "video-filter")
    }

    fn set_video_filter<P: IsA<gst::Element>>(&self, video_filter: Option<&P>) {
        glib::ObjectExt::set_property(self.as_ref(), "video-filter", &video_filter)
    }

    fn video_sink(&self) -> Option<gst::Element> {
        glib::ObjectExt::property(self.as_ref(), "video-sink")
    }

    fn set_video_sink<P: IsA<gst::Element>>(&self, video_sink: Option<&P>) {
        glib::ObjectExt::set_property(self.as_ref(), "video-sink", &video_sink)
    }

    fn connect_audio_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_audio_filter_trampoline<
            P: IsA<Pipeline>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESPipeline,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Pipeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::audio-filter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_audio_filter_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_audio_sink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_audio_sink_trampoline<P: IsA<Pipeline>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESPipeline,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Pipeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::audio-sink\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_audio_sink_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<P: IsA<Pipeline>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESPipeline,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Pipeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_timeline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeline_trampoline<P: IsA<Pipeline>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESPipeline,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Pipeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timeline_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_video_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_video_filter_trampoline<
            P: IsA<Pipeline>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESPipeline,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Pipeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::video-filter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_video_filter_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_video_sink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_video_sink_trampoline<P: IsA<Pipeline>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESPipeline,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Pipeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::video-sink\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_video_sink_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
