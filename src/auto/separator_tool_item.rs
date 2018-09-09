// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use ToolItem;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct SeparatorToolItem(Object<ffi::GtkSeparatorToolItem, ffi::GtkSeparatorToolItemClass>): ToolItem, Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_separator_tool_item_get_type(),
    }
}

impl SeparatorToolItem {
    pub fn new() -> SeparatorToolItem {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_separator_tool_item_new()).downcast_unchecked()
        }
    }
}

impl Default for SeparatorToolItem {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SeparatorToolItemExt {
    fn get_draw(&self) -> bool;

    fn set_draw(&self, draw: bool);

    fn connect_property_draw_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SeparatorToolItem> + IsA<glib::object::Object>> SeparatorToolItemExt for O {
    fn get_draw(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_separator_tool_item_get_draw(self.to_glib_none().0))
        }
    }

    fn set_draw(&self, draw: bool) {
        unsafe {
            ffi::gtk_separator_tool_item_set_draw(self.to_glib_none().0, draw.to_glib());
        }
    }

    fn connect_property_draw_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::draw",
                transmute(notify_draw_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_draw_trampoline<P>(this: *mut ffi::GtkSeparatorToolItem, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SeparatorToolItem> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SeparatorToolItem::from_glib_borrow(this).downcast_unchecked())
}
