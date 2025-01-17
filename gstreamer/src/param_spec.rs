// Take a look at the license at the top of the repository in the LICENSE file.

use glib::gobject_ffi;
use glib::translate::*;
use glib::ParamSpec;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GstParamSpecFraction")]
    pub struct ParamSpecFraction(Shared<ffi::GstParamSpecFraction>);

    match fn {
        ref => |ptr| gobject_ffi::g_param_spec_ref_sink(ptr as *mut gobject_ffi::GParamSpec),
        unref => |ptr| gobject_ffi::g_param_spec_unref(ptr as *mut gobject_ffi::GParamSpec),
        type_ => || ffi::gst_param_spec_fraction_get_type(),
    }
}

unsafe impl Send for ParamSpecFraction {}
unsafe impl Sync for ParamSpecFraction {}

impl std::ops::Deref for ParamSpecFraction {
    type Target = ParamSpec;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const ParamSpecFraction as *const ParamSpec) }
    }
}

unsafe impl glib::ParamSpecType for ParamSpecFraction {}

#[doc(hidden)]
impl FromGlibPtrFull<*mut gobject_ffi::GParamSpec> for ParamSpecFraction {
    unsafe fn from_glib_full(ptr: *mut gobject_ffi::GParamSpec) -> Self {
        from_glib_full(ptr as *mut ffi::GstParamSpecFraction)
    }
}

impl ParamSpecFraction {
    pub fn builder(name: &str) -> ParamSpecFractionBuilder {
        assert_initialized_main_thread!();
        ParamSpecFractionBuilder::new(name)
    }

    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "gst_param_spec_fraction")]
    pub fn new(
        name: &str,
        nick: &str,
        blurb: &str,
        min: crate::Fraction,
        max: crate::Fraction,
        default: crate::Fraction,
        flags: glib::ParamFlags,
    ) -> glib::ParamSpec {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_param_spec_fraction(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                min.numer(),
                min.denom(),
                max.numer(),
                max.denom(),
                default.numer(),
                default.denom(),
                flags.into_glib(),
            ))
        }
    }

    pub fn minimum(&self) -> crate::Fraction {
        unsafe {
            let ptr = self.to_glib_none().0;

            crate::Fraction::new((*ptr).min_num, (*ptr).min_den)
        }
    }

    pub fn maximum(&self) -> crate::Fraction {
        unsafe {
            let ptr = self.to_glib_none().0;

            crate::Fraction::new((*ptr).max_num, (*ptr).max_den)
        }
    }

    pub fn default_value(&self) -> crate::Fraction {
        unsafe {
            let ptr = self.to_glib_none().0;

            crate::Fraction::new((*ptr).def_num, (*ptr).def_den)
        }
    }

    pub fn upcast(self) -> ParamSpec {
        unsafe { from_glib_full(self.to_glib_full() as *mut gobject_ffi::GParamSpec) }
    }

    pub fn upcast_ref(&self) -> &ParamSpec {
        &*self
    }
}

#[derive(Default)]
#[must_use]
pub struct ParamSpecFractionBuilder<'a> {
    name: &'a str,
    nick: Option<&'a str>,
    blurb: Option<&'a str>,
    flags: glib::ParamFlags,
    minimum: Option<crate::Fraction>,
    maximum: Option<crate::Fraction>,
    default_value: Option<crate::Fraction>,
}

