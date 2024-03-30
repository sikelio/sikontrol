use std::{slice::from_raw_parts, u32};
use serde::{Serialize, Deserialize};
use windows::{
    core::{
        Interface, Result as WindowsResult, PWSTR
    },
    Win32::{
        Media::Audio::{
            eConsole, eRender, Endpoints::IAudioEndpointVolume, IAudioSessionControl,
            IAudioSessionControl2, IAudioSessionEnumerator, IAudioSessionManager2,
            IMMDevice, IMMDeviceEnumerator, ISimpleAudioVolume, MMDeviceEnumerator
        }, System::Com::{
            CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_ALL, CLSCTX_INPROC_SERVER
        },
    },
};

pub struct AudioController {}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub pid: u32,
    pub name: String
}

impl AudioController {
    pub fn get_audio_sessions() -> Vec<Session> {
        unsafe {
            CoInitialize(None).unwrap();

            let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).unwrap();
            let device: IMMDevice = enumerator.GetDefaultAudioEndpoint(eRender, eConsole).unwrap();
            let session_manager: IAudioSessionManager2 = device.Activate(CLSCTX_ALL, None).unwrap();
            let session_list: IAudioSessionEnumerator = session_manager.GetSessionEnumerator().unwrap();

            let count: i32 = session_list.GetCount().unwrap();
            let mut sessions: Vec<Session> = Vec::new();

            for i in 0..count {
                let session_control: IAudioSessionControl = session_list.GetSession(i).unwrap();
                let session_control_2: IAudioSessionControl2 = session_control.cast().unwrap();

                let pid: u32 = session_control_2.GetProcessId().unwrap();
                let name: String = AudioController::pwstr_to_string(session_control_2.GetDisplayName().unwrap()).unwrap();

                if !name.is_empty() {
                    sessions.push(Session { pid, name });
                }
            }

            sessions
        }
    }

    fn pwstr_to_string(pwstr: PWSTR) -> WindowsResult<String> {
        unsafe {
            let len = (0..).take_while(|&i| *pwstr.0.add(i) != 0).count();
            let slice = from_raw_parts(pwstr.0, len);

            Ok(String::from_utf16_lossy(slice))
        }
    }

    pub fn change_main_volume(volume: f32) {
        if volume < 0.0 || volume > 1.0 {
            return;
        }

        unsafe {
            let _ = CoInitialize(None);

            let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER).unwrap();
            let device: IMMDevice = enumerator.GetDefaultAudioEndpoint(eRender, eConsole).unwrap();

            let endpoint: IAudioEndpointVolume = device.Activate(CLSCTX_INPROC_SERVER, None).unwrap();
            let _ = endpoint.SetMasterVolumeLevelScalar(volume, std::ptr::null());

            CoUninitialize();
        }
    }

    pub fn change_app_volume(pid: u32, volume: f32) {
        if volume < 0.0 || volume > 1.0 {
            return;
        }

        unsafe {
            let _ = CoInitialize(None);
            let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER).unwrap();
            let device: IMMDevice = enumerator.GetDefaultAudioEndpoint(eRender, eConsole).unwrap();
            let session_manager: IAudioSessionManager2 = device.Activate(CLSCTX_ALL, None).unwrap();
            let session_enumerator: IAudioSessionEnumerator = session_manager.GetSessionEnumerator().unwrap();

            let count: i32 = session_enumerator.GetCount().unwrap();

            for i in 0..count {
                let session_control: IAudioSessionControl = session_enumerator.GetSession(i).unwrap();
                let session_control_2: IAudioSessionControl2 = session_control.cast().unwrap();
                let session_pid: u32 = session_control_2.GetProcessId().unwrap();

                if session_pid == pid {
                    let audio_volume: ISimpleAudioVolume = session_control_2.cast().unwrap();
                    let _ = audio_volume.SetMasterVolume(volume, std::ptr::null());
                }
            }

            CoUninitialize();
        }
    }
}