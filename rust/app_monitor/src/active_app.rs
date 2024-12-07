use core_graphics::display::*;
use core_foundation::string::*;
use core_foundation::base::*;
use core_foundation::number::*;
use std::ffi::c_void;
use std::ptr;

pub fn get_active_window_app() -> String {
    let options: CGWindowListOption = kCGWindowListOptionOnScreenOnly | kCGWindowListExcludeDesktopElements;
    let window_list_info = unsafe { CGWindowListCopyWindowInfo(options, kCGNullWindowID) };
    if window_list_info.is_null() {
        return "Unknown".to_string();
    }

    let count = unsafe { CFArrayGetCount(window_list_info) };
    for i in 0..count {
        let dic_ref = unsafe {
            CFArrayGetValueAtIndex(window_list_info, i as isize) as CFDictionaryRef
        };
        if dic_ref.is_null() {
            continue;
        }

        // kCGWindowLayer を取得
        let layer_key = CFString::new("kCGWindowLayer");
        let mut layer_value: *const c_void = ptr::null();
        let has_layer = unsafe {
            CFDictionaryGetValueIfPresent(dic_ref, layer_key.as_concrete_TypeRef() as *const c_void, &mut layer_value)
        } != 0;

        let mut layer_is_zero = false;
        if has_layer && !layer_value.is_null() {
            let layer_cfnum = layer_value as CFNumberRef;
            let mut layer: i32 = 0;
            let got_layer = unsafe {
                CFNumberGetValue(layer_cfnum, kCFNumberIntType, &mut layer as *mut i32 as *mut c_void)
            };
            if got_layer && layer == 0 {
                layer_is_zero = true;
            }
        }

        // layer が 0 の場合、ウィンドウ所有アプリケーション名を取得
        if layer_is_zero {
            let owner_key = CFString::new("kCGWindowOwnerName");
            let mut owner_value: *const c_void = ptr::null();
            let has_owner = unsafe {
                CFDictionaryGetValueIfPresent(dic_ref, owner_key.as_concrete_TypeRef() as *const c_void, &mut owner_value)
            } != 0;

            if has_owner && !owner_value.is_null() {
                let cf_ref = owner_value as CFStringRef;
                let cf_str: CFString = unsafe { CFString::wrap_under_get_rule(cf_ref) };
                let app_name = cf_str.to_string();

                unsafe { CFRelease(window_list_info as CFTypeRef) };
                return app_name;
            } else {
                unsafe { CFRelease(window_list_info as CFTypeRef) };
                return "Unknown".to_string();
            }
        }
    }

    unsafe { CFRelease(window_list_info as CFTypeRef) };
    "Unknown".to_string()
}