impl<'a> ParamSpecFractionBuilder<'a> {
    fn new(name: &'a str) -> Self {
        assert_initialized_main_thread!();
        Self {
            name,
            ..Default::default()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Default: `self.name`
    pub fn nick(mut self, nick: &'a str) -> Self {
        self.nick = Some(nick);
        self
    }

    // rustdoc-stripper-ignore-next
    /// Default: `self.name`
    pub fn blurb(mut self, blurb: &'a str) -> Self {
        self.blurb = Some(blurb);
        self
    }

    // rustdoc-stripper-ignore-next
    /// Default: `glib::ParamFlags::READWRITE`
    pub fn flags(mut self, flags: glib::ParamFlags) -> Self {
        self.flags = flags;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Default: `-i32::MAX/1`
    pub fn minimum(mut self, minimum: crate::Fraction) -> Self {
        self.minimum = Some(minimum);
        self
    }

    // rustdoc-stripper-ignore-next
    /// Default: `i32::MAX/1`
    pub fn maximum(mut self, maximum: crate::Fraction) -> Self {
        self.maximum = Some(maximum);
        self
    }

    // rustdoc-stripper-ignore-next
    /// Default: `0/1`
    pub fn default_value(mut self, default_value: crate::Fraction) -> Self {
        self.default_value = Some(default_value);
        self
    }

    #[must_use]
    pub fn build(self) -> ParamSpec {
        ParamSpecFraction::new(
            self.name,
            self.nick.unwrap_or(self.name),
            self.blurb.unwrap_or(self.name),
            self.minimum
                .unwrap_or_else(|| crate::Fraction::new(-i32::MAX, 1)),
            self.maximum
                .unwrap_or_else(|| crate::Fraction::new(i32::MAX, 1)),
            self.default_value
                .unwrap_or_else(|| crate::Fraction::new(0, 1)),
            self.flags,
        )
    }
}

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GstParamSpecArray")]
    pub struct ParamSpecArray(Shared<ffi::GstParamSpecArray>);

    match fn {
        ref => |ptr| gobject_ffi::g_param_spec_ref_sink(ptr as *mut gobject_ffi::GParamSpec),
        unref => |ptr| gobject_ffi::g_param_spec_unref(ptr as *mut gobject_ffi::GParamSpec),
        type_ => || ffi::gst_param_spec_fraction_get_type(),
    }
}

unsafe impl Send for ParamSpecArray {}
unsafe impl Sync for ParamSpecArray {}

impl std::ops::Deref for ParamSpecArray {
    type Target = ParamSpec;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const ParamSpecArray as *const ParamSpec) }
    }
}

unsafe impl glib::ParamSpecType for ParamSpecArray {}

#[doc(hidden)]
impl FromGlibPtrFull<*mut gobject_ffi::GParamSpec> for ParamSpecArray {
    unsafe fn from_glib_full(ptr: *mut gobject_ffi::GParamSpec) -> Self {
        from_glib_full(ptr as *mut ffi::GstParamSpecArray)
    }
}

impl ParamSpecArray {
    pub fn builder(name: &str) -> ParamSpecArrayBuilder {
        assert_initialized_main_thread!();
        ParamSpecArrayBuilder::new(name)
    }

    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "gst_param_spec_array")]
    pub fn new(
        name: &str,
        nick: &str,
        blurb: &str,
        element_spec: Option<&glib::ParamSpec>,
        flags: glib::ParamFlags,
    ) -> glib::ParamSpec {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_param_spec_array(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                element_spec.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    pub fn element_spec(&self) -> Option<ParamSpec> {
        unsafe {
            let ptr = self.to_glib_none().0;

            from_glib_none((*ptr).element_spec)
        }
    }

    pub fn upcast(self) -> ParamSpec {
        unsafe { from_glib_full(self.to_glib_full() as *mut gobject_ffi::GParamSpec) }
    }

    pub fn upcast_ref(&self) -> &ParamSpec {
        &*self
    }
}

#[derive(Default)]
#[must_use]
pub struct ParamSpecArrayBuilder<'a> {
    name: &'a str,
    nick: Option<&'a str>,
    blurb: Option<&'a str>,
    flags: glib::ParamFlags,
    element_spec: Option<&'a glib::ParamSpec>,
}

impl<'a> ParamSpecArrayBuilder<'a> {
    fn new(name: &'a str) -> Self {
        assert_initialized_main_thread!();
        Self {
            name,
            ..Default::default()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Default: `self.name`
    pub fn nick(mut self, nick: &'a str) -> Self {
        self.nick = Some(nick);
        self
    }

    // rustdoc-stripper-ignore-next
    /// Default: `self.name`
    pub fn blurb(mut self, blurb: &'a str) -> Self {
        self.blurb = Some(blurb);
        self
    }

    // rustdoc-stripper-ignore-next
    /// Default: `glib::ParamFlags::READWRITE`
    pub fn flags(mut self, flags: glib::ParamFlags) -> Self {
        self.flags = flags;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Default: `None`
    pub fn element_spec(mut self, element_spec: &'a glib::ParamSpec) -> Self {
        self.element_spec = Some(element_spec);
        self
    }

    #[must_use]
    pub fn build(self) -> ParamSpec {
        ParamSpecArray::new(
            self.name,
            self.nick.unwrap_or(self.name),
            self.blurb.unwrap_or(self.name),
            self.element_spec,
            self.flags,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait() {
        crate::init().unwrap();

        let _pspec = ParamSpecFraction::new(
            "foo",
            "Foo",
            "Foo Bar",
            (0, 1).into(),
            (100, 1).into(),
            (1, 1).into(),
            glib::ParamFlags::READWRITE,
        );
    }
}
