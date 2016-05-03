// This file was generated by gir (e6cb5d0) from gir-files (11e0e6d)
// DO NOT EDIT

use MenuItem;
use MenuModel;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Menu(Object<ffi::GMenu>): MenuModel;

    match fn {
        get_type => || ffi::g_menu_get_type(),
    }
}

impl Menu {
    pub fn new() -> Menu {
        unsafe {
            from_glib_full(ffi::g_menu_new())
        }
    }

    pub fn append(&self, label: Option<&str>, detailed_action: Option<&str>) {
        unsafe {
            ffi::g_menu_append(self.to_glib_none().0, label.to_glib_none().0, detailed_action.to_glib_none().0);
        }
    }

    pub fn append_item(&self, item: &MenuItem) {
        unsafe {
            ffi::g_menu_append_item(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    pub fn append_section<T: IsA<MenuModel>>(&self, label: Option<&str>, section: &T) {
        unsafe {
            ffi::g_menu_append_section(self.to_glib_none().0, label.to_glib_none().0, section.to_glib_none().0);
        }
    }

    pub fn append_submenu<T: IsA<MenuModel>>(&self, label: Option<&str>, submenu: &T) {
        unsafe {
            ffi::g_menu_append_submenu(self.to_glib_none().0, label.to_glib_none().0, submenu.to_glib_none().0);
        }
    }

    pub fn freeze(&self) {
        unsafe {
            ffi::g_menu_freeze(self.to_glib_none().0);
        }
    }

    pub fn insert(&self, position: i32, label: Option<&str>, detailed_action: Option<&str>) {
        unsafe {
            ffi::g_menu_insert(self.to_glib_none().0, position, label.to_glib_none().0, detailed_action.to_glib_none().0);
        }
    }

    pub fn insert_item(&self, position: i32, item: &MenuItem) {
        unsafe {
            ffi::g_menu_insert_item(self.to_glib_none().0, position, item.to_glib_none().0);
        }
    }

    pub fn insert_section<T: IsA<MenuModel>>(&self, position: i32, label: Option<&str>, section: &T) {
        unsafe {
            ffi::g_menu_insert_section(self.to_glib_none().0, position, label.to_glib_none().0, section.to_glib_none().0);
        }
    }

    pub fn insert_submenu<T: IsA<MenuModel>>(&self, position: i32, label: Option<&str>, submenu: &T) {
        unsafe {
            ffi::g_menu_insert_submenu(self.to_glib_none().0, position, label.to_glib_none().0, submenu.to_glib_none().0);
        }
    }

    pub fn prepend(&self, label: Option<&str>, detailed_action: Option<&str>) {
        unsafe {
            ffi::g_menu_prepend(self.to_glib_none().0, label.to_glib_none().0, detailed_action.to_glib_none().0);
        }
    }

    pub fn prepend_item(&self, item: &MenuItem) {
        unsafe {
            ffi::g_menu_prepend_item(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    pub fn prepend_section<T: IsA<MenuModel>>(&self, label: Option<&str>, section: &T) {
        unsafe {
            ffi::g_menu_prepend_section(self.to_glib_none().0, label.to_glib_none().0, section.to_glib_none().0);
        }
    }

    pub fn prepend_submenu<T: IsA<MenuModel>>(&self, label: Option<&str>, submenu: &T) {
        unsafe {
            ffi::g_menu_prepend_submenu(self.to_glib_none().0, label.to_glib_none().0, submenu.to_glib_none().0);
        }
    }

    pub fn remove(&self, position: i32) {
        unsafe {
            ffi::g_menu_remove(self.to_glib_none().0, position);
        }
    }

    #[cfg(feature = "v2_38")]
    pub fn remove_all(&self) {
        unsafe {
            ffi::g_menu_remove_all(self.to_glib_none().0);
        }
    }
}
