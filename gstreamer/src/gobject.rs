// Take a look at the license at the top of the repository in the LICENSE file.

use crate::value::GstValueExt;
use glib::prelude::*;

pub trait GObjectExtManualGst: 'static {
    #[doc(alias = "gst_util_set_object_arg")]
    fn try_set_property_from_str(&self, name: &str, value: &str) -> Result<(), glib::BoolError>;

    #[doc(alias = "gst_util_set_object_arg")]
    fn set_property_from_str(&self, name: &str, value: &str);
}

impl<O: IsA<glib::Object>> GObjectExtManualGst for O {
    fn try_set_property_from_str(&self, name: &str, value: &str) -> Result<(), glib::BoolError> {
        let pspec = self.find_property(name).ok_or_else(|| {
            glib::bool_error!("property '{}' of type '{}' not found", name, self.type_())
        })?;

        let value = {
            if pspec.value_type() == crate::Structure::static_type() && value == "NULL" {
                None::<crate::Structure>.to_value()
            } else {
                #[cfg(feature = "v1_20")]
                {
                    glib::Value::deserialize_with_pspec(value, &pspec)?
                }
                #[cfg(not(feature = "v1_20"))]
                {
                    glib::Value::deserialize(value, pspec.value_type())?
                }
            }
        };

        self.try_set_property_from_value(name, &value)
    }

    fn set_property_from_str(&self, name: &str, value: &str) {
        self.try_set_property_from_str(name, value).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_property_from_str() {
        crate::init().unwrap();

        let fakesink = crate::ElementFactory::make("fakesink", None).unwrap();
        fakesink.set_property_from_str("state-error", "ready-to-paused");
        let v = fakesink.property_value("state-error");
        let (_klass, e) = glib::EnumValue::from_value(&v).unwrap();
        assert_eq!(e.nick(), "ready-to-paused");
    }
}
