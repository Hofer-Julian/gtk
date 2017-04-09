// This file was generated by gir (bf7bd49) from gir-files (71d73f0)
// DO NOT EDIT

use AssistantPageType;
use Bin;
use Container;
use Widget;
use Window;
use ffi;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
#[cfg(feature = "v3_12")]
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Assistant(Object<ffi::GtkAssistant>): Window, Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_assistant_get_type(),
    }
}

impl Assistant {
    pub fn new() -> Assistant {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_assistant_new()).downcast_unchecked()
        }
    }

    pub fn add_action_widget<T: IsA<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_assistant_add_action_widget(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    pub fn append_page<T: IsA<Widget>>(&self, page: &T) -> i32 {
        unsafe {
            ffi::gtk_assistant_append_page(self.to_glib_none().0, page.to_glib_none().0)
        }
    }

    pub fn commit(&self) {
        unsafe {
            ffi::gtk_assistant_commit(self.to_glib_none().0);
        }
    }

    pub fn get_current_page(&self) -> i32 {
        unsafe {
            ffi::gtk_assistant_get_current_page(self.to_glib_none().0)
        }
    }

    pub fn get_n_pages(&self) -> i32 {
        unsafe {
            ffi::gtk_assistant_get_n_pages(self.to_glib_none().0)
        }
    }

    pub fn get_nth_page(&self, page_num: i32) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_assistant_get_nth_page(self.to_glib_none().0, page_num))
        }
    }

    pub fn get_page_complete<T: IsA<Widget>>(&self, page: &T) -> bool {
        unsafe {
            from_glib(ffi::gtk_assistant_get_page_complete(self.to_glib_none().0, page.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_18")]
    pub fn get_page_has_padding<T: IsA<Widget>>(&self, page: &T) -> bool {
        unsafe {
            from_glib(ffi::gtk_assistant_get_page_has_padding(self.to_glib_none().0, page.to_glib_none().0))
        }
    }

    pub fn get_page_title<T: IsA<Widget>>(&self, page: &T) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_assistant_get_page_title(self.to_glib_none().0, page.to_glib_none().0))
        }
    }

    pub fn get_page_type<T: IsA<Widget>>(&self, page: &T) -> AssistantPageType {
        unsafe {
            from_glib(ffi::gtk_assistant_get_page_type(self.to_glib_none().0, page.to_glib_none().0))
        }
    }

    pub fn insert_page<T: IsA<Widget>>(&self, page: &T, position: i32) -> i32 {
        unsafe {
            ffi::gtk_assistant_insert_page(self.to_glib_none().0, page.to_glib_none().0, position)
        }
    }

    pub fn next_page(&self) {
        unsafe {
            ffi::gtk_assistant_next_page(self.to_glib_none().0);
        }
    }

    pub fn prepend_page<T: IsA<Widget>>(&self, page: &T) -> i32 {
        unsafe {
            ffi::gtk_assistant_prepend_page(self.to_glib_none().0, page.to_glib_none().0)
        }
    }

    pub fn previous_page(&self) {
        unsafe {
            ffi::gtk_assistant_previous_page(self.to_glib_none().0);
        }
    }

    pub fn remove_action_widget<T: IsA<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_assistant_remove_action_widget(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    pub fn remove_page(&self, page_num: i32) {
        unsafe {
            ffi::gtk_assistant_remove_page(self.to_glib_none().0, page_num);
        }
    }

    pub fn set_current_page(&self, page_num: i32) {
        unsafe {
            ffi::gtk_assistant_set_current_page(self.to_glib_none().0, page_num);
        }
    }

    //pub fn set_forward_page_func(&self, page_func: /*Unknown conversion*//*Unimplemented*/AssistantPageFunc, data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_assistant_set_forward_page_func() }
    //}

    pub fn set_page_complete<T: IsA<Widget>>(&self, page: &T, complete: bool) {
        unsafe {
            ffi::gtk_assistant_set_page_complete(self.to_glib_none().0, page.to_glib_none().0, complete.to_glib());
        }
    }

    #[cfg(feature = "v3_18")]
    pub fn set_page_has_padding<T: IsA<Widget>>(&self, page: &T, has_padding: bool) {
        unsafe {
            ffi::gtk_assistant_set_page_has_padding(self.to_glib_none().0, page.to_glib_none().0, has_padding.to_glib());
        }
    }

    pub fn set_page_title<T: IsA<Widget>>(&self, page: &T, title: &str) {
        unsafe {
            ffi::gtk_assistant_set_page_title(self.to_glib_none().0, page.to_glib_none().0, title.to_glib_none().0);
        }
    }

    pub fn set_page_type<T: IsA<Widget>>(&self, page: &T, type_: AssistantPageType) {
        unsafe {
            ffi::gtk_assistant_set_page_type(self.to_glib_none().0, page.to_glib_none().0, type_.to_glib());
        }
    }

    pub fn update_buttons_state(&self) {
        unsafe {
            ffi::gtk_assistant_update_buttons_state(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn get_property_use_header_bar(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "use-header-bar".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn get_child_complete<T: IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "complete".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_child_complete<T: IsA<Widget>>(&self, item: &T, complete: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "complete".to_glib_none().0, Value::from(&complete).to_glib_none().0);
        }
    }

    pub fn get_child_has_padding<T: IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "has-padding".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_child_has_padding<T: IsA<Widget>>(&self, item: &T, has_padding: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "has-padding".to_glib_none().0, Value::from(&has_padding).to_glib_none().0);
        }
    }

    pub fn get_child_page_type<T: IsA<Widget>>(&self, item: &T) -> AssistantPageType {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "page-type".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    pub fn set_child_page_type<T: IsA<Widget>>(&self, item: &T, page_type: AssistantPageType) {
        let page_type = page_type.to_glib() as i32;
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "page-type".to_glib_none().0, Value::from(&page_type).to_glib_none().0);
        }
    }

    pub fn get_child_title<T: IsA<Widget>>(&self, item: &T) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "title".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_child_title<T: IsA<Widget>>(&self, item: &T, title: Option<&str>) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "title".to_glib_none().0, Value::from(title).to_glib_none().0);
        }
    }

    pub fn connect_apply<F: Fn(&Assistant) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Assistant) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "apply",
                transmute(apply_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_cancel<F: Fn(&Assistant) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Assistant) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cancel",
                transmute(cancel_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_close<F: Fn(&Assistant) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Assistant) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "close",
                transmute(close_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_escape<F: Fn(&Assistant) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Assistant) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "escape",
                transmute(escape_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_prepare<F: Fn(&Assistant, &Widget) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Assistant, &Widget) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "prepare",
                transmute(prepare_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn apply_trampoline(this: *mut ffi::GtkAssistant, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Assistant) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn cancel_trampoline(this: *mut ffi::GtkAssistant, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Assistant) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn close_trampoline(this: *mut ffi::GtkAssistant, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Assistant) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn escape_trampoline(this: *mut ffi::GtkAssistant, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Assistant) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn prepare_trampoline(this: *mut ffi::GtkAssistant, page: *mut ffi::GtkWidget, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Assistant, &Widget) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(page))
}
