use serde::{
    Serialize, Deserialize
};
use regex::Regex;
use windows::{
    core::Interface,
    Win32::{
        Foundation::BOOL,
        Media::Audio::{
            eConsole, eRender, Endpoints::IAudioEndpointVolume, IAudioSessionControl,
            IAudioSessionControl2, IAudioSessionEnumerator, IAudioSessionManager2,
            IMMDevice, IMMDeviceEnumerator, ISimpleAudioVolume, MMDeviceEnumerator
        }, System::Com::{
            CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_ALL, CLSCTX_INPROC_SERVER
        }
    },
};

use crate::windows_utils::WindowsUtils;

pub struct AudioController {}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Session {
    pub pid: u32,
    pub name: String,
    pub volume: f32,
    pub is_muted: bool
}

impl AudioController {
    fn with_audio_session<F>(mut action: F)
    where F: FnMut(IAudioSessionControl, IAudioSessionControl2) {
        unsafe {
            CoInitialize(None).unwrap();

            let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).unwrap();
            let device: IMMDevice = enumerator.GetDefaultAudioEndpoint(eRender, eConsole).unwrap();
            let session_manager: IAudioSessionManager2 = device.Activate(CLSCTX_ALL, None).unwrap();
            let session_enumerator: IAudioSessionEnumerator = session_manager.GetSessionEnumerator().unwrap();

            let count: i32 = session_enumerator.GetCount().unwrap();

            for i in 0..count {
                let session_control: IAudioSessionControl = session_enumerator.GetSession(i).unwrap();
                let session_control_2: IAudioSessionControl2 = session_control.cast().unwrap();

                action(session_control, session_control_2);
            }

            CoUninitialize();
        }
    }

    pub fn get_audio_sessions() -> Vec<Session> {
        let mut sessions: Vec<Session> = Vec::new();

        AudioController::with_audio_session(|session_control, session_control_2| {
            unsafe {
                let pid: u32 = session_control_2.GetProcessId().unwrap();
                let name: String;

                if AudioController::contain_audio_srv(&WindowsUtils::pwstr_to_string(session_control_2.GetDisplayName().unwrap()).unwrap()) {
                    name = "System Sounds".to_string();
                } else {
                    name = WindowsUtils::pwstr_to_string(session_control_2.GetDisplayName().unwrap()).unwrap();
                }

                let audio_volume: ISimpleAudioVolume = session_control.cast().unwrap();
                let volume: f32 = audio_volume.GetMasterVolume().unwrap();
                let is_muted: bool = audio_volume.GetMute().unwrap() != BOOL(0);

                if !name.is_empty() {
                    sessions.push(Session { pid, name, volume, is_muted });
                }
            }
        });

        sessions
    }

    fn execute_main_action<T, F>(mut action: F) -> T
    where F: FnMut(&IAudioEndpointVolume) -> T  {
        unsafe {
            CoInitialize(None).unwrap();

            let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER).unwrap();
            let device: IMMDevice = enumerator.GetDefaultAudioEndpoint(eRender, eConsole).unwrap();
            let endpoint: IAudioEndpointVolume = device.Activate(CLSCTX_INPROC_SERVER, None).unwrap();

            let result: T = action(&endpoint);

            CoUninitialize();

            result
        }
    }

    fn execute_session_action<F>(pid: u32, mut action: F)
    where F: FnMut(&ISimpleAudioVolume) {
        unsafe {
            CoInitialize(None).unwrap();

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
                    action(&audio_volume);
                }
            }

            CoUninitialize();
        }
    }

    pub fn change_main_volume(volume: f32) {
        if volume < 0.0 || volume > 1.0 {
            return;
        }

        AudioController::execute_main_action(|endpoint| {
            unsafe {
                endpoint.SetMasterVolumeLevelScalar(volume, std::ptr::null()).ok();
            }
        });
    }

    pub fn get_main_volume_value() -> f32 {
        AudioController::execute_main_action(|endpoint| {
            unsafe {
                endpoint.GetMasterVolumeLevelScalar().unwrap()
            }
        })
    }

    pub fn mute_unmute_main_volume(action: String) {
        if action != "mute" && action != "unmute" {
            return;
        }

        let is_mute: i32 = match action.as_str() {
            "unmute" => 0,
            "mute" => 1,
            _ => 1
        };

        AudioController::execute_main_action(|endpoint| {
            unsafe {
                endpoint.SetMute(BOOL(is_mute), std::ptr::null()).ok();
            }
        });
    }

    pub fn change_app_volume(pid: u32, volume: f32) {
        if volume < 0.0 || volume > 1.0 {
            return;
        }

        AudioController::execute_session_action(pid, |audio_volume| {
            unsafe {
                audio_volume.SetMasterVolume(volume, std::ptr::null()).ok();
            }
        });
    }

    pub fn mute_unmute_app_volume(pid: u32, action: String) {
        if action != "mute" && action != "unmute" {
            return;
        }

        let is_mute: i32 = match action.as_str() {
            "unmute" => 0,
            "mute" => 1,
            _ => 1
        };

        AudioController::execute_session_action(pid, move |audio_volume| {
            unsafe {
                audio_volume.SetMute(BOOL(is_mute), std::ptr::null()).ok();
            }
        });
    }

    fn contain_audio_srv(input: &str) -> bool {
        let re: Regex = Regex::new(r"\b(?i)AudioSrv\.dll\b").unwrap();
        re.is_match(input)
    }
}
