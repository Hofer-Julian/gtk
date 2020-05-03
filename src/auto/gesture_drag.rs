// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use EventController;
use Gesture;
use GestureSingle;
use PropagationPhase;
use Widget;

glib_wrapper! {
    pub struct GestureDrag(Object<gtk_sys::GtkGestureDrag, gtk_sys::GtkGestureDragClass, GestureDragClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        get_type => || gtk_sys::gtk_gesture_drag_get_type(),
    }
}

impl GestureDrag {
    pub fn new<P: IsA<Widget>>(widget: &P) -> GestureDrag {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(gtk_sys::gtk_gesture_drag_new(
                widget.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

#[derive(Clone, Default)]
pub struct GestureDragBuilder {
    button: Option<u32>,
    exclusive: Option<bool>,
    touch_only: Option<bool>,
    n_points: Option<u32>,
    window: Option<gdk::Window>,
    propagation_phase: Option<PropagationPhase>,
    widget: Option<Widget>,
}

impl GestureDragBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> GestureDrag {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref button) = self.button {
            properties.push(("button", button));
        }
        if let Some(ref exclusive) = self.exclusive {
            properties.push(("exclusive", exclusive));
        }
        if let Some(ref touch_only) = self.touch_only {
            properties.push(("touch-only", touch_only));
        }
        if let Some(ref n_points) = self.n_points {
            properties.push(("n-points", n_points));
        }
        if let Some(ref window) = self.window {
            properties.push(("window", window));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        if let Some(ref widget) = self.widget {
            properties.push(("widget", widget));
        }
        let ret = glib::Object::new(GestureDrag::static_type(), &properties)
            .expect("object new")
            .downcast::<GestureDrag>()
            .expect("downcast");
        ret
    }

    pub fn button(mut self, button: u32) -> Self {
        self.button = Some(button);
        self
    }

    pub fn exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = Some(exclusive);
        self
    }

    pub fn touch_only(mut self, touch_only: bool) -> Self {
        self.touch_only = Some(touch_only);
        self
    }

    pub fn n_points(mut self, n_points: u32) -> Self {
        self.n_points = Some(n_points);
        self
    }

    pub fn window<P: IsA<gdk::Window>>(mut self, window: &P) -> Self {
        self.window = Some(window.clone().upcast());
        self
    }

    pub fn propagation_phase(mut self, propagation_phase: PropagationPhase) -> Self {
        self.propagation_phase = Some(propagation_phase);
        self
    }

    pub fn widget<P: IsA<Widget>>(mut self, widget: &P) -> Self {
        self.widget = Some(widget.clone().upcast());
        self
    }
}

pub const NONE_GESTURE_DRAG: Option<&GestureDrag> = None;

pub trait GestureDragExt: 'static {
    fn get_offset(&self) -> Option<(f64, f64)>;

    fn get_start_point(&self) -> Option<(f64, f64)>;

    fn connect_drag_begin<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_drag_end<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_drag_update<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GestureDrag>> GestureDragExt for O {
    fn get_offset(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let ret = from_glib(gtk_sys::gtk_gesture_drag_get_offset(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            ));
            let x = x.assume_init();
            let y = y.assume_init();
            if ret {
                Some((x, y))
            } else {
                None
            }
        }
    }

    fn get_start_point(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let ret = from_glib(gtk_sys::gtk_gesture_drag_get_start_point(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            ));
            let x = x.assume_init();
            let y = y.assume_init();
            if ret {
                Some((x, y))
            } else {
                None
            }
        }
    }

    fn connect_drag_begin<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn drag_begin_trampoline<P, F: Fn(&P, f64, f64) + 'static>(
            this: *mut gtk_sys::GtkGestureDrag,
            start_x: libc::c_double,
            start_y: libc::c_double,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GestureDrag>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GestureDrag::from_glib_borrow(this).unsafe_cast_ref(),
                start_x,
                start_y,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-begin\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drag_begin_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_drag_end<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn drag_end_trampoline<P, F: Fn(&P, f64, f64) + 'static>(
            this: *mut gtk_sys::GtkGestureDrag,
            offset_x: libc::c_double,
            offset_y: libc::c_double,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GestureDrag>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GestureDrag::from_glib_borrow(this).unsafe_cast_ref(),
                offset_x,
                offset_y,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drag_end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_drag_update<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn drag_update_trampoline<P, F: Fn(&P, f64, f64) + 'static>(
            this: *mut gtk_sys::GtkGestureDrag,
            offset_x: libc::c_double,
            offset_y: libc::c_double,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GestureDrag>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GestureDrag::from_glib_borrow(this).unsafe_cast_ref(),
                offset_x,
                offset_y,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-update\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drag_update_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for GestureDrag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GestureDrag")
    }
}
