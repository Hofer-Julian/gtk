// This file was generated by gir (5c017c9) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use Container;
use Dialog;
use MessageType;
use Widget;
use Window;
use ffi;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use std::mem::transmute;

glib_wrapper! {
    pub struct MessageDialog(Object<ffi::GtkMessageDialog>): Dialog, Window, Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_message_dialog_get_type(),
    }
}

impl MessageDialog {
    //pub fn new<'a, 'b, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(parent: Q, flags: DialogFlags, type_: MessageType, buttons: ButtonsType, message_format: R, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> MessageDialog {
    //    unsafe { TODO: call ffi::gtk_message_dialog_new() }
    //}

    //pub fn new_with_markup<'a, 'b, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(parent: Q, flags: DialogFlags, type_: MessageType, buttons: ButtonsType, message_format: R, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> MessageDialog {
    //    unsafe { TODO: call ffi::gtk_message_dialog_new_with_markup() }
    //}
}

pub trait MessageDialogExt {
    //fn format_secondary_markup(&self, message_format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn format_secondary_text<'a, P: Into<Option<&'a str>>>(&self, message_format: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn get_image(&self) -> Option<Widget>;

    fn get_message_area(&self) -> Option<Widget>;

    fn set_image<P: IsA<Widget>>(&self, image: &P);

    fn set_markup(&self, str: &str);

    fn get_property_message_type(&self) -> MessageType;

    fn set_property_message_type(&self, message_type: MessageType);

    fn get_property_secondary_text(&self) -> Option<String>;

    fn set_property_secondary_text(&self, secondary_text: Option<&str>);

    fn get_property_secondary_use_markup(&self) -> bool;

    fn set_property_secondary_use_markup(&self, secondary_use_markup: bool);

    fn get_property_text(&self) -> Option<String>;

    fn set_property_text(&self, text: Option<&str>);

    fn get_property_use_markup(&self) -> bool;

    fn set_property_use_markup(&self, use_markup: bool);
}

impl<O: IsA<MessageDialog> + IsA<glib::object::Object>> MessageDialogExt for O {
    //fn format_secondary_markup(&self, message_format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_message_dialog_format_secondary_markup() }
    //}

    //fn format_secondary_text<'a, P: Into<Option<&'a str>>>(&self, message_format: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_message_dialog_format_secondary_text() }
    //}

    fn get_image(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_message_dialog_get_image(self.to_glib_none().0))
        }
    }

    fn get_message_area(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_message_dialog_get_message_area(self.to_glib_none().0))
        }
    }

    fn set_image<P: IsA<Widget>>(&self, image: &P) {
        unsafe {
            ffi::gtk_message_dialog_set_image(self.to_glib_none().0, image.to_glib_none().0);
        }
    }

    fn set_markup(&self, str: &str) {
        unsafe {
            ffi::gtk_message_dialog_set_markup(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    fn get_property_message_type(&self) -> MessageType {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "message-type".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_property_message_type(&self, message_type: MessageType) {
        let message_type = message_type.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "message-type".to_glib_none().0, Value::from(&message_type).to_glib_none().0);
        }
    }

    fn get_property_secondary_text(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "secondary-text".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_secondary_text(&self, secondary_text: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "secondary-text".to_glib_none().0, Value::from(secondary_text).to_glib_none().0);
        }
    }

    fn get_property_secondary_use_markup(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "secondary-use-markup".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_secondary_use_markup(&self, secondary_use_markup: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "secondary-use-markup".to_glib_none().0, Value::from(&secondary_use_markup).to_glib_none().0);
        }
    }

    fn get_property_text(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "text".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text".to_glib_none().0, Value::from(text).to_glib_none().0);
        }
    }

    fn get_property_use_markup(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "use-markup".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_use_markup(&self, use_markup: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "use-markup".to_glib_none().0, Value::from(&use_markup).to_glib_none().0);
        }
    }
}
