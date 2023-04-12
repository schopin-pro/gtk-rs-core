// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Script;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Language(Boxed<ffi::PangoLanguage>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::pango_language_get_type(), ptr as *mut _) as *mut ffi::PangoLanguage,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::pango_language_get_type(), ptr as *mut _),
        type_ => || ffi::pango_language_get_type(),
    }
}

impl Language {
    #[doc(alias = "pango_language_get_sample_string")]
    #[doc(alias = "get_sample_string")]
    pub fn sample_string(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::pango_language_get_sample_string(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[doc(alias = "pango_language_includes_script")]
    pub fn includes_script(&self, script: Script) -> bool {
        unsafe {
            from_glib(ffi::pango_language_includes_script(
                mut_override(self.to_glib_none().0),
                script.into_glib(),
            ))
        }
    }

    #[doc(alias = "pango_language_matches")]
    pub fn matches(&self, range_list: &str) -> bool {
        unsafe {
            from_glib(ffi::pango_language_matches(
                mut_override(self.to_glib_none().0),
                range_list.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_language_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::pango_language_to_string(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[doc(alias = "pango_language_from_string")]
    pub fn from_string(language: &str) -> Language {
        unsafe { from_glib_none(ffi::pango_language_from_string(language.to_glib_none().0)) }
    }

    #[doc(alias = "pango_language_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Language {
        unsafe { from_glib_none(ffi::pango_language_get_default()) }
    }

    #[cfg(any(feature = "v1_48", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_48")))]
    #[doc(alias = "pango_language_get_preferred")]
    #[doc(alias = "get_preferred")]
    pub fn preferred() -> Vec<Language> {
        unsafe { FromGlibPtrContainer::from_glib_none(ffi::pango_language_get_preferred()) }
    }
}

impl fmt::Display for Language {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}
