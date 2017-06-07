// This file was generated by gir (d121f7e) from gir-files (71d73f0)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureSingle;
#[cfg(feature = "v3_14")]
use Widget;
use ffi;
#[cfg(feature = "v3_14")]
use gdk;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v3_14")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v3_14")]
use glib_ffi;
#[cfg(feature = "v3_14")]
use libc;
#[cfg(feature = "v3_14")]
use std::boxed::Box as Box_;
#[cfg(feature = "v3_14")]
use std::mem::transmute;

glib_wrapper! {
    pub struct GestureMultiPress(Object<ffi::GtkGestureMultiPress>): GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_multi_press_get_type(),
    }
}

impl GestureMultiPress {
    #[cfg(feature = "v3_14")]
    pub fn new<P: IsA<Widget>>(widget: &P) -> GestureMultiPress {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_multi_press_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait GestureMultiPressExt {
    #[cfg(feature = "v3_14")]
    fn get_area(&self) -> Option<gdk::Rectangle>;

    #[cfg(feature = "v3_14")]
    fn set_area<'a, P: Into<Option<&'a gdk::Rectangle>>>(&self, rect: P);

    #[cfg(feature = "v3_14")]
    fn connect_pressed<F: Fn(&Self, i32, f64, f64) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_14")]
    fn connect_released<F: Fn(&Self, i32, f64, f64) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_14")]
    fn connect_stopped<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<GestureMultiPress> + IsA<glib::object::Object>> GestureMultiPressExt for O {
    #[cfg(feature = "v3_14")]
    fn get_area(&self) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_multi_press_get_area(self.to_glib_none().0, rect.to_glib_none_mut().0));
            if ret { Some(rect) } else { None }
        }
    }

    #[cfg(feature = "v3_14")]
    fn set_area<'a, P: Into<Option<&'a gdk::Rectangle>>>(&self, rect: P) {
        let rect = rect.into();
        let rect = rect.to_glib_none();
        unsafe {
            ffi::gtk_gesture_multi_press_set_area(self.to_glib_none().0, rect.0);
        }
    }

    #[cfg(feature = "v3_14")]
    fn connect_pressed<F: Fn(&Self, i32, f64, f64) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "pressed",
                transmute(pressed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    fn connect_released<F: Fn(&Self, i32, f64, f64) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "released",
                transmute(released_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    fn connect_stopped<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "stopped",
                transmute(stopped_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn pressed_trampoline<P>(this: *mut ffi::GtkGestureMultiPress, n_press: libc::c_int, x: libc::c_double, y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureMultiPress> {
    callback_guard!();
    let f: &Box_<Fn(&P, i32, f64, f64) + 'static> = transmute(f);
    f(&GestureMultiPress::from_glib_none(this).downcast_unchecked(), n_press, x, y)
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn released_trampoline<P>(this: *mut ffi::GtkGestureMultiPress, n_press: libc::c_int, x: libc::c_double, y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureMultiPress> {
    callback_guard!();
    let f: &Box_<Fn(&P, i32, f64, f64) + 'static> = transmute(f);
    f(&GestureMultiPress::from_glib_none(this).downcast_unchecked(), n_press, x, y)
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn stopped_trampoline<P>(this: *mut ffi::GtkGestureMultiPress, f: glib_ffi::gpointer)
where P: IsA<GestureMultiPress> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&GestureMultiPress::from_glib_none(this).downcast_unchecked())
}
