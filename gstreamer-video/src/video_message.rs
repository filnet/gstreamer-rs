// Take a look at the license at the top of the repository in the LICENSE file.

use crate::NavigationMessageType;
use glib::translate::{from_glib, from_glib_full, IntoGlib, ToGlibPtr};
use glib::ToSendValue;
use gst::ffi as gst_ffi;
use gst::{prelude::*, Message, Object, Seqnum};
use std::ptr;

macro_rules! message_builder_generic_impl {
    ($new_fn:expr) => {
        #[allow(clippy::needless_update)]
        pub fn src<O: IsA<Object> + Cast + Clone>(self, src: &O) -> Self {
            Self {
                builder: self.builder.src(src),
                ..self
            }
        }

        #[doc(alias = "gst_message_set_seqnum")]
        #[allow(clippy::needless_update)]
        pub fn seqnum(self, seqnum: Seqnum) -> Self {
            Self {
                builder: self.builder.seqnum(seqnum),
                ..self
            }
        }

        #[allow(clippy::needless_update)]
        pub fn other_fields(
            self,
            other_fields: &[(&'a str, &'a (dyn ToSendValue + Sync))],
        ) -> Self {
            Self {
                builder: self.builder.other_fields(other_fields),
                ..self
            }
        }

        #[must_use = "Building the message without using it has no effect"]
        pub fn build(mut self) -> Message {
            assert_initialized_main_thread!();
            unsafe {
                let src = self.builder.src.to_glib_none().0;
                let msg = $new_fn(&mut self, src);
                if let Some(seqnum) = self.builder.seqnum {
                    gst_ffi::gst_message_set_seqnum(msg, seqnum.into_glib());
                }

                if !self.builder.other_fields.is_empty() {
                    let structure = gst_ffi::gst_message_writable_structure(msg);

                    if !structure.is_null() {
                        let structure =
                            gst::StructureRef::from_glib_borrow_mut(structure as *mut _);

                        for (k, v) in self.builder.other_fields {
                            structure.set_value(k, v.to_send_value());
                        }
                    }
                }

                from_glib_full(msg)
            }
        }
    };
}

struct MessageBuilder<'a> {
    pub src: Option<Object>,
    pub seqnum: Option<Seqnum>,
    #[allow(unused)]
    pub other_fields: Vec<(&'a str, &'a (dyn ToSendValue + Sync))>,
}

impl<'a> MessageBuilder<'a> {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
        }
    }

    pub fn src<O: IsA<Object> + Cast + Clone>(self, src: &O) -> Self {
        Self {
            src: Some(src.clone().upcast::<Object>()),
            ..self
        }
    }

    pub fn seqnum(self, seqnum: Seqnum) -> Self {
        Self {
            seqnum: Some(seqnum),
            ..self
        }
    }

    pub fn other_fields(self, other_fields: &[(&'a str, &'a (dyn ToSendValue + Sync))]) -> Self {
        Self {
            other_fields: self
                .other_fields
                .iter()
                .cloned()
                .chain(other_fields.iter().cloned())
                .collect(),
            ..self
        }
    }
}

#[must_use = "The builder must be built to be used"]
pub struct NavigationEventMessageBuilder<'a> {
    builder: MessageBuilder<'a>,
    event: gst::Event,
}

impl<'a> NavigationEventMessageBuilder<'a> {
    fn new(event: gst::Event) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            event,
        }
    }

    message_builder_generic_impl!(|s: &Self, src| ffi::gst_navigation_message_new_event(
        src,
        s.event.to_glib_none().0
    ));
}

#[derive(Clone, Debug)]
pub struct NavigationEventMessage {
    pub event: gst::Event,
}

impl PartialEq for NavigationEventMessage {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.event.as_ptr() == other.event.as_ptr() }
    }
}

impl Eq for NavigationEventMessage {}

impl NavigationEventMessage {
    #[doc(alias = "gst_navigation_message_new_event")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(event: gst::Event) -> gst::Message {
        assert_initialized_main_thread!();
        NavigationEventMessageBuilder::new(event).build()
    }

    pub fn builder<'a>(event: gst::Event) -> NavigationEventMessageBuilder<'a> {
        assert_initialized_main_thread!();
        NavigationEventMessageBuilder::new(event)
    }

    #[doc(alias = "gst_navigation_message_parse_event")]
    pub fn parse(msg: &gst::MessageRef) -> Result<Self, glib::error::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let mut event = ptr::null_mut();
            let ret = from_glib(ffi::gst_navigation_message_parse_event(
                msg.as_mut_ptr(),
                &mut event,
            ));
            if ret {
                Ok(Self {
                    event: from_glib_full(event),
                })
            } else {
                Err(glib::bool_error!("Invalid navigation event msg"))
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum NavigationMessage {
    Event(NavigationEventMessage),
}

impl NavigationMessage {
    #[doc(alias = "gst_navigation_message_get_type")]
    pub fn type_(msg: &gst::MessageRef) -> NavigationMessageType {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gst_navigation_message_get_type(msg.as_mut_ptr())) }
    }

    #[doc(alias = "gst_navigation_message_parse_event")]
    pub fn parse(msg: &gst::MessageRef) -> Result<Self, glib::error::BoolError> {
        skip_assert_initialized!();

        let event_type: NavigationMessageType = Self::type_(msg);

        match event_type {
            NavigationMessageType::Event => NavigationEventMessage::parse(msg).map(Self::Event),
            _ => {
                return Err(glib::bool_error!(
                    "Unsupported navigation msg {:?}",
                    event_type
                ))
            }
        }
    }
}
