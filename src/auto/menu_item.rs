// This file was generated by gir (e6cb5d0) from gir-files (11e0e6d)
// DO NOT EDIT

use MenuModel;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct MenuItem(Object<ffi::GMenuItem>);

    match fn {
        get_type => || ffi::g_menu_item_get_type(),
    }
}

impl MenuItem {
    pub fn new(label: Option<&str>, detailed_action: Option<&str>) -> MenuItem {
        unsafe {
            from_glib_full(ffi::g_menu_item_new(label.to_glib_none().0, detailed_action.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_34")]
    pub fn new_from_model<T: IsA<MenuModel>>(model: &T, item_index: i32) -> MenuItem {
        unsafe {
            from_glib_full(ffi::g_menu_item_new_from_model(model.to_glib_none().0, item_index))
        }
    }

    pub fn new_section<T: IsA<MenuModel>>(label: Option<&str>, section: &T) -> MenuItem {
        unsafe {
            from_glib_full(ffi::g_menu_item_new_section(label.to_glib_none().0, section.to_glib_none().0))
        }
    }

    pub fn new_submenu<T: IsA<MenuModel>>(label: Option<&str>, submenu: &T) -> MenuItem {
        unsafe {
            from_glib_full(ffi::g_menu_item_new_submenu(label.to_glib_none().0, submenu.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v2_34")]
    //pub fn get_attribute(&self, attribute: &str, format_string: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call ffi::g_menu_item_get_attribute() }
    //}

    #[cfg(feature = "v2_34")]
    pub fn get_attribute_value(&self, attribute: &str, expected_type: Option<&glib::VariantTy>) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_menu_item_get_attribute_value(self.to_glib_none().0, attribute.to_glib_none().0, expected_type.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_34")]
    pub fn get_link(&self, link: &str) -> Option<MenuModel> {
        unsafe {
            from_glib_full(ffi::g_menu_item_get_link(self.to_glib_none().0, link.to_glib_none().0))
        }
    }

    //pub fn set_action_and_target(&self, action: Option<&str>, format_string: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::g_menu_item_set_action_and_target() }
    //}

    pub fn set_action_and_target_value(&self, action: Option<&str>, target_value: Option<&glib::Variant>) {
        unsafe {
            ffi::g_menu_item_set_action_and_target_value(self.to_glib_none().0, action.to_glib_none().0, target_value.to_glib_none().0);
        }
    }

    //pub fn set_attribute(&self, attribute: &str, format_string: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::g_menu_item_set_attribute() }
    //}

    pub fn set_attribute_value(&self, attribute: &str, value: Option<&glib::Variant>) {
        unsafe {
            ffi::g_menu_item_set_attribute_value(self.to_glib_none().0, attribute.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_detailed_action(&self, detailed_action: &str) {
        unsafe {
            ffi::g_menu_item_set_detailed_action(self.to_glib_none().0, detailed_action.to_glib_none().0);
        }
    }

    //#[cfg(feature = "v2_38")]
    //pub fn set_icon<T: IsA</*Ignored*/Icon>>(&self, icon: &T) {
    //    unsafe { TODO: call ffi::g_menu_item_set_icon() }
    //}

    pub fn set_label(&self, label: Option<&str>) {
        unsafe {
            ffi::g_menu_item_set_label(self.to_glib_none().0, label.to_glib_none().0);
        }
    }

    pub fn set_link<T: IsA<MenuModel>>(&self, link: &str, model: Option<&T>) {
        unsafe {
            ffi::g_menu_item_set_link(self.to_glib_none().0, link.to_glib_none().0, model.to_glib_none().0);
        }
    }

    pub fn set_section<T: IsA<MenuModel>>(&self, section: Option<&T>) {
        unsafe {
            ffi::g_menu_item_set_section(self.to_glib_none().0, section.to_glib_none().0);
        }
    }

    pub fn set_submenu<T: IsA<MenuModel>>(&self, submenu: Option<&T>) {
        unsafe {
            ffi::g_menu_item_set_submenu(self.to_glib_none().0, submenu.to_glib_none().0);
        }
    }
}
