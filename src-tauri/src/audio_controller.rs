use std::slice::from_raw_parts;
use windows::{
    core::{
        Result, PWSTR
    },
    Win32::{
        Media::Audio::{
            eConsole, eRender, Endpoints::IAudioEndpointVolume, IMMDevice, IMMDeviceCollection, IMMDeviceEnumerator, MMDeviceEnumerator, DEVICE_STATE_ACTIVE
        }, System::Com::{
            CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_ALL, CLSCTX_INPROC_SERVER
        }
    }
};

pub struct AudioController {}

impl AudioController {
    pub fn get_audio_session() -> Result<Vec<String>> {
        unsafe {
            let mm_device_enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
            let mm_device_collection: IMMDeviceCollection = mm_device_enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;
            let count: u32 = mm_device_collection.GetCount()?;
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

    pub fn change_main_volume(volume: f32) -> windows::core::Result<()> {
        if volume < 0.0 || volume > 1.0 {
            panic!("The value send to set the volume need to be between or equals of 0.0 and 1.0");
        }

        unsafe {
            let _ = CoInitialize(None);

            let device_enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER)?;
            let default_device = device_enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;

            let endpoint_volume: IAudioEndpointVolume = default_device.Activate(CLSCTX_INPROC_SERVER, None)?;
            endpoint_volume.SetMasterVolumeLevelScalar(volume, std::ptr::null())?;

            CoUninitialize();

            Ok(())
        }
    }
}
