use windows::{
    core::{
        implement, Result as WindowsResult, GUID, PCWSTR
    },
    Win32::{
        Foundation::BOOL,
        Media::Audio::{
            AudioSessionDisconnectReason, AudioSessionState, AudioSessionStateActive,
            AudioSessionStateInactive, DisconnectReasonDeviceRemoval,
            DisconnectReasonExclusiveModeOverride, DisconnectReasonFormatChanged,
            DisconnectReasonServerShutdown, DisconnectReasonSessionDisconnected,
            DisconnectReasonSessionLogoff, IAudioSessionEvents, IAudioSessionEvents_Impl
        }
    }
};

#[implement(IAudioSessionEvents)]
pub struct AudioEvents {}

impl AudioEvents {
    pub fn new() -> Self {
        Self {}
    }
}

#[allow(non_snake_case)]
impl IAudioSessionEvents_Impl for AudioEvents {
    fn OnDisplayNameChanged(&self, _new_display_name: &PCWSTR, _event_context: *const GUID) -> WindowsResult<()> {
        todo!()
    }

    fn OnIconPathChanged(&self, _new_icon_path: &PCWSTR, _event_context: *const GUID) -> WindowsResult<()> {
        todo!()
    }

    fn OnSimpleVolumeChanged(&self, new_volume: f32, new_mute: BOOL, _event_context: *const GUID) -> WindowsResult<()> {
        if new_mute.into() {
            println!("MUTE");
        } else {
            println!("Volume = {} percent", new_volume);
        }

        Ok(())
    }

    fn OnChannelVolumeChanged(&self, _channel_count: u32, _new_channel_volume_array: *const f32, _changed_channel :u32, _event_context: *const GUID) -> WindowsResult<()> {
        todo!()
    }

    fn OnGroupingParamChanged(&self, _new_grouping_param: *const GUID, _event_context: *const GUID) -> WindowsResult<()> {
        todo!()
    }

    #[allow(non_upper_case_globals)]
    fn OnStateChanged(&self, new_state: AudioSessionState) -> WindowsResult<()> {
        let psz_state: &str = match new_state {
            AudioSessionStateActive => "active",
            AudioSessionStateInactive => "inactive",
            _ => "unknown"
        };

        println!("New session state = {}", psz_state);

        Ok(())
    }

    #[allow(non_upper_case_globals)]
    fn OnSessionDisconnected(&self, disconnect_reason: AudioSessionDisconnectReason) -> WindowsResult<()> {
        let psz_reason: &str = match disconnect_reason {
            DisconnectReasonDeviceRemoval => "User removed the audio endpoint device.",
            DisconnectReasonServerShutdown => "Windows audio service has stopped.",
            DisconnectReasonFormatChanged => "Stream format changed for the device that the audio session is connected to.",
            DisconnectReasonSessionLogoff => "User logged off the Windows Terminal Services (WTS) session that the audio session was running in.",
            DisconnectReasonSessionDisconnected => "WTS session that the audio session was running in was disconnected.",
            DisconnectReasonExclusiveModeOverride => "Audio session was disconnected to make the audio endpoint device available for an exclusive-mode connection.",
            _ => "unknown",
        };

        println!("Audio session disconnected (reason: {})", psz_reason);

        Ok(())
    }
}
