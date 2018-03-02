// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use FileType;
use Icon;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FileInfo(Object<ffi::GFileInfo, ffi::GFileInfoClass>);

    match fn {
        get_type => || ffi::g_file_info_get_type(),
    }
}

impl FileInfo {
    pub fn new() -> FileInfo {
        unsafe {
            from_glib_full(ffi::g_file_info_new())
        }
    }
}

impl Default for FileInfo {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FileInfoExt {
    fn clear_status(&self);

    fn copy_into(&self, dest_info: &FileInfo);

    fn dup(&self) -> Option<FileInfo>;

    fn get_attribute_as_string(&self, attribute: &str) -> Option<String>;

    fn get_attribute_boolean(&self, attribute: &str) -> bool;

    fn get_attribute_byte_string(&self, attribute: &str) -> Option<String>;

    //fn get_attribute_data(&self, attribute: &str, value_pp: /*Unimplemented*/&mut Fundamental: Pointer) -> Option<(/*Ignored*/FileAttributeType, /*Ignored*/FileAttributeStatus)>;

    fn get_attribute_int32(&self, attribute: &str) -> i32;

    fn get_attribute_int64(&self, attribute: &str) -> i64;

    fn get_attribute_object(&self, attribute: &str) -> Option<glib::Object>;

    //fn get_attribute_status(&self, attribute: &str) -> /*Ignored*/FileAttributeStatus;

    fn get_attribute_string(&self, attribute: &str) -> Option<String>;

    fn get_attribute_stringv(&self, attribute: &str) -> Vec<String>;

    //fn get_attribute_type(&self, attribute: &str) -> /*Ignored*/FileAttributeType;

    fn get_attribute_uint32(&self, attribute: &str) -> u32;

    fn get_attribute_uint64(&self, attribute: &str) -> u64;

    fn get_content_type(&self) -> Option<String>;

    //#[cfg(any(feature = "v2_36", feature = "dox"))]
    //fn get_deletion_date(&self) -> /*Ignored*/Option<glib::DateTime>;

    fn get_display_name(&self) -> Option<String>;

    fn get_edit_name(&self) -> Option<String>;

    fn get_etag(&self) -> Option<String>;

    fn get_file_type(&self) -> FileType;

    fn get_icon(&self) -> Option<Icon>;

    fn get_is_backup(&self) -> bool;

    fn get_is_hidden(&self) -> bool;

    fn get_is_symlink(&self) -> bool;

    //fn get_modification_time(&self, result: /*Ignored*/glib::TimeVal);

    fn get_name(&self) -> Option<std::path::PathBuf>;

    fn get_size(&self) -> i64;

    fn get_sort_order(&self) -> i32;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_symbolic_icon(&self) -> Option<Icon>;

    fn get_symlink_target(&self) -> Option<String>;

    fn has_attribute(&self, attribute: &str) -> bool;

    fn has_namespace(&self, name_space: &str) -> bool;

    fn list_attributes<'a, P: Into<Option<&'a str>>>(&self, name_space: P) -> Vec<String>;

    fn remove_attribute(&self, attribute: &str);

    //fn set_attribute(&self, attribute: &str, type_: /*Ignored*/FileAttributeType, value_p: /*Unimplemented*/Fundamental: Pointer);

    fn set_attribute_boolean(&self, attribute: &str, attr_value: bool);

    fn set_attribute_byte_string(&self, attribute: &str, attr_value: &str);

    fn set_attribute_int32(&self, attribute: &str, attr_value: i32);

    fn set_attribute_int64(&self, attribute: &str, attr_value: i64);

    //fn set_attribute_mask(&self, mask: /*Ignored*/&FileAttributeMatcher);

    fn set_attribute_object<P: IsA<glib::Object>>(&self, attribute: &str, attr_value: &P);

    //fn set_attribute_status(&self, attribute: &str, status: /*Ignored*/FileAttributeStatus) -> bool;

    fn set_attribute_string(&self, attribute: &str, attr_value: &str);

    fn set_attribute_stringv(&self, attribute: &str, attr_value: &[&str]);

    fn set_attribute_uint32(&self, attribute: &str, attr_value: u32);

    fn set_attribute_uint64(&self, attribute: &str, attr_value: u64);

    fn set_content_type(&self, content_type: &str);

    fn set_display_name(&self, display_name: &str);

