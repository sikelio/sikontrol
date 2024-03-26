use std::slice::from_raw_parts;
use windows::{
    core::{
        Result, PWSTR
    },
    Win32::{
        Media::Audio::{
            eRender, IMMDevice, IMMDeviceCollection, IMMDeviceEnumerator, MMDeviceEnumerator, DEVICE_STATE_ACTIVE
        }, System::Com::{
            CoCreateInstance, CLSCTX_ALL
        }
    }
};

pub struct AudioController {}

impl AudioController {
    pub fn get_audio_session() -> Result<Vec<String>> {
        unsafe {
            let mm_device_enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
            let mm_device_collection: IMMDeviceCollection = mm_device_enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;
            let count = mm_device_collection.GetCount()?;
            let mut sessions: Vec<String> = Vec::new();

            for i in 0..count {
                let mm_device: IMMDevice = mm_device_collection.Item(i)?;
                let session_id: PWSTR = mm_device.GetId()?;

                sessions.push(AudioController::pwstr_to_string(session_id)?);
            }

            Ok(sessions)
        }
    }
    
    fn pwstr_to_string(pwstr: PWSTR) -> Result<String> {
        unsafe {
            let len = (0..).take_while(|&i| *pwstr.0.add(i) != 0).count();
            let slice = from_raw_parts(pwstr.0, len);
            Ok(String::from_utf16_lossy(slice))
        }
    }
}
