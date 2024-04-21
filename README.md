# Sikontrol

## About

Renewed version of Sikontrol Legacy ([Sikontrol](https://github.com/sikelio/sikontrol-legacy) & [Sikontrol App](https://github.com/sikelio/sikontrol-app-legacy)).

It still desktop app for starting a websocket instance though [socketioxide](https://github.com/Totodore/socketioxide) to control the main and apps. Currently there is no Android or iOS app to connect to the instance, but it will come. I'm waiting on the release of the [V2 of Tauri](https://beta.tauri.app/blog/tauri-2-0-0-beta/) who will support mobile developpment.

This project is now written in [Typescript](https://www.typescriptlang.org/) and [Rust](https://www.rust-lang.org/) through the usage of [Tauri](https://tauri.app/). The mobile app will come later (may be with the usage of [Tauri V2](https://beta.tauri.app/blog/tauri-2-0-0-beta/)). This project still works under Socket IO but with [socketioxide](https://github.com/Totodore/socketioxide) crate.

The legacy version still work but isn't supported anymore. But I don't think you wanna see an ugly interface (it was my first functionnal project).

## Why doing a renew ?

[Electron JS](https://www.electronjs.org/) is nowadays not very appreciated because of the [Chromium](https://www.chromium.org/chromium-projects/) base layer and the usage of [Node JS](https://nodejs.org/en) for direct communication to the Windows API is really shitty.

So I decided to make a new version of this project, to learn [Rust](https://www.rust-lang.org/).

## How to use

Please refer to the [wiki](https://github.com/sikelio/sikontrol/wiki).

If something is missing please let me now by opening an [issue](https://github.com/sikelio/sikontrol/issues).

## Roadmap

You can either download the desktop app from the [release page](https://github.com/sikelio/sikontrol/releases) (coming soon) or clone the project and build it yourself.

The project is not yet ready but the basic functions (Play, Pause, Next, Prev) are yet implemented on the following Socket IO events by the usage of [socketioxide](https://github.com/Totodore/socketioxide).

* Play or Pause : `play_pause`
* Next Track : `next_track`
* Prev Track : `prev_track`
* Change Main Volume : `change_main_volume`
* Change Apps Volume : `change_app_volume`
* Mute Unmute Main Volume : `mute_unmute_main_volume`
* Mute Unmute Apps Volume : `mute_unmute_app_volume`

The following functions are yet not implemented but will be in some time.

* [IAudioSessionNotification](https://learn.microsoft.com/en-us/windows/win32/api/audiopolicy/nn-audiopolicy-iaudiosessionnotification) implementation : For getting new or deleted sessions
* [IAudioSessionEvents](https://learn.microsoft.com/fr-fr/windows/win32/api/audiopolicy/nn-audiopolicy-iaudiosessionevents) implementation (in progress) : For getting sessions updates
* mDNS implementation for findind easely the instance for a client device.
