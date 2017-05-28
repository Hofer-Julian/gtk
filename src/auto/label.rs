// This file was generated by gir (5c017c9) from gir-files (71d73f0)
// DO NOT EDIT

use Justification;
use Menu;
use Misc;
use MovementStep;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use pango;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct Label(Object<ffi::GtkLabel>): Misc, Widget;

    match fn {
        get_type => || ffi::gtk_label_get_type(),
    }
}

impl Label {
    pub fn new<'a, P: Into<Option<&'a str>>>(str: P) -> Label {
        assert_initialized_main_thread!();
        let str = str.into();
        let str = str.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_label_new(str.0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic<'a, P: Into<Option<&'a str>>>(str: P) -> Label {
        assert_initialized_main_thread!();
        let str = str.into();
        let str = str.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_label_new_with_mnemonic(str.0)).downcast_unchecked()
        }
    }
}

pub trait LabelExt {
    fn get_angle(&self) -> f64;

    //fn get_attributes(&self) -> /*Ignored*/Option<pango::AttrList>;

    fn get_current_uri(&self) -> Option<String>;

    fn get_ellipsize(&self) -> pango::EllipsizeMode;

    fn get_justify(&self) -> Justification;

    fn get_label(&self) -> Option<String>;

    fn get_layout(&self) -> Option<pango::Layout>;

    fn get_layout_offsets(&self) -> (i32, i32);

    fn get_line_wrap(&self) -> bool;

    fn get_line_wrap_mode(&self) -> pango::WrapMode;

    #[cfg(feature = "v3_10")]
    fn get_lines(&self) -> i32;

    fn get_max_width_chars(&self) -> i32;

    fn get_mnemonic_keyval(&self) -> u32;

    fn get_mnemonic_widget(&self) -> Option<Widget>;

    fn get_selectable(&self) -> bool;

    fn get_selection_bounds(&self) -> Option<(i32, i32)>;

    fn get_single_line_mode(&self) -> bool;

    fn get_text(&self) -> Option<String>;

    fn get_track_visited_links(&self) -> bool;

    fn get_use_markup(&self) -> bool;

    fn get_use_underline(&self) -> bool;

    fn get_width_chars(&self) -> i32;

    #[cfg(feature = "v3_16")]
    fn get_xalign(&self) -> f32;

    #[cfg(feature = "v3_16")]
    fn get_yalign(&self) -> f32;

    fn select_region(&self, start_offset: i32, end_offset: i32);

    fn set_angle(&self, angle: f64);

    //fn set_attributes<'a, P: Into<Option<&'a /*Ignored*/pango::AttrList>>>(&self, attrs: P);

    fn set_ellipsize(&self, mode: pango::EllipsizeMode);

    fn set_justify(&self, jtype: Justification);

    fn set_label(&self, str: &str);

    fn set_line_wrap(&self, wrap: bool);

    fn set_line_wrap_mode(&self, wrap_mode: pango::WrapMode);

    #[cfg(feature = "v3_10")]
    fn set_lines(&self, lines: i32);

    fn set_markup(&self, str: &str);

    fn set_markup_with_mnemonic(&self, str: &str);

    fn set_max_width_chars(&self, n_chars: i32);

