// This file was generated by gir (6bcd52a) from gir-files (1069259)
// DO NOT EDIT

use Box;
use Container;
use Orientable;
use RecentChooser;
use RecentManager;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct RecentChooserWidget(Object<ffi::GtkRecentChooserWidget>): Box, Container, Widget, Orientable, RecentChooser;

    match fn {
        get_type => || ffi::gtk_recent_chooser_widget_get_type(),
    }
}

impl RecentChooserWidget {
    pub fn new() -> RecentChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_recent_chooser_widget_new()).downcast_unchecked()
        }
    }

    pub fn new_for_manager(manager: &RecentManager) -> RecentChooserWidget {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_recent_chooser_widget_new_for_manager(manager.to_glib_none().0)).downcast_unchecked()
        }
    }
}

impl Default for RecentChooserWidget {
    fn default() -> Self {
        Self::new()
    }
}
