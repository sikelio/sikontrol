use std::slice::from_raw_parts;
use windows::core::{
    Result as WindowsResult, GUID, PCWSTR, PWSTR
};

pub struct WindowsUtils {}

impl WindowsUtils {
    pub fn pwstr_to_string(pwstr: PWSTR) -> WindowsResult<String> {
        unsafe {
            let len: usize = (0..).take_while(|&i| *pwstr.0.add(i) != 0).count();
            let slice: &[u16] = from_raw_parts(pwstr.0, len);

            Ok(String::from_utf16_lossy(slice))
        }
    }

    pub fn pcwstr_to_string(pcwstr: &PCWSTR) -> WindowsResult<String> {
        unsafe {
            let len: usize = (0..).take_while(|&i| *pcwstr.0.offset(i) != 0).count();
            let slice: &[u16] = from_raw_parts(pcwstr.0, len);

            Ok(String::from_utf16_lossy(slice))
        }
    }

    pub fn guid_ptr_to_string(guid: *const GUID) -> WindowsResult<String> {
        unsafe {
            let guid = &*guid;
            Ok(format!("{:?}", guid))
        }
    }
}
