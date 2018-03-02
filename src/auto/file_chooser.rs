// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Error;
use FileChooserAction;
use FileChooserConfirmation;
use FileFilter;
use Widget;
use ffi;
use gio;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct FileChooser(Object<ffi::GtkFileChooser>);

    match fn {
        get_type => || ffi::gtk_file_chooser_get_type(),
    }
}

pub trait FileChooserExt {
    fn add_filter(&self, filter: &FileFilter);

    fn add_shortcut_folder<P: AsRef<std::path::Path>>(&self, folder: P) -> Result<(), Error>;

    fn add_shortcut_folder_uri(&self, uri: &str) -> Result<(), Error>;

    fn get_action(&self) -> FileChooserAction;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_choice(&self, id: &str) -> Option<String>;

    fn get_create_folders(&self) -> bool;

    fn get_current_folder(&self) -> Option<std::path::PathBuf>;

    fn get_current_folder_file(&self) -> Option<gio::File>;

    fn get_current_folder_uri(&self) -> Option<String>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_current_name(&self) -> Option<String>;

    fn get_do_overwrite_confirmation(&self) -> bool;

    fn get_extra_widget(&self) -> Option<Widget>;

    fn get_file(&self) -> Option<gio::File>;

    fn get_filename(&self) -> Option<std::path::PathBuf>;

    fn get_filenames(&self) -> Vec<std::path::PathBuf>;

    fn get_files(&self) -> Vec<gio::File>;

    fn get_filter(&self) -> Option<FileFilter>;

    fn get_local_only(&self) -> bool;

    fn get_preview_file(&self) -> Option<gio::File>;

    fn get_preview_filename(&self) -> Option<std::path::PathBuf>;

    fn get_preview_uri(&self) -> Option<String>;

    fn get_preview_widget(&self) -> Option<Widget>;

    fn get_preview_widget_active(&self) -> bool;

    fn get_select_multiple(&self) -> bool;

    fn get_show_hidden(&self) -> bool;

    fn get_uri(&self) -> Option<String>;

    fn get_uris(&self) -> Vec<String>;

    fn get_use_preview_label(&self) -> bool;

    fn list_filters(&self) -> Vec<FileFilter>;

    fn list_shortcut_folder_uris(&self) -> Vec<String>;

    fn list_shortcut_folders(&self) -> Vec<std::path::PathBuf>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn remove_choice(&self, id: &str);

    fn remove_filter(&self, filter: &FileFilter);

    fn remove_shortcut_folder<P: AsRef<std::path::Path>>(&self, folder: P) -> Result<(), Error>;

    fn remove_shortcut_folder_uri(&self, uri: &str) -> Result<(), Error>;

    fn select_all(&self);

