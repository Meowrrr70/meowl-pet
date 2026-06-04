#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "linux")]
    {
        // WebKitGTK's DMA-BUF renderer breaks transparent windows on X11.
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

        // Force XWayland on Wayland sessions: native Wayland has no
        // always-on-top concept for regular xdg-shell surfaces, and
        // gtk-layer-shell doesn't integrate cleanly with KWin. Under
        // XWayland, KWin honors _NET_WM_STATE_ABOVE / _NET_WM_DESKTOP
        // exactly like on a real X11 session.
        std::env::set_var("GDK_BACKEND", "x11");
    }

    meowl_pet_lib::run()
}
