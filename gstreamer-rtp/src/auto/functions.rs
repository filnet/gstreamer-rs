// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::GString;
use gst_rtp_sys;
use RTCPSDESType;

//#[cfg(any(feature = "v1_16", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
//pub fn buffer_add_rtp_source_meta(buffer: &gst::Buffer, ssrc: u32, csrc: u32, csrc_count: u32) -> /*Ignored*/Option<RTPSourceMeta> {
//    unsafe { TODO: call gst_rtp_sys:gst_buffer_add_rtp_source_meta() }
//}

//#[cfg(any(feature = "v1_16", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
//pub fn buffer_get_rtp_source_meta(buffer: &gst::Buffer) -> /*Ignored*/Option<RTPSourceMeta> {
//    unsafe { TODO: call gst_rtp_sys:gst_buffer_get_rtp_source_meta() }
//}

pub fn rtcp_ntp_to_unix(ntptime: u64) -> u64 {
    assert_initialized_main_thread!();
    unsafe { gst_rtp_sys::gst_rtcp_ntp_to_unix(ntptime) }
}

pub fn rtcp_sdes_name_to_type(name: &str) -> RTCPSDESType {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(gst_rtp_sys::gst_rtcp_sdes_name_to_type(
            name.to_glib_none().0,
        ))
    }
}

pub fn rtcp_sdes_type_to_name(type_: RTCPSDESType) -> Option<GString> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(gst_rtp_sys::gst_rtcp_sdes_type_to_name(type_.to_glib())) }
}

pub fn rtcp_unix_to_ntp(unixtime: u64) -> u64 {
    assert_initialized_main_thread!();
    unsafe { gst_rtp_sys::gst_rtcp_unix_to_ntp(unixtime) }
}

//pub fn rtp_hdrext_set_ntp_56(data: /*Unimplemented*/Option<Fundamental: Pointer>, size: u32, ntptime: u64) -> bool {
//    unsafe { TODO: call gst_rtp_sys:gst_rtp_hdrext_set_ntp_56() }
//}

//pub fn rtp_hdrext_set_ntp_64(data: /*Unimplemented*/Option<Fundamental: Pointer>, size: u32, ntptime: u64) -> bool {
//    unsafe { TODO: call gst_rtp_sys:gst_rtp_hdrext_set_ntp_64() }
//}