    fn set_mnemonic_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, widget: Q);

    fn set_pattern(&self, pattern: &str);

    fn set_selectable(&self, setting: bool);

    fn set_single_line_mode(&self, single_line_mode: bool);

    fn set_text(&self, str: &str);

    fn set_text_with_mnemonic(&self, str: &str);

    fn set_track_visited_links(&self, track_links: bool);

    fn set_use_markup(&self, setting: bool);

    fn set_use_underline(&self, setting: bool);

    fn set_width_chars(&self, n_chars: i32);

    #[cfg(feature = "v3_16")]
    fn set_xalign(&self, xalign: f32);

    #[cfg(feature = "v3_16")]
    fn set_yalign(&self, yalign: f32);

    fn get_property_cursor_position(&self) -> i32;

    fn get_property_selection_bound(&self) -> i32;

    fn get_property_wrap(&self) -> bool;

    fn set_property_wrap(&self, wrap: bool);

    fn get_property_wrap_mode(&self) -> pango::WrapMode;

    fn set_property_wrap_mode(&self, wrap_mode: pango::WrapMode);

    fn connect_activate_current_link<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_activate_link<F: Fn(&Self, &str) -> Inhibit + 'static>(&self, f: F) -> u64;

    fn connect_copy_clipboard<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32, bool) + 'static>(&self, f: F) -> u64;

    fn connect_populate_popup<F: Fn(&Self, &Menu) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Label> + IsA<glib::object::Object>> LabelExt for O {
    fn get_angle(&self) -> f64 {
        unsafe {
            ffi::gtk_label_get_angle(self.to_glib_none().0)
        }
    }

    //fn get_attributes(&self) -> /*Ignored*/Option<pango::AttrList> {
    //    unsafe { TODO: call ffi::gtk_label_get_attributes() }
    //}

    fn get_current_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_current_uri(self.to_glib_none().0))
        }
    }

    fn get_ellipsize(&self) -> pango::EllipsizeMode {
        unsafe {
            from_glib(ffi::gtk_label_get_ellipsize(self.to_glib_none().0))
        }
    }

    fn get_justify(&self) -> Justification {
        unsafe {
            from_glib(ffi::gtk_label_get_justify(self.to_glib_none().0))
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_label(self.to_glib_none().0))
        }
    }

    fn get_layout(&self) -> Option<pango::Layout> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_layout(self.to_glib_none().0))
        }
    }

    fn get_layout_offsets(&self) -> (i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            ffi::gtk_label_get_layout_offsets(self.to_glib_none().0, &mut x, &mut y);
            (x, y)
        }
    }

    fn get_line_wrap(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_line_wrap(self.to_glib_none().0))
        }
    }

    fn get_line_wrap_mode(&self) -> pango::WrapMode {
        unsafe {
            from_glib(ffi::gtk_label_get_line_wrap_mode(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_lines(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_lines(self.to_glib_none().0)
        }
    }

    fn get_max_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_max_width_chars(self.to_glib_none().0)
        }
    }

    fn get_mnemonic_keyval(&self) -> u32 {
        unsafe {
            ffi::gtk_label_get_mnemonic_keyval(self.to_glib_none().0)
        }
    }

    fn get_mnemonic_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_mnemonic_widget(self.to_glib_none().0))
        }
    }

    fn get_selectable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_selectable(self.to_glib_none().0))
        }
    }

    fn get_selection_bounds(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut start = mem::uninitialized();
            let mut end = mem::uninitialized();
            let ret = from_glib(ffi::gtk_label_get_selection_bounds(self.to_glib_none().0, &mut start, &mut end));
            if ret { Some((start, end)) } else { None }
        }
    }

    fn get_single_line_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_single_line_mode(self.to_glib_none().0))
        }
    }

    fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_text(self.to_glib_none().0))
        }
    }

    fn get_track_visited_links(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_track_visited_links(self.to_glib_none().0))
        }
    }

    fn get_use_markup(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_use_markup(self.to_glib_none().0))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_use_underline(self.to_glib_none().0))
        }
    }

    fn get_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_width_chars(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_xalign(&self) -> f32 {
        unsafe {
            ffi::gtk_label_get_xalign(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_yalign(&self) -> f32 {
        unsafe {
            ffi::gtk_label_get_yalign(self.to_glib_none().0)
        }
    }

    fn select_region(&self, start_offset: i32, end_offset: i32) {
        unsafe {
            ffi::gtk_label_select_region(self.to_glib_none().0, start_offset, end_offset);
        }
    }

    fn set_angle(&self, angle: f64) {
        unsafe {
            ffi::gtk_label_set_angle(self.to_glib_none().0, angle);
        }
    }

    //fn set_attributes<'a, P: Into<Option<&'a /*Ignored*/pango::AttrList>>>(&self, attrs: P) {
    //    unsafe { TODO: call ffi::gtk_label_set_attributes() }
    //}

    fn set_ellipsize(&self, mode: pango::EllipsizeMode) {
        unsafe {
            ffi::gtk_label_set_ellipsize(self.to_glib_none().0, mode.to_glib());
        }
    }

    fn set_justify(&self, jtype: Justification) {
        unsafe {
            ffi::gtk_label_set_justify(self.to_glib_none().0, jtype.to_glib());
        }
    }

    fn set_label(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_label(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    fn set_line_wrap(&self, wrap: bool) {
        unsafe {
            ffi::gtk_label_set_line_wrap(self.to_glib_none().0, wrap.to_glib());
        }
    }

    fn set_line_wrap_mode(&self, wrap_mode: pango::WrapMode) {
        unsafe {
            ffi::gtk_label_set_line_wrap_mode(self.to_glib_none().0, wrap_mode.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_lines(&self, lines: i32) {
        unsafe {
            ffi::gtk_label_set_lines(self.to_glib_none().0, lines);
        }
    }

    fn set_markup(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_markup(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    fn set_markup_with_mnemonic(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_markup_with_mnemonic(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    fn set_max_width_chars(&self, n_chars: i32) {
        unsafe {
            ffi::gtk_label_set_max_width_chars(self.to_glib_none().0, n_chars);
        }
    }

    fn set_mnemonic_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, widget: Q) {
        let widget = widget.into();
        let widget = widget.to_glib_none();
        unsafe {
            ffi::gtk_label_set_mnemonic_widget(self.to_glib_none().0, widget.0);
        }
    }

    fn set_pattern(&self, pattern: &str) {
        unsafe {
            ffi::gtk_label_set_pattern(self.to_glib_none().0, pattern.to_glib_none().0);
        }
    }

    fn set_selectable(&self, setting: bool) {
        unsafe {
            ffi::gtk_label_set_selectable(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_single_line_mode(&self, single_line_mode: bool) {
        unsafe {
            ffi::gtk_label_set_single_line_mode(self.to_glib_none().0, single_line_mode.to_glib());
        }
    }

    fn set_text(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_text(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    fn set_text_with_mnemonic(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_text_with_mnemonic(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    fn set_track_visited_links(&self, track_links: bool) {
        unsafe {
            ffi::gtk_label_set_track_visited_links(self.to_glib_none().0, track_links.to_glib());
        }
    }

    fn set_use_markup(&self, setting: bool) {
        unsafe {
            ffi::gtk_label_set_use_markup(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_use_underline(&self, setting: bool) {
        unsafe {
            ffi::gtk_label_set_use_underline(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_width_chars(&self, n_chars: i32) {
        unsafe {
            ffi::gtk_label_set_width_chars(self.to_glib_none().0, n_chars);
        }
    }

    #[cfg(feature = "v3_16")]
    fn set_xalign(&self, xalign: f32) {
        unsafe {
            ffi::gtk_label_set_xalign(self.to_glib_none().0, xalign);
        }
    }

    #[cfg(feature = "v3_16")]
    fn set_yalign(&self, yalign: f32) {
        unsafe {
            ffi::gtk_label_set_yalign(self.to_glib_none().0, yalign);
        }
    }

    fn get_property_cursor_position(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "cursor-position".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_selection_bound(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "selection-bound".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_wrap(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "wrap".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_wrap(&self, wrap: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "wrap".to_glib_none().0, Value::from(&wrap).to_glib_none().0);
        }
    }

    fn get_property_wrap_mode(&self) -> pango::WrapMode {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "wrap-mode".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_property_wrap_mode(&self, wrap_mode: pango::WrapMode) {
        let wrap_mode = wrap_mode.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "wrap-mode".to_glib_none().0, Value::from(&wrap_mode).to_glib_none().0);
        }
    }

    fn connect_activate_current_link<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-current-link",
                transmute(activate_current_link_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_activate_link<F: Fn(&Self, &str) -> Inhibit + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-link",
                transmute(activate_link_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_copy_clipboard<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "copy-clipboard",
                transmute(copy_clipboard_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32, bool) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, MovementStep, i32, bool) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-cursor",
                transmute(move_cursor_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_populate_popup<F: Fn(&Self, &Menu) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Menu) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "populate-popup",
                transmute(populate_popup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_current_link_trampoline<P>(this: *mut ffi::GtkLabel, f: glib_ffi::gpointer)
where P: IsA<Label> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Label::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn activate_link_trampoline<P>(this: *mut ffi::GtkLabel, uri: *mut libc::c_char, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Label> {
    callback_guard!();
    let f: &Box_<Fn(&P, &str) -> Inhibit + 'static> = transmute(f);
    f(&Label::from_glib_none(this).downcast_unchecked(), &String::from_glib_none(uri)).to_glib()
}

unsafe extern "C" fn copy_clipboard_trampoline<P>(this: *mut ffi::GtkLabel, f: glib_ffi::gpointer)
where P: IsA<Label> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Label::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn move_cursor_trampoline<P>(this: *mut ffi::GtkLabel, step: ffi::GtkMovementStep, count: libc::c_int, extend_selection: glib_ffi::gboolean, f: glib_ffi::gpointer)
where P: IsA<Label> {
    callback_guard!();
    let f: &Box_<Fn(&P, MovementStep, i32, bool) + 'static> = transmute(f);
    f(&Label::from_glib_none(this).downcast_unchecked(), from_glib(step), count, from_glib(extend_selection))
}

unsafe extern "C" fn populate_popup_trampoline<P>(this: *mut ffi::GtkLabel, menu: *mut ffi::GtkMenu, f: glib_ffi::gpointer)
where P: IsA<Label> {
    callback_guard!();
    let f: &Box_<Fn(&P, &Menu) + 'static> = transmute(f);
    f(&Label::from_glib_none(this).downcast_unchecked(), &from_glib_none(menu))
}
