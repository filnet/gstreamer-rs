// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::non_send_fields_in_send_ty)]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use glib;
pub use gst;

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { gst::ffi::gst_is_initialized() } != glib::ffi::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
#[allow(clippy::use_self)]
#[allow(unused_imports)]
mod auto;
mod control_point;
pub use crate::auto::*;
use crate::control_point::*;

pub mod prelude {
    #[doc(hidden)]
    pub use gst::prelude::*;

    pub use crate::auto::traits::*;
}
