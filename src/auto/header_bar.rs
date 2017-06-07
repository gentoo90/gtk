// This file was generated by gir (d121f7e) from gir-files (71d73f0)
// DO NOT EDIT

use Container;
use PackType;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use std::mem::transmute;

glib_wrapper! {
    pub struct HeaderBar(Object<ffi::GtkHeaderBar>): Container, Widget;

    match fn {
        get_type => || ffi::gtk_header_bar_get_type(),
    }
}

impl HeaderBar {
    #[cfg(feature = "v3_10")]
    pub fn new() -> HeaderBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_header_bar_new()).downcast_unchecked()
        }
    }
}

pub trait HeaderBarExt {
    #[cfg(feature = "v3_10")]
    fn get_custom_title(&self) -> Option<Widget>;

    #[cfg(feature = "v3_12")]
    fn get_decoration_layout(&self) -> Option<String>;

    #[cfg(feature = "v3_12")]
    fn get_has_subtitle(&self) -> bool;

    #[cfg(feature = "v3_10")]
    fn get_show_close_button(&self) -> bool;

    #[cfg(feature = "v3_10")]
    fn get_subtitle(&self) -> Option<String>;

    #[cfg(feature = "v3_10")]
    fn get_title(&self) -> Option<String>;

    #[cfg(feature = "v3_10")]
    fn pack_end<P: IsA<Widget>>(&self, child: &P);

    #[cfg(feature = "v3_10")]
    fn pack_start<P: IsA<Widget>>(&self, child: &P);

    #[cfg(feature = "v3_10")]
    fn set_custom_title<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, title_widget: Q);

    #[cfg(feature = "v3_12")]
    fn set_decoration_layout<'a, P: Into<Option<&'a str>>>(&self, layout: P);

    #[cfg(feature = "v3_12")]
    fn set_has_subtitle(&self, setting: bool);

    #[cfg(feature = "v3_10")]
    fn set_show_close_button(&self, setting: bool);

    #[cfg(feature = "v3_10")]
    fn set_subtitle<'a, P: Into<Option<&'a str>>>(&self, subtitle: P);

    #[cfg(feature = "v3_10")]
    fn set_title<'a, P: Into<Option<&'a str>>>(&self, title: P);

    fn get_property_custom_title(&self) -> Option<Widget>;

    fn set_property_custom_title<P: IsA<Widget> + IsA<glib::object::Object>>(&self, custom_title: Option<&P>);

    #[cfg(feature = "v3_12")]
    fn get_property_decoration_layout_set(&self) -> bool;

    #[cfg(feature = "v3_12")]
    fn set_property_decoration_layout_set(&self, decoration_layout_set: bool);

    fn get_property_show_close_button(&self) -> bool;

    fn set_property_show_close_button(&self, show_close_button: bool);

    fn get_property_spacing(&self) -> i32;

    fn set_property_spacing(&self, spacing: i32);

    fn get_property_subtitle(&self) -> Option<String>;

    fn set_property_subtitle(&self, subtitle: Option<&str>);

    fn get_property_title(&self) -> Option<String>;

    fn set_property_title(&self, title: Option<&str>);

    fn get_child_pack_type<T: IsA<Widget>>(&self, item: &T) -> PackType;

    fn set_child_pack_type<T: IsA<Widget>>(&self, item: &T, pack_type: PackType);

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32);
}

impl<O: IsA<HeaderBar> + IsA<Container> + IsA<glib::object::Object>> HeaderBarExt for O {
    #[cfg(feature = "v3_10")]
    fn get_custom_title(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_custom_title(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    fn get_decoration_layout(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_decoration_layout(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    fn get_has_subtitle(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_header_bar_get_has_subtitle(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_show_close_button(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_header_bar_get_show_close_button(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_subtitle(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_subtitle(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_title(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn pack_end<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_header_bar_pack_end(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn pack_start<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_header_bar_pack_start(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_custom_title<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, title_widget: Q) {
        let title_widget = title_widget.into();
        let title_widget = title_widget.to_glib_none();
        unsafe {
            ffi::gtk_header_bar_set_custom_title(self.to_glib_none().0, title_widget.0);
        }
    }

    #[cfg(feature = "v3_12")]
    fn set_decoration_layout<'a, P: Into<Option<&'a str>>>(&self, layout: P) {
        let layout = layout.into();
        let layout = layout.to_glib_none();
        unsafe {
            ffi::gtk_header_bar_set_decoration_layout(self.to_glib_none().0, layout.0);
        }
    }

    #[cfg(feature = "v3_12")]
    fn set_has_subtitle(&self, setting: bool) {
        unsafe {
            ffi::gtk_header_bar_set_has_subtitle(self.to_glib_none().0, setting.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_show_close_button(&self, setting: bool) {
        unsafe {
            ffi::gtk_header_bar_set_show_close_button(self.to_glib_none().0, setting.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_subtitle<'a, P: Into<Option<&'a str>>>(&self, subtitle: P) {
        let subtitle = subtitle.into();
        let subtitle = subtitle.to_glib_none();
        unsafe {
            ffi::gtk_header_bar_set_subtitle(self.to_glib_none().0, subtitle.0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_title<'a, P: Into<Option<&'a str>>>(&self, title: P) {
        let title = title.into();
        let title = title.to_glib_none();
        unsafe {
            ffi::gtk_header_bar_set_title(self.to_glib_none().0, title.0);
        }
    }

    fn get_property_custom_title(&self) -> Option<Widget> {
        let mut value = Value::from(None::<&Widget>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "custom-title".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_custom_title<P: IsA<Widget> + IsA<glib::object::Object>>(&self, custom_title: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "custom-title".to_glib_none().0, Value::from(custom_title).to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    fn get_property_decoration_layout_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "decoration-layout-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[cfg(feature = "v3_12")]
    fn set_property_decoration_layout_set(&self, decoration_layout_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "decoration-layout-set".to_glib_none().0, Value::from(&decoration_layout_set).to_glib_none().0);
        }
    }

    fn get_property_show_close_button(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-close-button".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_close_button(&self, show_close_button: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-close-button".to_glib_none().0, Value::from(&show_close_button).to_glib_none().0);
        }
    }

    fn get_property_spacing(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "spacing".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_spacing(&self, spacing: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "spacing".to_glib_none().0, Value::from(&spacing).to_glib_none().0);
        }
    }

    fn get_property_subtitle(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "subtitle".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_subtitle(&self, subtitle: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "subtitle".to_glib_none().0, Value::from(subtitle).to_glib_none().0);
        }
    }

    fn get_property_title(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "title".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_title(&self, title: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "title".to_glib_none().0, Value::from(title).to_glib_none().0);
        }
    }

    fn get_child_pack_type<T: IsA<Widget>>(&self, item: &T) -> PackType {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "pack-type".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_child_pack_type<T: IsA<Widget>>(&self, item: &T, pack_type: PackType) {
        let pack_type = pack_type.to_glib() as i32;
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "pack-type".to_glib_none().0, Value::from(&pack_type).to_glib_none().0);
        }
    }

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, Value::from(&position).to_glib_none().0);
        }
    }
}