    fn set_edit_name(&self, edit_name: &str);

    fn set_file_type(&self, type_: FileType);

    fn set_icon<P: IsA<Icon>>(&self, icon: &P);

    fn set_is_hidden(&self, is_hidden: bool);

    fn set_is_symlink(&self, is_symlink: bool);

    //fn set_modification_time(&self, mtime: /*Ignored*/&mut glib::TimeVal);

    fn set_name<P: AsRef<std::path::Path>>(&self, name: P);

    fn set_size(&self, size: i64);

    fn set_sort_order(&self, sort_order: i32);

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn set_symbolic_icon<P: IsA<Icon>>(&self, icon: &P);

    fn set_symlink_target(&self, symlink_target: &str);

    fn unset_attribute_mask(&self);
}

impl<O: IsA<FileInfo>> FileInfoExt for O {
    fn clear_status(&self) {
        unsafe {
            ffi::g_file_info_clear_status(self.to_glib_none().0);
        }
    }

    fn copy_into(&self, dest_info: &FileInfo) {
        unsafe {
            ffi::g_file_info_copy_into(self.to_glib_none().0, dest_info.to_glib_none().0);
        }
    }

    fn dup(&self) -> Option<FileInfo> {
        unsafe {
            from_glib_full(ffi::g_file_info_dup(self.to_glib_none().0))
        }
    }

