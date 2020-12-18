// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::error::ErrorDomain;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::Quark;
use glib::StaticType;
use glib::Type;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstPlayerColorBalanceType")]
pub enum PlayerColorBalanceType {
    #[doc(alias = "GST_PLAYER_COLOR_BALANCE_HUE")]
    Hue,
    #[doc(alias = "GST_PLAYER_COLOR_BALANCE_BRIGHTNESS")]
    Brightness,
    #[doc(alias = "GST_PLAYER_COLOR_BALANCE_SATURATION")]
    Saturation,
    #[doc(alias = "GST_PLAYER_COLOR_BALANCE_CONTRAST")]
    Contrast,
    #[doc(hidden)]
    __Unknown(i32),
}

impl PlayerColorBalanceType {
    #[doc(alias = "gst_player_color_balance_type_get_name")]
    pub fn get_name(self) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gst_player_color_balance_type_get_name(self.to_glib())) }
    }
}

#[doc(hidden)]
impl ToGlib for PlayerColorBalanceType {
    type GlibType = ffi::GstPlayerColorBalanceType;

    fn to_glib(&self) -> ffi::GstPlayerColorBalanceType {
        match *self {
            PlayerColorBalanceType::Hue => ffi::GST_PLAYER_COLOR_BALANCE_HUE,
            PlayerColorBalanceType::Brightness => ffi::GST_PLAYER_COLOR_BALANCE_BRIGHTNESS,
            PlayerColorBalanceType::Saturation => ffi::GST_PLAYER_COLOR_BALANCE_SATURATION,
            PlayerColorBalanceType::Contrast => ffi::GST_PLAYER_COLOR_BALANCE_CONTRAST,
            PlayerColorBalanceType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPlayerColorBalanceType> for PlayerColorBalanceType {
    unsafe fn from_glib(value: ffi::GstPlayerColorBalanceType) -> Self {
        skip_assert_initialized!();
        match value {
            3 => PlayerColorBalanceType::Hue,
            0 => PlayerColorBalanceType::Brightness,
            2 => PlayerColorBalanceType::Saturation,
            1 => PlayerColorBalanceType::Contrast,
            value => PlayerColorBalanceType::__Unknown(value),
        }
    }
}

impl StaticType for PlayerColorBalanceType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_player_color_balance_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PlayerColorBalanceType {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PlayerColorBalanceType {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for PlayerColorBalanceType {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstPlayerError")]
pub enum PlayerError {
    #[doc(alias = "GST_PLAYER_ERROR_FAILED")]
    Failed,
    #[doc(hidden)]
    __Unknown(i32),
}

impl PlayerError {
    #[doc(alias = "gst_player_error_get_name")]
    pub fn get_name(self) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gst_player_error_get_name(self.to_glib())) }
    }
}

#[doc(hidden)]
impl ToGlib for PlayerError {
    type GlibType = ffi::GstPlayerError;

    fn to_glib(&self) -> ffi::GstPlayerError {
        match *self {
            PlayerError::Failed => ffi::GST_PLAYER_ERROR_FAILED,
            PlayerError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPlayerError> for PlayerError {
    unsafe fn from_glib(value: ffi::GstPlayerError) -> Self {
        skip_assert_initialized!();
        match value {
            0 => PlayerError::Failed,
            value => PlayerError::__Unknown(value),
        }
    }
}

impl ErrorDomain for PlayerError {
    fn domain() -> Quark {
        skip_assert_initialized!();

        unsafe { from_glib(ffi::gst_player_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match code {
            0 => Some(PlayerError::Failed),
            _ => Some(PlayerError::Failed),
        }
    }
}

impl StaticType for PlayerError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_player_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PlayerError {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PlayerError {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for PlayerError {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstPlayerSnapshotFormat")]
pub enum PlayerSnapshotFormat {
    #[doc(alias = "GST_PLAYER_THUMBNAIL_RAW_NATIVE")]
    RawNative,
    #[doc(alias = "GST_PLAYER_THUMBNAIL_RAW_xRGB")]
    RawXrgb,
    #[doc(alias = "GST_PLAYER_THUMBNAIL_RAW_BGRx")]
    RawBgrx,
    #[doc(alias = "GST_PLAYER_THUMBNAIL_JPG")]
    Jpg,
    #[doc(alias = "GST_PLAYER_THUMBNAIL_PNG")]
    Png,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for PlayerSnapshotFormat {
    type GlibType = ffi::GstPlayerSnapshotFormat;

    fn to_glib(&self) -> ffi::GstPlayerSnapshotFormat {
        match *self {
            PlayerSnapshotFormat::RawNative => ffi::GST_PLAYER_THUMBNAIL_RAW_NATIVE,
            PlayerSnapshotFormat::RawXrgb => ffi::GST_PLAYER_THUMBNAIL_RAW_xRGB,
            PlayerSnapshotFormat::RawBgrx => ffi::GST_PLAYER_THUMBNAIL_RAW_BGRx,
            PlayerSnapshotFormat::Jpg => ffi::GST_PLAYER_THUMBNAIL_JPG,
            PlayerSnapshotFormat::Png => ffi::GST_PLAYER_THUMBNAIL_PNG,
            PlayerSnapshotFormat::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPlayerSnapshotFormat> for PlayerSnapshotFormat {
    unsafe fn from_glib(value: ffi::GstPlayerSnapshotFormat) -> Self {
        skip_assert_initialized!();
        match value {
            0 => PlayerSnapshotFormat::RawNative,
            1 => PlayerSnapshotFormat::RawXrgb,
            2 => PlayerSnapshotFormat::RawBgrx,
            3 => PlayerSnapshotFormat::Jpg,
            4 => PlayerSnapshotFormat::Png,
            value => PlayerSnapshotFormat::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstPlayerState")]
pub enum PlayerState {
    #[doc(alias = "GST_PLAYER_STATE_STOPPED")]
    Stopped,
    #[doc(alias = "GST_PLAYER_STATE_BUFFERING")]
    Buffering,
    #[doc(alias = "GST_PLAYER_STATE_PAUSED")]
    Paused,
    #[doc(alias = "GST_PLAYER_STATE_PLAYING")]
    Playing,
    #[doc(hidden)]
    __Unknown(i32),
}

impl PlayerState {
    #[doc(alias = "gst_player_state_get_name")]
    pub fn get_name(self) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gst_player_state_get_name(self.to_glib())) }
    }
}

#[doc(hidden)]
impl ToGlib for PlayerState {
    type GlibType = ffi::GstPlayerState;

    fn to_glib(&self) -> ffi::GstPlayerState {
        match *self {
            PlayerState::Stopped => ffi::GST_PLAYER_STATE_STOPPED,
            PlayerState::Buffering => ffi::GST_PLAYER_STATE_BUFFERING,
            PlayerState::Paused => ffi::GST_PLAYER_STATE_PAUSED,
            PlayerState::Playing => ffi::GST_PLAYER_STATE_PLAYING,
            PlayerState::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPlayerState> for PlayerState {
    unsafe fn from_glib(value: ffi::GstPlayerState) -> Self {
        skip_assert_initialized!();
        match value {
            0 => PlayerState::Stopped,
            1 => PlayerState::Buffering,
            2 => PlayerState::Paused,
            3 => PlayerState::Playing,
            value => PlayerState::__Unknown(value),
        }
    }
}

impl StaticType for PlayerState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_player_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PlayerState {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PlayerState {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for PlayerState {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}
