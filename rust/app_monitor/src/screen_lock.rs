use core_foundation::base::{TCFType, CFType};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::string::CFString;
use core_foundation::boolean::CFBoolean;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGSessionCopyCurrentDictionary() -> CFDictionaryRef;
}

/// 画面ロック状態をチェックする関数
pub fn is_screen_locked() -> Option<bool> {
    unsafe {
        let dict_ref = CGSessionCopyCurrentDictionary();
        if dict_ref.is_null() {
            return None;
        }

        // 辞書のキー: CFString, 値: CFType として扱う
        let cf_dict: CFDictionary<CFString, CFType> = CFDictionary::wrap_under_create_rule(dict_ref);
        let key = CFString::new("CGSSessionScreenIsLocked");

        if let Some(value) = cf_dict.find(&key) {
            // value は &CFType
            // CFTypeRefを取得してCFBooleanにキャスト
            let cf_bool = CFBoolean::wrap_under_get_rule(value.as_CFTypeRef() as core_foundation::boolean::CFBooleanRef);
            return Some(cf_bool == CFBoolean::true_value());
        }

        None
    }
}


