// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RTSPToken;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct RTSPAuth(Object<ffi::GstRTSPAuth, ffi::GstRTSPAuthClass>);

    match fn {
        get_type => || ffi::gst_rtsp_auth_get_type(),
    }
}

impl RTSPAuth {
    pub fn new() -> RTSPAuth {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_rtsp_auth_new()) }
    }

    pub fn check(check: &str) -> Result<(), glib::error::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::gst_rtsp_auth_check(check.to_glib_none().0),
                "Check failed"
            )
        }
    }

    pub fn make_basic(user: &str, pass: &str) -> glib::GString {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_auth_make_basic(
                user.to_glib_none().0,
                pass.to_glib_none().0,
            ))
        }
    }
}

impl Default for RTSPAuth {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPAuth {}
unsafe impl Sync for RTSPAuth {}

pub const NONE_RTSP_AUTH: Option<&RTSPAuth> = None;

pub trait RTSPAuthExt: 'static {
    fn add_basic(&self, basic: &str, token: &RTSPToken);

    #[cfg(any(feature = "v1_12", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_12")))]
    fn add_digest(&self, user: &str, pass: &str, token: &RTSPToken);

    fn get_default_token(&self) -> Option<RTSPToken>;

    #[cfg(any(feature = "v1_16", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_16")))]
    fn get_realm(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v1_12", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_12")))]
    fn get_supported_methods(&self) -> gst_rtsp::RTSPAuthMethod;

    fn get_tls_authentication_mode(&self) -> gio::TlsAuthenticationMode;

    fn get_tls_certificate(&self) -> Option<gio::TlsCertificate>;

    fn get_tls_database(&self) -> Option<gio::TlsDatabase>;

    #[cfg(any(feature = "v1_16", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_16")))]
    fn parse_htdigest<P: AsRef<std::path::Path>>(&self, path: P, token: &RTSPToken) -> bool;

    fn remove_basic(&self, basic: &str);

    #[cfg(any(feature = "v1_12", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_12")))]
    fn remove_digest(&self, user: &str);

    #[cfg(any(feature = "v1_16", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_16")))]
    fn set_realm(&self, realm: &str);

    #[cfg(any(feature = "v1_12", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_12")))]
    fn set_supported_methods(&self, methods: gst_rtsp::RTSPAuthMethod);

    fn set_tls_authentication_mode(&self, mode: gio::TlsAuthenticationMode);

    fn set_tls_certificate<P: IsA<gio::TlsCertificate>>(&self, cert: Option<&P>);

    fn set_tls_database<P: IsA<gio::TlsDatabase>>(&self, database: Option<&P>);

    fn connect_accept_certificate<
        F: Fn(&Self, &gio::TlsConnection, &gio::TlsCertificate, gio::TlsCertificateFlags) -> bool
            + Send
            + Sync
            + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<RTSPAuth>> RTSPAuthExt for O {
    fn add_basic(&self, basic: &str, token: &RTSPToken) {
        unsafe {
            ffi::gst_rtsp_auth_add_basic(
                self.as_ref().to_glib_none().0,
                basic.to_glib_none().0,
                token.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_12", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_12")))]
    fn add_digest(&self, user: &str, pass: &str, token: &RTSPToken) {
        unsafe {
            ffi::gst_rtsp_auth_add_digest(
                self.as_ref().to_glib_none().0,
                user.to_glib_none().0,
                pass.to_glib_none().0,
                token.to_glib_none().0,
            );
        }
    }

    fn get_default_token(&self) -> Option<RTSPToken> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_auth_get_default_token(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_16")))]
    fn get_realm(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::gst_rtsp_auth_get_realm(self.as_ref().to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_12", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_12")))]
    fn get_supported_methods(&self) -> gst_rtsp::RTSPAuthMethod {
        unsafe {
            from_glib(ffi::gst_rtsp_auth_get_supported_methods(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_tls_authentication_mode(&self) -> gio::TlsAuthenticationMode {
        unsafe {
            from_glib(ffi::gst_rtsp_auth_get_tls_authentication_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_tls_certificate(&self) -> Option<gio::TlsCertificate> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_auth_get_tls_certificate(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_tls_database(&self) -> Option<gio::TlsDatabase> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_auth_get_tls_database(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_16")))]
    fn parse_htdigest<P: AsRef<std::path::Path>>(&self, path: P, token: &RTSPToken) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_auth_parse_htdigest(
                self.as_ref().to_glib_none().0,
                path.as_ref().to_glib_none().0,
                token.to_glib_none().0,
            ))
        }
    }

    fn remove_basic(&self, basic: &str) {
        unsafe {
            ffi::gst_rtsp_auth_remove_basic(self.as_ref().to_glib_none().0, basic.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_12", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_12")))]
    fn remove_digest(&self, user: &str) {
        unsafe {
            ffi::gst_rtsp_auth_remove_digest(self.as_ref().to_glib_none().0, user.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_16", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_16")))]
    fn set_realm(&self, realm: &str) {
        unsafe {
            ffi::gst_rtsp_auth_set_realm(self.as_ref().to_glib_none().0, realm.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_12", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_12")))]
    fn set_supported_methods(&self, methods: gst_rtsp::RTSPAuthMethod) {
        unsafe {
            ffi::gst_rtsp_auth_set_supported_methods(
                self.as_ref().to_glib_none().0,
                methods.to_glib(),
            );
        }
    }

    fn set_tls_authentication_mode(&self, mode: gio::TlsAuthenticationMode) {
        unsafe {
            ffi::gst_rtsp_auth_set_tls_authentication_mode(
                self.as_ref().to_glib_none().0,
                mode.to_glib(),
            );
        }
    }

    fn set_tls_certificate<P: IsA<gio::TlsCertificate>>(&self, cert: Option<&P>) {
        unsafe {
            ffi::gst_rtsp_auth_set_tls_certificate(
                self.as_ref().to_glib_none().0,
                cert.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_tls_database<P: IsA<gio::TlsDatabase>>(&self, database: Option<&P>) {
        unsafe {
            ffi::gst_rtsp_auth_set_tls_database(
                self.as_ref().to_glib_none().0,
                database.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn connect_accept_certificate<
        F: Fn(&Self, &gio::TlsConnection, &gio::TlsCertificate, gio::TlsCertificateFlags) -> bool
            + Send
            + Sync
            + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn accept_certificate_trampoline<
            P,
            F: Fn(&P, &gio::TlsConnection, &gio::TlsCertificate, gio::TlsCertificateFlags) -> bool
                + Send
                + Sync
                + 'static,
        >(
            this: *mut ffi::GstRTSPAuth,
            connection: *mut gio::ffi::GTlsConnection,
            peer_cert: *mut gio::ffi::GTlsCertificate,
            errors: gio::ffi::GTlsCertificateFlags,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean
        where
            P: IsA<RTSPAuth>,
        {
            let f: &F = &*(f as *const F);
            f(
                &RTSPAuth::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(connection),
                &from_glib_borrow(peer_cert),
                from_glib(errors),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accept-certificate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    accept_certificate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
