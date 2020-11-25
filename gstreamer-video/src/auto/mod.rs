// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod video_buffer_pool;
pub use self::video_buffer_pool::{VideoBufferPool, NONE_VIDEO_BUFFER_POOL};

mod video_decoder;
pub use self::video_decoder::VideoDecoderExt;
pub use self::video_decoder::{VideoDecoder, NONE_VIDEO_DECODER};

mod video_encoder;
pub use self::video_encoder::VideoEncoderExt;
pub use self::video_encoder::{VideoEncoder, NONE_VIDEO_ENCODER};

mod video_filter;
pub use self::video_filter::{VideoFilter, NONE_VIDEO_FILTER};

mod video_overlay;
pub use self::video_overlay::VideoOverlayExt;
pub use self::video_overlay::{VideoOverlay, NONE_VIDEO_OVERLAY};

mod video_sink;
pub use self::video_sink::VideoSinkExt;
pub use self::video_sink::{VideoSink, NONE_VIDEO_SINK};

mod enums;
#[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
pub use self::enums::VideoAFDSpec;
#[cfg(any(feature = "v1_18", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_18")))]
pub use self::enums::VideoAFDValue;
pub use self::enums::VideoAlphaMode;
#[cfg(any(feature = "v1_16", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_16")))]
pub use self::enums::VideoCaptionType;
pub use self::enums::VideoChromaMode;
pub use self::enums::VideoColorMatrix;
pub use self::enums::VideoColorPrimaries;
pub use self::enums::VideoDitherMethod;
#[cfg(any(feature = "v1_12", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_12")))]
pub use self::enums::VideoFieldOrder;
pub use self::enums::VideoFormat;
pub use self::enums::VideoGammaMode;
pub use self::enums::VideoInterlaceMode;
pub use self::enums::VideoMatrixMode;
pub use self::enums::VideoMultiviewFramePacking;
pub use self::enums::VideoMultiviewMode;
pub use self::enums::VideoPrimariesMode;
pub use self::enums::VideoResamplerMethod;
pub use self::enums::VideoTileMode;
pub use self::enums::VideoTransferFunction;

mod flags;
pub use self::flags::VideoBufferFlags;
pub use self::flags::VideoChromaSite;
pub use self::flags::VideoCodecFrameFlags;
pub use self::flags::VideoFlags;
pub use self::flags::VideoFormatFlags;
pub use self::flags::VideoFrameFlags;
pub use self::flags::VideoMultiviewFlags;
pub use self::flags::VideoOverlayFormatFlags;
pub use self::flags::VideoPackFlags;
#[cfg(any(feature = "v1_10", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_10")))]
pub use self::flags::VideoTimeCodeFlags;

#[doc(hidden)]
pub mod traits {
    pub use super::VideoDecoderExt;
    pub use super::VideoEncoderExt;
    pub use super::VideoOverlayExt;
    pub use super::VideoSinkExt;
}
