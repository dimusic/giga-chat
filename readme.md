## GigaChat

Simple cross-platform Twitch chat app that renders as a customizable transparent overlay on top of other windows.

Made with [Tauri](https://tauri.app/) and [Yew](https://yew.rs/)

### Demo

<img src="https://user-images.githubusercontent.com/1485977/223316965-1df79e11-f748-4170-97f4-6ac3b98ec60c.gif" width="380">

https://user-images.githubusercontent.com/1485977/222994391-84012fd7-4e2f-4658-a203-e0b9ab587e72.mp4

*Keeping up with important chat messages while driving*

### Features
* BetterTTV, FFZ, 7tv emotes support
* Toggle Always on top
* Adjust background color and opacity
* Remember size and position on the screen

### Development

You'll need to have [trunk](https://trunkrs.dev/) and [tauri-cli](https://crates.io/crates/tauri-cli) installed.

Run tauri in dev mode:

```
cargo tauri dev
```

Serve yew frontend app:

```
cd ./crates/yew-ui
trunk serve
```