    fn select_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), Error>;

    fn select_filename<P: AsRef<std::path::Path>>(&self, filename: P) -> bool;

    fn select_uri(&self, uri: &str) -> bool;

    fn set_action(&self, action: FileChooserAction);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_choice(&self, id: &str, option: &str);

    fn set_create_folders(&self, create_folders: bool);

    fn set_current_folder<P: AsRef<std::path::Path>>(&self, filename: P) -> bool;

    fn set_current_folder_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), Error>;

    fn set_current_folder_uri(&self, uri: &str) -> bool;

    fn set_current_name<P: AsRef<std::path::Path>>(&self, name: P);

    fn set_do_overwrite_confirmation(&self, do_overwrite_confirmation: bool);

    fn set_extra_widget<P: IsA<Widget>>(&self, extra_widget: &P);

    fn set_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), Error>;

    fn set_filename<P: AsRef<std::path::Path>>(&self, filename: P) -> bool;

    fn set_filter(&self, filter: &FileFilter);

    fn set_local_only(&self, local_only: bool);

    fn set_preview_widget<P: IsA<Widget>>(&self, preview_widget: &P);

    fn set_preview_widget_active(&self, active: bool);

    fn set_select_multiple(&self, select_multiple: bool);

    fn set_show_hidden(&self, show_hidden: bool);

    fn set_uri(&self, uri: &str) -> bool;

    fn set_use_preview_label(&self, use_label: bool);

    fn unselect_all(&self);

    fn unselect_file<P: IsA<gio::File>>(&self, file: &P);

    fn unselect_filename<P: AsRef<std::path::Path>>(&self, filename: P);

    fn unselect_uri(&self, uri: &str);

    fn connect_confirm_overwrite<F: Fn(&Self) -> FileChooserConfirmation + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_current_folder_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_file_activated<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_update_preview<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_create_folders_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_do_overwrite_confirmation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_extra_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_local_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_preview_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_preview_widget_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_select_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_hidden_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_preview_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileChooser> + IsA<glib::object::Object>> FileChooserExt for O {
    fn add_filter(&self, filter: &FileFilter) {
        unsafe {
            ffi::gtk_file_chooser_add_filter(self.to_glib_none().0, filter.to_glib_full());
        }
    }

    fn add_shortcut_folder<P: AsRef<std::path::Path>>(&self, folder: P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_file_chooser_add_shortcut_folder(self.to_glib_none().0, folder.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn add_shortcut_folder_uri(&self, uri: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_file_chooser_add_shortcut_folder_uri(self.to_glib_none().0, uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_action(&self) -> FileChooserAction {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_action(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_choice(&self, id: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_get_choice(self.to_glib_none().0, id.to_glib_none().0))
        }
    }

    fn get_create_folders(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_create_folders(self.to_glib_none().0))
        }
    }

    fn get_current_folder(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_current_folder(self.to_glib_none().0))
        }
    }

    fn get_current_folder_file(&self) -> Option<gio::File> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_current_folder_file(self.to_glib_none().0))
        }
    }

    fn get_current_folder_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_current_folder_uri(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_current_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_current_name(self.to_glib_none().0))
        }
    }

    fn get_do_overwrite_confirmation(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_do_overwrite_confirmation(self.to_glib_none().0))
        }
    }

    fn get_extra_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_get_extra_widget(self.to_glib_none().0))
        }
    }

    fn get_file(&self) -> Option<gio::File> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_file(self.to_glib_none().0))
        }
    }

    fn get_filename(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_filename(self.to_glib_none().0))
        }
    }

    fn get_filenames(&self) -> Vec<std::path::PathBuf> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_file_chooser_get_filenames(self.to_glib_none().0))
        }
    }

    fn get_files(&self) -> Vec<gio::File> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_file_chooser_get_files(self.to_glib_none().0))
        }
    }

    fn get_filter(&self) -> Option<FileFilter> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_get_filter(self.to_glib_none().0))
        }
    }

    fn get_local_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_local_only(self.to_glib_none().0))
        }
    }

    fn get_preview_file(&self) -> Option<gio::File> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_preview_file(self.to_glib_none().0))
        }
    }

    fn get_preview_filename(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_preview_filename(self.to_glib_none().0))
        }
    }

    fn get_preview_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_preview_uri(self.to_glib_none().0))
        }
    }

    fn get_preview_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_get_preview_widget(self.to_glib_none().0))
        }
    }

    fn get_preview_widget_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_preview_widget_active(self.to_glib_none().0))
        }
    }

    fn get_select_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_select_multiple(self.to_glib_none().0))
        }
    }

    fn get_show_hidden(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_show_hidden(self.to_glib_none().0))
        }
    }

    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_uri(self.to_glib_none().0))
        }
    }

    fn get_uris(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_file_chooser_get_uris(self.to_glib_none().0))
        }
    }

    fn get_use_preview_label(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_use_preview_label(self.to_glib_none().0))
        }
    }

    fn list_filters(&self) -> Vec<FileFilter> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_file_chooser_list_filters(self.to_glib_none().0))
        }
    }

    fn list_shortcut_folder_uris(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_file_chooser_list_shortcut_folder_uris(self.to_glib_none().0))
        }
    }

    fn list_shortcut_folders(&self) -> Vec<std::path::PathBuf> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_file_chooser_list_shortcut_folders(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn remove_choice(&self, id: &str) {
        unsafe {
            ffi::gtk_file_chooser_remove_choice(self.to_glib_none().0, id.to_glib_none().0);
        }
    }

    fn remove_filter(&self, filter: &FileFilter) {
        unsafe {
            ffi::gtk_file_chooser_remove_filter(self.to_glib_none().0, filter.to_glib_none().0);
        }
    }

    fn remove_shortcut_folder<P: AsRef<std::path::Path>>(&self, folder: P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_file_chooser_remove_shortcut_folder(self.to_glib_none().0, folder.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn remove_shortcut_folder_uri(&self, uri: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_file_chooser_remove_shortcut_folder_uri(self.to_glib_none().0, uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn select_all(&self) {
        unsafe {
            ffi::gtk_file_chooser_select_all(self.to_glib_none().0);
        }
    }

    fn select_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_file_chooser_select_file(self.to_glib_none().0, file.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn select_filename<P: AsRef<std::path::Path>>(&self, filename: P) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_select_filename(self.to_glib_none().0, filename.as_ref().to_glib_none().0))
        }
    }

    fn select_uri(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_select_uri(self.to_glib_none().0, uri.to_glib_none().0))
        }
    }

    fn set_action(&self, action: FileChooserAction) {
        unsafe {
            ffi::gtk_file_chooser_set_action(self.to_glib_none().0, action.to_glib());
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_choice(&self, id: &str, option: &str) {
        unsafe {
            ffi::gtk_file_chooser_set_choice(self.to_glib_none().0, id.to_glib_none().0, option.to_glib_none().0);
        }
    }

    fn set_create_folders(&self, create_folders: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_create_folders(self.to_glib_none().0, create_folders.to_glib());
        }
    }

    fn set_current_folder<P: AsRef<std::path::Path>>(&self, filename: P) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_set_current_folder(self.to_glib_none().0, filename.as_ref().to_glib_none().0))
        }
    }

    fn set_current_folder_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_file_chooser_set_current_folder_file(self.to_glib_none().0, file.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_current_folder_uri(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_set_current_folder_uri(self.to_glib_none().0, uri.to_glib_none().0))
        }
    }

    fn set_current_name<P: AsRef<std::path::Path>>(&self, name: P) {
        unsafe {
            ffi::gtk_file_chooser_set_current_name(self.to_glib_none().0, name.as_ref().to_glib_none().0);
        }
    }

    fn set_do_overwrite_confirmation(&self, do_overwrite_confirmation: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_do_overwrite_confirmation(self.to_glib_none().0, do_overwrite_confirmation.to_glib());
        }
    }

    fn set_extra_widget<P: IsA<Widget>>(&self, extra_widget: &P) {
        unsafe {
            ffi::gtk_file_chooser_set_extra_widget(self.to_glib_none().0, extra_widget.to_glib_none().0);
        }
    }

    fn set_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_file_chooser_set_file(self.to_glib_none().0, file.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_filename<P: AsRef<std::path::Path>>(&self, filename: P) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_set_filename(self.to_glib_none().0, filename.as_ref().to_glib_none().0))
        }
    }

    fn set_filter(&self, filter: &FileFilter) {
        unsafe {
            ffi::gtk_file_chooser_set_filter(self.to_glib_none().0, filter.to_glib_none().0);
        }
    }

    fn set_local_only(&self, local_only: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_local_only(self.to_glib_none().0, local_only.to_glib());
        }
    }

    fn set_preview_widget<P: IsA<Widget>>(&self, preview_widget: &P) {
        unsafe {
            ffi::gtk_file_chooser_set_preview_widget(self.to_glib_none().0, preview_widget.to_glib_none().0);
        }
    }

    fn set_preview_widget_active(&self, active: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_preview_widget_active(self.to_glib_none().0, active.to_glib());
        }
    }

    fn set_select_multiple(&self, select_multiple: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_select_multiple(self.to_glib_none().0, select_multiple.to_glib());
        }
    }

    fn set_show_hidden(&self, show_hidden: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_show_hidden(self.to_glib_none().0, show_hidden.to_glib());
        }
    }

    fn set_uri(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_set_uri(self.to_glib_none().0, uri.to_glib_none().0))
        }
    }

    fn set_use_preview_label(&self, use_label: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_use_preview_label(self.to_glib_none().0, use_label.to_glib());
        }
    }

    fn unselect_all(&self) {
        unsafe {
            ffi::gtk_file_chooser_unselect_all(self.to_glib_none().0);
        }
    }

    fn unselect_file<P: IsA<gio::File>>(&self, file: &P) {
        unsafe {
            ffi::gtk_file_chooser_unselect_file(self.to_glib_none().0, file.to_glib_none().0);
        }
    }

    fn unselect_filename<P: AsRef<std::path::Path>>(&self, filename: P) {
        unsafe {
            ffi::gtk_file_chooser_unselect_filename(self.to_glib_none().0, filename.as_ref().to_glib_none().0);
        }
    }

    fn unselect_uri(&self, uri: &str) {
        unsafe {
            ffi::gtk_file_chooser_unselect_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }

    fn connect_confirm_overwrite<F: Fn(&Self) -> FileChooserConfirmation + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> FileChooserConfirmation + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "confirm-overwrite",
                transmute(confirm_overwrite_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_current_folder_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "current-folder-changed",
                transmute(current_folder_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_file_activated<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "file-activated",
                transmute(file_activated_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "selection-changed",
                transmute(selection_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_update_preview<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "update-preview",
                transmute(update_preview_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::action",
                transmute(notify_action_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_create_folders_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::create-folders",
                transmute(notify_create_folders_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_do_overwrite_confirmation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::do-overwrite-confirmation",
                transmute(notify_do_overwrite_confirmation_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_extra_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::extra-widget",
                transmute(notify_extra_widget_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::filter",
                transmute(notify_filter_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_local_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::local-only",
                transmute(notify_local_only_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_preview_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::preview-widget",
                transmute(notify_preview_widget_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_preview_widget_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::preview-widget-active",
                transmute(notify_preview_widget_active_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_select_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::select-multiple",
                transmute(notify_select_multiple_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_hidden_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-hidden",
                transmute(notify_show_hidden_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_preview_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-preview-label",
                transmute(notify_use_preview_label_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn confirm_overwrite_trampoline<P>(this: *mut ffi::GtkFileChooser, f: glib_ffi::gpointer) -> ffi::GtkFileChooserConfirmation
where P: IsA<FileChooser> {
    callback_guard!();
    let f: &&(Fn(&P) -> FileChooserConfirmation + 'static) = transmute(f);
    f(&FileChooser::from_glib_borrow(this).downcast_unchecked()).to_glib()
}

unsafe extern "C" fn current_folder_changed_trampoline<P>(this: *mut ffi::GtkFileChooser, f: glib_ffi::gpointer)
where P: IsA<FileChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn file_activated_trampoline<P>(this: *mut ffi::GtkFileChooser, f: glib_ffi::gpointer)
where P: IsA<FileChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn selection_changed_trampoline<P>(this: *mut ffi::GtkFileChooser, f: glib_ffi::gpointer)
where P: IsA<FileChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn update_preview_trampoline<P>(this: *mut ffi::GtkFileChooser, f: glib_ffi::gpointer)
where P: IsA<FileChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_action_trampoline<P>(this: *mut ffi::GtkFileChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_create_folders_trampoline<P>(this: *mut ffi::GtkFileChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_do_overwrite_confirmation_trampoline<P>(this: *mut ffi::GtkFileChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_extra_widget_trampoline<P>(this: *mut ffi::GtkFileChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_filter_trampoline<P>(this: *mut ffi::GtkFileChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_local_only_trampoline<P>(this: *mut ffi::GtkFileChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_preview_widget_trampoline<P>(this: *mut ffi::GtkFileChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_preview_widget_active_trampoline<P>(this: *mut ffi::GtkFileChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_select_multiple_trampoline<P>(this: *mut ffi::GtkFileChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_hidden_trampoline<P>(this: *mut ffi::GtkFileChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_preview_label_trampoline<P>(this: *mut ffi::GtkFileChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooser::from_glib_borrow(this).downcast_unchecked())
}
