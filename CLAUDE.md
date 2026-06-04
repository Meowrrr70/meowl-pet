# Meowl desktop pet

Meowl desktop pet is a small desktop pet that stays on your linux desktop and counts the number of words you type.
It uses the famous meme character "Meowl".

## Stack

- Rust + Tauri 2
  - Svelte (frontend)
- `evdev` for global keyboard input (requires the user to be in the `input` group)

## Platform notes

- Developed and tested on **KDE Plasma 6** (both X11 and Wayland sessions).
- Always-on-top, skip-taskbar and visible-on-all-workspaces are set via Tauri's native APIs from `setup()` — KWin honors the corresponding `_NET_WM_*` hints on X11.
- The window is positioned bottom-right using monitor geometry. `outer_size()` is unreliable pre-map on X11, so we use the known logical size (`WIN_W`/`WIN_H`) for the calculation instead of querying.
- `main.rs` sets two env vars on Linux before Tauri initializes GTK/WebKit:
  - `WEBKIT_DISABLE_DMABUF_RENDERER=1` — transparent undecorated windows would otherwise be mapped but render nothing.
  - `GDK_BACKEND=x11` — forces XWayland on Wayland sessions. Native Wayland (xdg-shell) has no `_NET_WM_STATE_ABOVE` equivalent for regular surfaces, and `gtk-layer-shell` doesn't integrate cleanly with KWin. Under XWayland, KWin treats the window like any other X11 client and respects the hints.
- A proper Wayland-native path (`wlr-layer-shell` via `wayland-client` + `wayland-protocols-wlr`) is possible but requires extracting the `wl_surface` from the WebKit window and driving the layer-surface lifecycle. Not worth the complexity here.

## Word counting

- A background thread per keyboard device polls `evdev` events.
- A "word" is counted when a sequence of alphanumeric key-downs is terminated by space or enter.
- The count is persisted to `app_data_dir/count` and emitted to the frontend as the `word-count` event.
