use std::slice::from_raw_parts;
use windows::{
    core::{
        implement, IntoParam, Param, Result as WindowsResult, GUID, PCWSTR
    },
    Win32::{
        Foundation::BOOL,
        Media::Audio::{
            AudioSessionDisconnectReason, AudioSessionState, AudioSessionStateActive,
            AudioSessionStateExpired, AudioSessionStateInactive, DisconnectReasonDeviceRemoval,
            DisconnectReasonExclusiveModeOverride, DisconnectReasonFormatChanged,
            DisconnectReasonServerShutdown, DisconnectReasonSessionDisconnected,
            DisconnectReasonSessionLogoff, IAudioSessionEvents, IAudioSessionEvents_Impl
        }
    }
};

use crate::windows_utils::WindowsUtils;

#[implement(IAudioSessionEvents)]
pub struct AudioEvents {}

impl AudioEvents {
    pub fn new() -> Self {
        Self {}
    }
}

#[allow(non_snake_case)]
impl IAudioSessionEvents_Impl for AudioEvents {
    fn OnDisplayNameChanged(&self, new_display_name: &PCWSTR, _event_context: *const GUID) -> WindowsResult<()> {
        println!("Session new name: {}", WindowsUtils::pcwstr_to_string(new_display_name).unwrap());

        Ok(())
    }

    fn OnIconPathChanged(&self, new_icon_path: &PCWSTR, _event_context: *const GUID) -> WindowsResult<()> {
        println!("Session new icon path: {}", WindowsUtils::pcwstr_to_string(new_icon_path).unwrap());

        Ok(())
    }

    fn OnSimpleVolumeChanged(&self, new_volume: f32, new_mute: BOOL, _event_context: *const GUID) -> WindowsResult<()> {
        if new_mute.into() {
            println!("Session is now muted");
        } else {
            println!("Session new volume: {}", new_volume);
        }

        Ok(())
    }

    fn OnChannelVolumeChanged(&self, channel_count: u32, new_channel_volume_array: *const f32, changed_channel :u32, _event_context: *const GUID) -> WindowsResult<()> {
        let volumes: &[f32] = unsafe {
            from_raw_parts(new_channel_volume_array, channel_count as usize)
        };

        println!("Volume changed for channel {}: {}", changed_channel, volumes[changed_channel as usize]);

        Ok(())
    }

    fn OnGroupingParamChanged(&self, new_grouping_param: *const GUID, _event_context: *const GUID) -> WindowsResult<()> {
        println!("Session new grouping parameter: {}", WindowsUtils::guid_ptr_to_string(new_grouping_param).unwrap());

        Ok(())
    }

    #[allow(non_upper_case_globals)]
    fn OnStateChanged(&self, new_state: AudioSessionState) -> WindowsResult<()> {
        let psz_state: &str = match new_state {
            AudioSessionStateActive => "Inactive",
            AudioSessionStateInactive => "Active",
            AudioSessionStateExpired => "Expired",
            _ => "Unknown"
        };

        println!("New session state: {}", psz_state);

        Ok(())
    }

    #[allow(non_upper_case_globals)]
    fn OnSessionDisconnected(&self, disconnect_reason: AudioSessionDisconnectReason) -> WindowsResult<()> {
        let psz_reason: &str = match disconnect_reason {
            DisconnectReasonDeviceRemoval => "User removed the audio endpoint device",
            DisconnectReasonServerShutdown => "Windows audio service has stopped",
            DisconnectReasonFormatChanged => "Stream format changed for the device that the audio session is connected to",
            DisconnectReasonSessionLogoff => "User logged off the Windows Terminal Services (WTS) session that the audio session was running in",
            DisconnectReasonSessionDisconnected => "WTS session that the audio session was running in was disconnected",
            DisconnectReasonExclusiveModeOverride => "Audio session was disconnected to make the audio endpoint device available for an exclusive-mode connection",
            _ => "Unknown",
        };

        println!("Audio session disconnected: '{}'", psz_reason);

        Ok(())
    }
}

impl IntoParam<IAudioSessionEvents> for AudioEvents {
    unsafe fn into_param(self) -> Param<IAudioSessionEvents> {
        Param::Owned(self.into())
    }
}
