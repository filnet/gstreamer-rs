// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::EncodingProfile;
use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstEncodingContainerProfile")]
    pub struct EncodingContainerProfile(Object<ffi::GstEncodingContainerProfile, ffi::GstEncodingContainerProfileClass>) @extends EncodingProfile;

    match fn {
        type_ => || ffi::gst_encoding_container_profile_get_type(),
    }
}

impl EncodingContainerProfile {
    #[doc(alias = "gst_encoding_container_profile_contains_profile")]
    pub fn contains_profile(&self, profile: &impl IsA<EncodingProfile>) -> bool {
        unsafe {
            from_glib(ffi::gst_encoding_container_profile_contains_profile(
                self.to_glib_none().0,
                profile.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_container_profile_get_profiles")]
    #[doc(alias = "get_profiles")]
    pub fn profiles(&self) -> Vec<EncodingProfile> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_encoding_container_profile_get_profiles(
                self.to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for EncodingContainerProfile {}
unsafe impl Sync for EncodingContainerProfile {}
