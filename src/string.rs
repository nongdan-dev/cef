use crate::ffi::*;
use widestring::U16CString;

pub(crate) trait ToCefString {
    fn to_cef_string(self) -> cef_string_t;
}
unsafe extern "C" fn free_cef_string(ptr: *mut u16) {
    if ptr.is_null() {
        return;
    }
    drop(U16CString::from_raw(ptr));
}
impl ToCefString for U16CString {
    fn to_cef_string(self) -> cef_string_t {
        cef_string_t {
            length: self.len(),
            str_: self.into_raw(),
            dtor: Some(free_cef_string),
        }
    }
}
impl ToCefString for String {
    #[inline(always)]
    fn to_cef_string(self) -> cef_string_t {
        U16CString::from_str(self).unwrap().to_cef_string()
    }
}

pub(crate) unsafe fn cef_to_string(ptr: *const cef_string_t) -> String {
    if ptr.is_null() {
        return "".to_string();
    }
    U16CString::from_ptr((*ptr).str_, (*ptr).length)
        .unwrap()
        .to_string()
        .unwrap()
}
