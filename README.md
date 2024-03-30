# Sikontrol

## About

Renewed version of Sikontrol Legacy ([Sikontrol](https://github.com/sikelio/sikontrol-legacy) & [Sikontrol App](https://github.com/sikelio/sikontrol-app-legacy)).

This project is now written in [Typescript](https://www.typescriptlang.org/) and [Rust](https://www.rust-lang.org/) through the usage of [Tauri](https://tauri.app/). The mobile app will come later (may be with the usage of [Tauri V2](https://beta.tauri.app/blog/tauri-2-0-0-beta/)). This project still works under Socket IO but with [socketioxide](https://github.com/Totodore/socketioxide) package.

The legacy version still work but isn't supported anymore.

## Why doing a renew ?

[Electron JS](https://www.electronjs.org/) is nowadays not very appreciated because of the [Chromium](https://www.chromium.org/chromium-projects/) base layer and the usage of [Node JS](https://nodejs.org/en) for direct communication to the Windows API is really shitty.

So I decided to make a new version of this project, to learn [Rust](https://www.rust-lang.org/).

## Roadmap

You can either download the desktop app from the [release page](https://github.com/sikelio/sikontrol/releases) (coming soon) or clone the project and build it yourself.

The project is not yet ready but the basic functions (Play, Pause, Next, Prev) are yet implemented on the following Socket IO events by the usage of [socketioxide](https://github.com/Totodore/socketioxide).

* Play or Pause : `play_pause`
* Next Track : `next_track`
* Prev Track : `prev_track`
* Change Main Volume : `change_main_volume` (needs a volume value between 0 and 1)
* Change Apps Volmue : `change_main_volume` (needs a pid of an app and a volume value between 0 and 1)

The following functions are yet not implemented but will be in some time.

* Main volume controller
* Individually app volume controller
