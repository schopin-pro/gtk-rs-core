// This file was generated by gir (38add47) from gir-files (469db10)
// DO NOT EDIT

use Widget;
use ffi;
use gdk;
use gdk_pixbuf;
use gio;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Tooltip(Object<ffi::GtkTooltip>);

    match fn {
        get_type => || ffi::gtk_tooltip_get_type(),
    }
}

impl Tooltip {
    pub fn trigger_tooltip_query(display: &gdk::Display) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_tooltip_trigger_tooltip_query(display.to_glib_none().0);
        }
    }
}

pub trait TooltipExt {
    fn set_custom<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, custom_widget: Q);

    fn set_icon<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, pixbuf: P);

    fn set_icon_from_gicon<'a, P: IsA<gio::Icon> + 'a, Q: Into<Option<&'a P>>>(&self, gicon: Q, size: i32);

    fn set_icon_from_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P, size: i32);

    fn set_icon_from_stock<'a, P: Into<Option<&'a str>>>(&self, stock_id: P, size: i32);

    fn set_markup<'a, P: Into<Option<&'a str>>>(&self, markup: P);

    fn set_text<'a, P: Into<Option<&'a str>>>(&self, text: P);

    fn set_tip_area(&self, rect: &gdk::Rectangle);
}

impl<O: IsA<Tooltip>> TooltipExt for O {
    fn set_custom<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, custom_widget: Q) {
        let custom_widget = custom_widget.into();
        let custom_widget = custom_widget.to_glib_none();
        unsafe {
            ffi::gtk_tooltip_set_custom(self.to_glib_none().0, custom_widget.0);
        }
    }

    fn set_icon<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, pixbuf: P) {
        let pixbuf = pixbuf.into();
        let pixbuf = pixbuf.to_glib_none();
        unsafe {
            ffi::gtk_tooltip_set_icon(self.to_glib_none().0, pixbuf.0);
        }
    }

    fn set_icon_from_gicon<'a, P: IsA<gio::Icon> + 'a, Q: Into<Option<&'a P>>>(&self, gicon: Q, size: i32) {
        let gicon = gicon.into();
        let gicon = gicon.to_glib_none();
        unsafe {
            ffi::gtk_tooltip_set_icon_from_gicon(self.to_glib_none().0, gicon.0, size);
        }
    }

    fn set_icon_from_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P, size: i32) {
        let icon_name = icon_name.into();
        let icon_name = icon_name.to_glib_none();
        unsafe {
            ffi::gtk_tooltip_set_icon_from_icon_name(self.to_glib_none().0, icon_name.0, size);
        }
    }

    fn set_icon_from_stock<'a, P: Into<Option<&'a str>>>(&self, stock_id: P, size: i32) {
        let stock_id = stock_id.into();
        let stock_id = stock_id.to_glib_none();
        unsafe {
            ffi::gtk_tooltip_set_icon_from_stock(self.to_glib_none().0, stock_id.0, size);
        }
    }

    fn set_markup<'a, P: Into<Option<&'a str>>>(&self, markup: P) {
        let markup = markup.into();
        let markup = markup.to_glib_none();
        unsafe {
            ffi::gtk_tooltip_set_markup(self.to_glib_none().0, markup.0);
        }
    }

    fn set_text<'a, P: Into<Option<&'a str>>>(&self, text: P) {
        let text = text.into();
        let text = text.to_glib_none();
        unsafe {
            ffi::gtk_tooltip_set_text(self.to_glib_none().0, text.0);
        }
    }

    fn set_tip_area(&self, rect: &gdk::Rectangle) {
        unsafe {
            ffi::gtk_tooltip_set_tip_area(self.to_glib_none().0, rect.to_glib_none().0);
        }
    }
}
