// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{SocketConnectable, SocketFamily};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GSocketAddress")]
    pub struct SocketAddress(Object<ffi::GSocketAddress, ffi::GSocketAddressClass>) @implements SocketConnectable;

    match fn {
        type_ => || ffi::g_socket_address_get_type(),
    }
}

impl SocketAddress {
    pub const NONE: Option<&'static SocketAddress> = None;

    //#[doc(alias = "g_socket_address_new_from_native")]
    //#[doc(alias = "new_from_native")]
    //pub fn from_native(native: /*Unimplemented*/Basic: Pointer, len: usize) -> SocketAddress {
    //    unsafe { TODO: call ffi:g_socket_address_new_from_native() }
    //}
}

unsafe impl Send for SocketAddress {}
unsafe impl Sync for SocketAddress {}

pub trait SocketAddressExt: IsA<SocketAddress> + 'static {
    #[doc(alias = "g_socket_address_get_family")]
    #[doc(alias = "get_family")]
    fn family(&self) -> SocketFamily {
        unsafe {
            from_glib(ffi::g_socket_address_get_family(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_socket_address_get_native_size")]
    #[doc(alias = "get_native_size")]
    fn native_size(&self) -> isize {
        unsafe { ffi::g_socket_address_get_native_size(self.as_ref().to_glib_none().0) }
    }

    //#[doc(alias = "g_socket_address_to_native")]
    //fn to_native(&self, dest: /*Unimplemented*/Option<Basic: Pointer>, destlen: usize) -> Result<(), glib::Error> {
    //    unsafe { TODO: call ffi:g_socket_address_to_native() }
    //}

    #[doc(alias = "family")]
    fn connect_family_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_family_trampoline<
            P: IsA<SocketAddress>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GSocketAddress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SocketAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::family\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_family_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<SocketAddress>> SocketAddressExt for O {}

impl fmt::Display for SocketAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SocketAddress")
    }
}
