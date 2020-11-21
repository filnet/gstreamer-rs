// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use glib::StaticType;
use glib::Type;

bitflags! {
    pub struct AudioFlags: u32 {
        const UNPOSITIONED = 1;
    }
}

#[doc(hidden)]
impl ToGlib for AudioFlags {
    type GlibType = ffi::GstAudioFlags;

    fn to_glib(&self) -> ffi::GstAudioFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioFlags> for AudioFlags {
    fn from_glib(value: ffi::GstAudioFlags) -> AudioFlags {
        skip_assert_initialized!();
        AudioFlags::from_bits_truncate(value)
    }
}

impl StaticType for AudioFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AudioFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AudioFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for AudioFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct AudioFormatFlags: u32 {
        const INTEGER = 1;
        const FLOAT = 2;
        const SIGNED = 4;
        const COMPLEX = 16;
        const UNPACK = 32;
    }
}

#[doc(hidden)]
impl ToGlib for AudioFormatFlags {
    type GlibType = ffi::GstAudioFormatFlags;

    fn to_glib(&self) -> ffi::GstAudioFormatFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioFormatFlags> for AudioFormatFlags {
    fn from_glib(value: ffi::GstAudioFormatFlags) -> AudioFormatFlags {
        skip_assert_initialized!();
        AudioFormatFlags::from_bits_truncate(value)
    }
}

impl StaticType for AudioFormatFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_format_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AudioFormatFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AudioFormatFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for AudioFormatFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct AudioPackFlags: u32 {
        const TRUNCATE_RANGE = 1;
    }
}

#[doc(hidden)]
impl ToGlib for AudioPackFlags {
    type GlibType = ffi::GstAudioPackFlags;

    fn to_glib(&self) -> ffi::GstAudioPackFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioPackFlags> for AudioPackFlags {
    fn from_glib(value: ffi::GstAudioPackFlags) -> AudioPackFlags {
        skip_assert_initialized!();
        AudioPackFlags::from_bits_truncate(value)
    }
}

impl StaticType for AudioPackFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_pack_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AudioPackFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AudioPackFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for AudioPackFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}