    fn get_attribute_as_string(&self, attribute: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_file_info_get_attribute_as_string(self.to_glib_none().0, attribute.to_glib_none().0))
        }
    }

    fn get_attribute_boolean(&self, attribute: &str) -> bool {
        unsafe {
            from_glib(ffi::g_file_info_get_attribute_boolean(self.to_glib_none().0, attribute.to_glib_none().0))
        }
    }

    fn get_attribute_byte_string(&self, attribute: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_file_info_get_attribute_byte_string(self.to_glib_none().0, attribute.to_glib_none().0))
        }
    }

    //fn get_attribute_data(&self, attribute: &str, value_pp: /*Unimplemented*/&mut Fundamental: Pointer) -> Option<(/*Ignored*/FileAttributeType, /*Ignored*/FileAttributeStatus)> {
    //    unsafe { TODO: call ffi::g_file_info_get_attribute_data() }
    //}

    fn get_attribute_int32(&self, attribute: &str) -> i32 {
        unsafe {
            ffi::g_file_info_get_attribute_int32(self.to_glib_none().0, attribute.to_glib_none().0)
        }
    }

    fn get_attribute_int64(&self, attribute: &str) -> i64 {
        unsafe {
            ffi::g_file_info_get_attribute_int64(self.to_glib_none().0, attribute.to_glib_none().0)
        }
    }

    fn get_attribute_object(&self, attribute: &str) -> Option<glib::Object> {
        unsafe {
            from_glib_none(ffi::g_file_info_get_attribute_object(self.to_glib_none().0, attribute.to_glib_none().0))
        }
    }

    //fn get_attribute_status(&self, attribute: &str) -> /*Ignored*/FileAttributeStatus {
    //    unsafe { TODO: call ffi::g_file_info_get_attribute_status() }
    //}

    fn get_attribute_string(&self, attribute: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_file_info_get_attribute_string(self.to_glib_none().0, attribute.to_glib_none().0))
        }
    }

    fn get_attribute_stringv(&self, attribute: &str) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_file_info_get_attribute_stringv(self.to_glib_none().0, attribute.to_glib_none().0))
        }
    }

    //fn get_attribute_type(&self, attribute: &str) -> /*Ignored*/FileAttributeType {
    //    unsafe { TODO: call ffi::g_file_info_get_attribute_type() }
    //}

    fn get_attribute_uint32(&self, attribute: &str) -> u32 {
        unsafe {
            ffi::g_file_info_get_attribute_uint32(self.to_glib_none().0, attribute.to_glib_none().0)
        }
    }

    fn get_attribute_uint64(&self, attribute: &str) -> u64 {
        unsafe {
            ffi::g_file_info_get_attribute_uint64(self.to_glib_none().0, attribute.to_glib_none().0)
        }
    }

    fn get_content_type(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_file_info_get_content_type(self.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v2_36", feature = "dox"))]
    //fn get_deletion_date(&self) -> /*Ignored*/Option<glib::DateTime> {
    //    unsafe { TODO: call ffi::g_file_info_get_deletion_date() }
    //}

    fn get_display_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_file_info_get_display_name(self.to_glib_none().0))
        }
    }

    fn get_edit_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_file_info_get_edit_name(self.to_glib_none().0))
        }
    }

    fn get_etag(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_file_info_get_etag(self.to_glib_none().0))
        }
    }

    fn get_file_type(&self) -> FileType {
        unsafe {
            from_glib(ffi::g_file_info_get_file_type(self.to_glib_none().0))
        }
    }

    fn get_icon(&self) -> Option<Icon> {
        unsafe {
            from_glib_none(ffi::g_file_info_get_icon(self.to_glib_none().0))
        }
    }

    fn get_is_backup(&self) -> bool {
        unsafe {
            from_glib(ffi::g_file_info_get_is_backup(self.to_glib_none().0))
        }
    }

    fn get_is_hidden(&self) -> bool {
        unsafe {
            from_glib(ffi::g_file_info_get_is_hidden(self.to_glib_none().0))
        }
    }

    fn get_is_symlink(&self) -> bool {
        unsafe {
            from_glib(ffi::g_file_info_get_is_symlink(self.to_glib_none().0))
        }
    }

    //fn get_modification_time(&self, result: /*Ignored*/glib::TimeVal) {
    //    unsafe { TODO: call ffi::g_file_info_get_modification_time() }
    //}

    fn get_name(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::g_file_info_get_name(self.to_glib_none().0))
        }
    }

    fn get_size(&self) -> i64 {
        unsafe {
            ffi::g_file_info_get_size(self.to_glib_none().0)
        }
    }

    fn get_sort_order(&self) -> i32 {
        unsafe {
            ffi::g_file_info_get_sort_order(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_symbolic_icon(&self) -> Option<Icon> {
        unsafe {
            from_glib_none(ffi::g_file_info_get_symbolic_icon(self.to_glib_none().0))
        }
    }

    fn get_symlink_target(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_file_info_get_symlink_target(self.to_glib_none().0))
        }
    }

    fn has_attribute(&self, attribute: &str) -> bool {
        unsafe {
            from_glib(ffi::g_file_info_has_attribute(self.to_glib_none().0, attribute.to_glib_none().0))
        }
    }

    fn has_namespace(&self, name_space: &str) -> bool {
        unsafe {
            from_glib(ffi::g_file_info_has_namespace(self.to_glib_none().0, name_space.to_glib_none().0))
        }
    }

    fn list_attributes<'a, P: Into<Option<&'a str>>>(&self, name_space: P) -> Vec<String> {
        let name_space = name_space.into();
        let name_space = name_space.to_glib_none();
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_file_info_list_attributes(self.to_glib_none().0, name_space.0))
        }
    }

    fn remove_attribute(&self, attribute: &str) {
        unsafe {
            ffi::g_file_info_remove_attribute(self.to_glib_none().0, attribute.to_glib_none().0);
        }
    }

    //fn set_attribute(&self, attribute: &str, type_: /*Ignored*/FileAttributeType, value_p: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::g_file_info_set_attribute() }
    //}

    fn set_attribute_boolean(&self, attribute: &str, attr_value: bool) {
        unsafe {
            ffi::g_file_info_set_attribute_boolean(self.to_glib_none().0, attribute.to_glib_none().0, attr_value.to_glib());
        }
    }

    fn set_attribute_byte_string(&self, attribute: &str, attr_value: &str) {
        unsafe {
            ffi::g_file_info_set_attribute_byte_string(self.to_glib_none().0, attribute.to_glib_none().0, attr_value.to_glib_none().0);
        }
    }

    fn set_attribute_int32(&self, attribute: &str, attr_value: i32) {
        unsafe {
            ffi::g_file_info_set_attribute_int32(self.to_glib_none().0, attribute.to_glib_none().0, attr_value);
        }
    }

    fn set_attribute_int64(&self, attribute: &str, attr_value: i64) {
        unsafe {
            ffi::g_file_info_set_attribute_int64(self.to_glib_none().0, attribute.to_glib_none().0, attr_value);
        }
    }

    //fn set_attribute_mask(&self, mask: /*Ignored*/&FileAttributeMatcher) {
    //    unsafe { TODO: call ffi::g_file_info_set_attribute_mask() }
    //}

    fn set_attribute_object<P: IsA<glib::Object>>(&self, attribute: &str, attr_value: &P) {
        unsafe {
            ffi::g_file_info_set_attribute_object(self.to_glib_none().0, attribute.to_glib_none().0, attr_value.to_glib_none().0);
        }
    }

    //fn set_attribute_status(&self, attribute: &str, status: /*Ignored*/FileAttributeStatus) -> bool {
    //    unsafe { TODO: call ffi::g_file_info_set_attribute_status() }
    //}

    fn set_attribute_string(&self, attribute: &str, attr_value: &str) {
        unsafe {
            ffi::g_file_info_set_attribute_string(self.to_glib_none().0, attribute.to_glib_none().0, attr_value.to_glib_none().0);
        }
    }

    fn set_attribute_stringv(&self, attribute: &str, attr_value: &[&str]) {
        unsafe {
            ffi::g_file_info_set_attribute_stringv(self.to_glib_none().0, attribute.to_glib_none().0, attr_value.to_glib_none().0);
        }
    }

    fn set_attribute_uint32(&self, attribute: &str, attr_value: u32) {
        unsafe {
            ffi::g_file_info_set_attribute_uint32(self.to_glib_none().0, attribute.to_glib_none().0, attr_value);
        }
    }

    fn set_attribute_uint64(&self, attribute: &str, attr_value: u64) {
        unsafe {
            ffi::g_file_info_set_attribute_uint64(self.to_glib_none().0, attribute.to_glib_none().0, attr_value);
        }
    }

    fn set_content_type(&self, content_type: &str) {
        unsafe {
            ffi::g_file_info_set_content_type(self.to_glib_none().0, content_type.to_glib_none().0);
        }
    }

    fn set_display_name(&self, display_name: &str) {
        unsafe {
            ffi::g_file_info_set_display_name(self.to_glib_none().0, display_name.to_glib_none().0);
        }
    }

    fn set_edit_name(&self, edit_name: &str) {
        unsafe {
            ffi::g_file_info_set_edit_name(self.to_glib_none().0, edit_name.to_glib_none().0);
        }
    }

    fn set_file_type(&self, type_: FileType) {
        unsafe {
            ffi::g_file_info_set_file_type(self.to_glib_none().0, type_.to_glib());
        }
    }

    fn set_icon<P: IsA<Icon>>(&self, icon: &P) {
        unsafe {
            ffi::g_file_info_set_icon(self.to_glib_none().0, icon.to_glib_none().0);
        }
    }

    fn set_is_hidden(&self, is_hidden: bool) {
        unsafe {
            ffi::g_file_info_set_is_hidden(self.to_glib_none().0, is_hidden.to_glib());
        }
    }

    fn set_is_symlink(&self, is_symlink: bool) {
        unsafe {
            ffi::g_file_info_set_is_symlink(self.to_glib_none().0, is_symlink.to_glib());
        }
    }

    //fn set_modification_time(&self, mtime: /*Ignored*/&mut glib::TimeVal) {
    //    unsafe { TODO: call ffi::g_file_info_set_modification_time() }
    //}

    fn set_name<P: AsRef<std::path::Path>>(&self, name: P) {
        unsafe {
            ffi::g_file_info_set_name(self.to_glib_none().0, name.as_ref().to_glib_none().0);
        }
    }

    fn set_size(&self, size: i64) {
        unsafe {
            ffi::g_file_info_set_size(self.to_glib_none().0, size);
        }
    }

    fn set_sort_order(&self, sort_order: i32) {
        unsafe {
            ffi::g_file_info_set_sort_order(self.to_glib_none().0, sort_order);
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn set_symbolic_icon<P: IsA<Icon>>(&self, icon: &P) {
        unsafe {
            ffi::g_file_info_set_symbolic_icon(self.to_glib_none().0, icon.to_glib_none().0);
        }
    }

    fn set_symlink_target(&self, symlink_target: &str) {
        unsafe {
            ffi::g_file_info_set_symlink_target(self.to_glib_none().0, symlink_target.to_glib_none().0);
        }
    }

    fn unset_attribute_mask(&self) {
        unsafe {
            ffi::g_file_info_unset_attribute_mask(self.to_glib_none().0);
        }
    }
}
