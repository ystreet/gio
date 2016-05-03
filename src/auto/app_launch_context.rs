// This file was generated by gir (e6cb5d0) from gir-files (11e0e6d)
// DO NOT EDIT

#[cfg(feature = "v2_36")]
use AppInfo;
use ffi;
#[cfg(feature = "v2_36")]
use glib;
#[cfg(feature = "v2_36")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v2_36")]
use glib_ffi;
#[cfg(feature = "v2_36")]
use libc;
#[cfg(feature = "v2_36")]
use std::boxed::Box as Box_;
#[cfg(feature = "v2_36")]
use std::mem::transmute;

glib_wrapper! {
    pub struct AppLaunchContext(Object<ffi::GAppLaunchContext>);

    match fn {
        get_type => || ffi::g_app_launch_context_get_type(),
    }
}

impl AppLaunchContext {
    pub fn new() -> AppLaunchContext {
        unsafe {
            from_glib_full(ffi::g_app_launch_context_new())
        }
    }

    //pub fn get_display<T: IsA<AppInfo>>(&self, info: &T, files: /*Ignored*/&[File]) -> Option<String> {
    //    unsafe { TODO: call ffi::g_app_launch_context_get_display() }
    //}

    pub fn get_environment(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_app_launch_context_get_environment(self.to_glib_none().0))
        }
    }

    //pub fn get_startup_notify_id<T: IsA<AppInfo>>(&self, info: &T, files: /*Ignored*/&[File]) -> Option<String> {
    //    unsafe { TODO: call ffi::g_app_launch_context_get_startup_notify_id() }
    //}

    pub fn launch_failed(&self, startup_notify_id: &str) {
        unsafe {
            ffi::g_app_launch_context_launch_failed(self.to_glib_none().0, startup_notify_id.to_glib_none().0);
        }
    }

    pub fn setenv(&self, variable: &str, value: &str) {
        unsafe {
            ffi::g_app_launch_context_setenv(self.to_glib_none().0, variable.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn unsetenv(&self, variable: &str) {
        unsafe {
            ffi::g_app_launch_context_unsetenv(self.to_glib_none().0, variable.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_36")]
    pub fn connect_launch_failed<F: Fn(&AppLaunchContext, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&AppLaunchContext, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "launch-failed",
                transmute(launch_failed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v2_36")]
    pub fn connect_launched<F: Fn(&AppLaunchContext, &AppInfo, &glib::Variant) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&AppLaunchContext, &AppInfo, &glib::Variant) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "launched",
                transmute(launched_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v2_36")]
unsafe extern "C" fn launch_failed_trampoline(this: *mut ffi::GAppLaunchContext, startup_notify_id: *mut libc::c_char, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&AppLaunchContext, &str) + 'static> = transmute(f);
    f(&from_glib_none(this), &String::from_glib_none(startup_notify_id))
}

#[cfg(feature = "v2_36")]
unsafe extern "C" fn launched_trampoline(this: *mut ffi::GAppLaunchContext, info: *mut ffi::GAppInfo, platform_data: *mut glib_ffi::GVariant, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&AppLaunchContext, &AppInfo, &glib::Variant) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(info), &from_glib_none(platform_data))
}
