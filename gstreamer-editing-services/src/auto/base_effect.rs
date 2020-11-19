// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ges_sys;
use glib::object::IsA;
use glib::translate::*;
use Extractable;
use TimelineElement;
use TrackElement;

glib_wrapper! {
    pub struct BaseEffect(Object<ges_sys::GESBaseEffect, ges_sys::GESBaseEffectClass>) @extends TrackElement, TimelineElement, @implements Extractable;

    match fn {
        get_type => || ges_sys::ges_base_effect_get_type(),
    }
}

pub const NONE_BASE_EFFECT: Option<&BaseEffect> = None;

pub trait BaseEffectExt: 'static {
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn is_time_effect(&self) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn register_time_property(&self, child_property_name: &str) -> bool;

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //fn set_time_translation_funcs(&self, source_to_sink_func: /*Unimplemented*/Fn(&BaseEffect, gst::ClockTime, /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 3, id: 11 }) -> gst::ClockTime, sink_to_source_func: /*Unimplemented*/Fn(&BaseEffect, gst::ClockTime, /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 3, id: 11 }) -> gst::ClockTime, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool;
}

impl<O: IsA<BaseEffect>> BaseEffectExt for O {
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn is_time_effect(&self) -> bool {
        unsafe {
            from_glib(ges_sys::ges_base_effect_is_time_effect(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn register_time_property(&self, child_property_name: &str) -> bool {
        unsafe {
            from_glib(ges_sys::ges_base_effect_register_time_property(
                self.as_ref().to_glib_none().0,
                child_property_name.to_glib_none().0,
            ))
        }
    }

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //fn set_time_translation_funcs(&self, source_to_sink_func: /*Unimplemented*/Fn(&BaseEffect, gst::ClockTime, /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 3, id: 11 }) -> gst::ClockTime, sink_to_source_func: /*Unimplemented*/Fn(&BaseEffect, gst::ClockTime, /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 3, id: 11 }) -> gst::ClockTime, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call ges_sys:ges_base_effect_set_time_translation_funcs() }
    //}
}
