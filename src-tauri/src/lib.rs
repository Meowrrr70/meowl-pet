use evdev::{Device, InputEventKind, Key};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use tauri::{AppHandle, Emitter, LogicalPosition, LogicalSize, Manager};

static WORD_COUNT: AtomicU64 = AtomicU64::new(0);

fn find_keyboards() -> Vec<Device> {
    evdev::enumerate()
        .filter_map(|(_, device)| {
            device
                .supported_keys()
                .map(|keys| keys.contains(Key::KEY_A))
                .unwrap_or(false)
                .then_some(device)
        })
        .collect()
}

fn is_word_char(key: Key) -> bool {
    matches!(
        key,
        Key::KEY_Q | Key::KEY_W | Key::KEY_E | Key::KEY_R | Key::KEY_T
            | Key::KEY_Y | Key::KEY_U | Key::KEY_I | Key::KEY_O | Key::KEY_P
            | Key::KEY_A | Key::KEY_S | Key::KEY_D | Key::KEY_F | Key::KEY_G
            | Key::KEY_H | Key::KEY_J | Key::KEY_K | Key::KEY_L
            | Key::KEY_Z | Key::KEY_X | Key::KEY_C | Key::KEY_V | Key::KEY_B
            | Key::KEY_N | Key::KEY_M
            | Key::KEY_0 | Key::KEY_1 | Key::KEY_2 | Key::KEY_3 | Key::KEY_4
            | Key::KEY_5 | Key::KEY_6 | Key::KEY_7 | Key::KEY_8 | Key::KEY_9
    )
}

fn is_word_sep(key: Key) -> bool {
    matches!(key, Key::KEY_SPACE | Key::KEY_ENTER | Key::KEY_KPENTER)
}

fn watch_keyboard(mut device: Device, app: AppHandle, count_file: PathBuf) {
    std::thread::spawn(move || {
        let mut had_word_char = false;
        loop {
            let Ok(events) = device.fetch_events() else {
                break;
            };
            for event in events {
                let InputEventKind::Key(key) = event.kind() else {
                    continue;
                };
                if event.value() != 1 {
                    continue;
                }
                if is_word_char(key) {
                    had_word_char = true;
                } else if is_word_sep(key) {
                    if had_word_char {
                        let count = WORD_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
                        let _ = std::fs::write(&count_file, count.to_string());
                        let _ = app.emit("word-count", count);
                    }
                    had_word_char = false;
                }
            }
        }
    });
}


#[tauri::command]
fn get_word_count() -> u64 {
    WORD_COUNT.load(Ordering::Relaxed)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_word_count])
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let count_file = data_dir.join("count");

            let saved = std::fs::read_to_string(&count_file)
                .ok()
                .and_then(|s| s.trim().parse::<u64>().ok())
                .unwrap_or(0);
            WORD_COUNT.store(saved, Ordering::Relaxed);

            let keyboards = find_keyboards();
            if keyboards.is_empty() {
                eprintln!("No keyboard devices found. Add yourself to the 'input' group: sudo usermod -aG input $USER");
            }
            for keyboard in keyboards {
                watch_keyboard(keyboard, app.handle().clone(), count_file.clone());
            }

            const WIN_W: f64 = 200.0;
            const WIN_H: f64 = 260.0;
            const MARGIN_RIGHT: f64 = 20.0;
            const MARGIN_BOTTOM: f64 = 80.0;

            let webview_window = app
                .get_webview_window("main")
                .ok_or("window not found")?;

            let _ = webview_window.set_size(LogicalSize::new(WIN_W, WIN_H));
            let _ = webview_window.set_always_on_top(true);
            let _ = webview_window.set_skip_taskbar(true);
            let _ = webview_window.set_visible_on_all_workspaces(true);

            // outer_size() is unreliable pre-map on X11, so use our known size
            // along with monitor geometry to anchor bottom-right.
            if let Ok(Some(monitor)) = webview_window.current_monitor() {
                let scale = monitor.scale_factor();
                let mpos = monitor.position();
                let msize = monitor.size();
                let mx = mpos.x as f64 / scale;
                let my = mpos.y as f64 / scale;
                let mw = msize.width as f64 / scale;
                let mh = msize.height as f64 / scale;
                let x = mx + mw - WIN_W - MARGIN_RIGHT;
                let y = my + mh - WIN_H - MARGIN_BOTTOM;
                let _ = webview_window.set_position(LogicalPosition::new(x, y));
            }

            webview_window.show()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
