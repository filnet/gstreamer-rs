// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
use crate::Edge;
#[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
use crate::EditMode;
use crate::Extractable;
#[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
use crate::Layer;
use crate::Timeline;
use crate::TrackType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use std::boxed::Box as Box_;
#[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
use std::mem;
use std::mem::transmute;
#[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
use std::ptr;

glib::glib_wrapper! {
    pub struct TimelineElement(Object<ffi::GESTimelineElement, ffi::GESTimelineElementClass>) @implements Extractable;

    match fn {
        get_type => || ffi::ges_timeline_element_get_type(),
    }
}

pub const NONE_TIMELINE_ELEMENT: Option<&TimelineElement> = None;

pub trait TimelineElementExt: 'static {
    //fn add_child_property<P: IsA<glib::Object>>(&self, pspec: /*Ignored*/&glib::ParamSpec, child: &P) -> bool;

    fn copy(&self, deep: bool) -> Result<TimelineElement, glib::BoolError>;

    #[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
    fn edit(
        &self,
        layers: &[Layer],
        new_layer_priority: i64,
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> bool;

    #[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
    fn edit_full(
        &self,
        new_layer_priority: i64,
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> Result<(), glib::Error>;

    //fn get_child_properties(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn get_child_property(&self, property_name: &str, value: /*Ignored*/glib::Value) -> bool;

    //fn get_child_property_by_pspec(&self, pspec: /*Ignored*/&glib::ParamSpec, value: /*Ignored*/glib::Value);

    //fn get_child_property_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    fn get_duration(&self) -> gst::ClockTime;

    fn get_inpoint(&self) -> gst::ClockTime;

    #[cfg(any(feature = "v1_16", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_16")))]
    fn get_layer_priority(&self) -> u32;

    fn get_max_duration(&self) -> gst::ClockTime;

    fn get_name(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
    fn get_natural_framerate(&self) -> Option<(i32, i32)>;

    fn get_parent(&self) -> Option<TimelineElement>;

    fn get_priority(&self) -> u32;

    fn get_start(&self) -> gst::ClockTime;

    fn get_timeline(&self) -> Option<Timeline>;

    fn get_toplevel_parent(&self) -> Option<TimelineElement>;

    fn get_track_types(&self) -> TrackType;

    //fn list_children_properties(&self) -> /*Ignored*/Vec<glib::ParamSpec>;

    //fn lookup_child(&self, prop_name: &str, pspec: /*Ignored*/glib::ParamSpec) -> Option<glib::Object>;

    fn paste(&self, paste_position: gst::ClockTime) -> Result<TimelineElement, glib::BoolError>;

    //fn remove_child_property(&self, pspec: /*Ignored*/&glib::ParamSpec) -> bool;

    fn ripple(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError>;

    fn ripple_end(&self, end: gst::ClockTime) -> Result<(), glib::error::BoolError>;

    fn roll_end(&self, end: gst::ClockTime) -> Result<(), glib::error::BoolError>;

    fn roll_start(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError>;

    //fn set_child_properties(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn set_child_property(&self, property_name: &str, value: /*Ignored*/&glib::Value) -> bool;

    //fn set_child_property_by_pspec(&self, pspec: /*Ignored*/&glib::ParamSpec, value: /*Ignored*/&glib::Value);

    //#[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
    //#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
    //fn set_child_property_full(&self, property_name: &str, value: /*Ignored*/&glib::Value) -> Result<(), glib::Error>;

    //fn set_child_property_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    fn set_duration(&self, duration: gst::ClockTime) -> bool;

    fn set_inpoint(&self, inpoint: gst::ClockTime) -> bool;

    fn set_max_duration(&self, maxduration: gst::ClockTime) -> bool;

    fn set_name(&self, name: Option<&str>) -> Result<(), glib::error::BoolError>;

    fn set_parent<P: IsA<TimelineElement>>(&self, parent: &P)
        -> Result<(), glib::error::BoolError>;

    #[cfg_attr(feature = "v1_10", deprecated)]
    fn set_priority(&self, priority: u32) -> bool;

    fn set_start(&self, start: gst::ClockTime) -> bool;

    fn set_timeline<P: IsA<Timeline>>(&self, timeline: &P) -> Result<(), glib::error::BoolError>;

    fn trim(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError>;

    fn get_property_in_point(&self) -> u64;

    fn set_property_in_point(&self, in_point: u64);

    fn get_property_serialize(&self) -> bool;

    fn set_property_serialize(&self, serialize: bool);

    //#[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
    //#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
    //fn connect_child_property_added<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
    //#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
    //fn connect_child_property_removed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //fn connect_deep_notify<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_in_point_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_duration_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v1_10", deprecated)]
    fn connect_property_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_serialize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_timeline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TimelineElement>> TimelineElementExt for O {
    //fn add_child_property<P: IsA<glib::Object>>(&self, pspec: /*Ignored*/&glib::ParamSpec, child: &P) -> bool {
    //    unsafe { TODO: call ffi:ges_timeline_element_add_child_property() }
    //}

    fn copy(&self, deep: bool) -> Result<TimelineElement, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_none(ffi::ges_timeline_element_copy(
                self.as_ref().to_glib_none().0,
                deep.to_glib(),
            ))
            .ok_or_else(|| glib::glib_bool_error!("Failed to copy timeline element"))
        }
    }

    #[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
    fn edit(
        &self,
        layers: &[Layer],
        new_layer_priority: i64,
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_edit(
                self.as_ref().to_glib_none().0,
                layers.to_glib_none().0,
                new_layer_priority,
                mode.to_glib(),
                edge.to_glib(),
                position,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
    fn edit_full(
        &self,
        new_layer_priority: i64,
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ges_timeline_element_edit_full(
                self.as_ref().to_glib_none().0,
                new_layer_priority,
                mode.to_glib(),
                edge.to_glib(),
                position,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //fn get_child_properties(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:ges_timeline_element_get_child_properties() }
    //}

    //fn get_child_property(&self, property_name: &str, value: /*Ignored*/glib::Value) -> bool {
    //    unsafe { TODO: call ffi:ges_timeline_element_get_child_property() }
    //}

    //fn get_child_property_by_pspec(&self, pspec: /*Ignored*/&glib::ParamSpec, value: /*Ignored*/glib::Value) {
    //    unsafe { TODO: call ffi:ges_timeline_element_get_child_property_by_pspec() }
    //}

    //fn get_child_property_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:ges_timeline_element_get_child_property_valist() }
    //}

    fn get_duration(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::ges_timeline_element_get_duration(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_inpoint(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::ges_timeline_element_get_inpoint(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_16")))]
    fn get_layer_priority(&self) -> u32 {
        unsafe { ffi::ges_timeline_element_get_layer_priority(self.as_ref().to_glib_none().0) }
    }

    fn get_max_duration(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::ges_timeline_element_get_max_duration(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::ges_timeline_element_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
    fn get_natural_framerate(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut framerate_n = mem::MaybeUninit::uninit();
            let mut framerate_d = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::ges_timeline_element_get_natural_framerate(
                self.as_ref().to_glib_none().0,
                framerate_n.as_mut_ptr(),
                framerate_d.as_mut_ptr(),
            ));
            let framerate_n = framerate_n.assume_init();
            let framerate_d = framerate_d.assume_init();
            if ret {
                Some((framerate_n, framerate_d))
            } else {
                None
            }
        }
    }

    fn get_parent(&self) -> Option<TimelineElement> {
        unsafe {
            from_glib_full(ffi::ges_timeline_element_get_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_priority(&self) -> u32 {
        unsafe { ffi::ges_timeline_element_get_priority(self.as_ref().to_glib_none().0) }
    }

    fn get_start(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::ges_timeline_element_get_start(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_timeline(&self) -> Option<Timeline> {
        unsafe {
            from_glib_full(ffi::ges_timeline_element_get_timeline(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_toplevel_parent(&self) -> Option<TimelineElement> {
        unsafe {
            from_glib_full(ffi::ges_timeline_element_get_toplevel_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_track_types(&self) -> TrackType {
        unsafe {
            from_glib(ffi::ges_timeline_element_get_track_types(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn list_children_properties(&self) -> /*Ignored*/Vec<glib::ParamSpec> {
    //    unsafe { TODO: call ffi:ges_timeline_element_list_children_properties() }
    //}

    //fn lookup_child(&self, prop_name: &str, pspec: /*Ignored*/glib::ParamSpec) -> Option<glib::Object> {
    //    unsafe { TODO: call ffi:ges_timeline_element_lookup_child() }
    //}

    fn paste(&self, paste_position: gst::ClockTime) -> Result<TimelineElement, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::ges_timeline_element_paste(
                self.as_ref().to_glib_none().0,
                paste_position.to_glib(),
            ))
            .ok_or_else(|| glib::glib_bool_error!("Failed to paste timeline element"))
        }
    }

    //fn remove_child_property(&self, pspec: /*Ignored*/&glib::ParamSpec) -> bool {
    //    unsafe { TODO: call ffi:ges_timeline_element_remove_child_property() }
    //}

    fn ripple(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::ges_timeline_element_ripple(self.as_ref().to_glib_none().0, start.to_glib()),
                "Failed to ripple"
            )
        }
    }

    fn ripple_end(&self, end: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::ges_timeline_element_ripple_end(self.as_ref().to_glib_none().0, end.to_glib()),
                "Failed to ripple"
            )
        }
    }

    fn roll_end(&self, end: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::ges_timeline_element_roll_end(self.as_ref().to_glib_none().0, end.to_glib()),
                "Failed to roll"
            )
        }
    }

    fn roll_start(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::ges_timeline_element_roll_start(
                    self.as_ref().to_glib_none().0,
                    start.to_glib()
                ),
                "Failed to roll"
            )
        }
    }

    //fn set_child_properties(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:ges_timeline_element_set_child_properties() }
    //}

    //fn set_child_property(&self, property_name: &str, value: /*Ignored*/&glib::Value) -> bool {
    //    unsafe { TODO: call ffi:ges_timeline_element_set_child_property() }
    //}

    //fn set_child_property_by_pspec(&self, pspec: /*Ignored*/&glib::ParamSpec, value: /*Ignored*/&glib::Value) {
    //    unsafe { TODO: call ffi:ges_timeline_element_set_child_property_by_pspec() }
    //}

    //#[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
    //#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
    //fn set_child_property_full(&self, property_name: &str, value: /*Ignored*/&glib::Value) -> Result<(), glib::Error> {
    //    unsafe { TODO: call ffi:ges_timeline_element_set_child_property_full() }
    //}

    //fn set_child_property_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:ges_timeline_element_set_child_property_valist() }
    //}

    fn set_duration(&self, duration: gst::ClockTime) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_set_duration(
                self.as_ref().to_glib_none().0,
                duration.to_glib(),
            ))
        }
    }

    fn set_inpoint(&self, inpoint: gst::ClockTime) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_set_inpoint(
                self.as_ref().to_glib_none().0,
                inpoint.to_glib(),
            ))
        }
    }

    fn set_max_duration(&self, maxduration: gst::ClockTime) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_set_max_duration(
                self.as_ref().to_glib_none().0,
                maxduration.to_glib(),
            ))
        }
    }

    fn set_name(&self, name: Option<&str>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::ges_timeline_element_set_name(
                    self.as_ref().to_glib_none().0,
                    name.to_glib_none().0
                ),
                "Failed to set name"
            )
        }
    }

    fn set_parent<P: IsA<TimelineElement>>(
        &self,
        parent: &P,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::ges_timeline_element_set_parent(
                    self.as_ref().to_glib_none().0,
                    parent.as_ref().to_glib_none().0
                ),
                "`TimelineElement` already had a parent or its parent was the same as specified"
            )
        }
    }

    fn set_priority(&self, priority: u32) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_set_priority(
                self.as_ref().to_glib_none().0,
                priority,
            ))
        }
    }

    fn set_start(&self, start: gst::ClockTime) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_set_start(
                self.as_ref().to_glib_none().0,
                start.to_glib(),
            ))
        }
    }

    fn set_timeline<P: IsA<Timeline>>(&self, timeline: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::ges_timeline_element_set_timeline(
                    self.as_ref().to_glib_none().0,
                    timeline.as_ref().to_glib_none().0
                ),
                "`Failed to set timeline"
            )
        }
    }

    fn trim(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::ges_timeline_element_trim(self.as_ref().to_glib_none().0, start.to_glib()),
                "Failed to trim"
            )
        }
    }

    fn get_property_in_point(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"in-point\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `in-point` getter")
                .unwrap()
        }
    }

    fn set_property_in_point(&self, in_point: u64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"in-point\0".as_ptr() as *const _,
                Value::from(&in_point).to_glib_none().0,
            );
        }
    }

    fn get_property_serialize(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"serialize\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `serialize` getter")
                .unwrap()
        }
    }

    fn set_property_serialize(&self, serialize: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"serialize\0".as_ptr() as *const _,
                Value::from(&serialize).to_glib_none().0,
            );
        }
    }

    //#[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
    //#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
    //fn connect_child_property_added<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored prop: GObject.ParamSpec
    //}

    //#[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
    //#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
    //fn connect_child_property_removed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored prop: GObject.ParamSpec
    //}

    //fn connect_deep_notify<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored prop: GObject.ParamSpec
    //}

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_duration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_in_point_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_in_point_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::in-point\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_in_point_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_max_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_duration_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_duration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_property_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_priority_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::priority\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_priority_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_serialize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_serialize_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::serialize\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_serialize_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_start_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::start\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_start_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_timeline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeline_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
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
}
