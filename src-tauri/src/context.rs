use arboard::Clipboard;
use serde::Serialize;

#[derive(Serialize)]
pub struct SystemContext {
    pub app: String,
    pub clipboard: String,
}

#[tauri::command]
pub fn get_context() -> SystemContext {
    let app = get_active_app();
    let clipboard = get_clipboard_contents();
    SystemContext { app, clipboard }
}

fn get_active_app() -> String {
    #[cfg(target_os = "macos")]
    {
        use applescript::AppleScript;
        let script = AppleScript::new(
            "tell application \"System Events\" to get name of (process 1 where frontmost is true)"
        ).unwrap();
        return script.execute().unwrap_or("Unknown".into());
    }

    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        use winapi::um::winuser::{GetForegroundWindow, GetWindowTextW, GetWindowTextLengthW};

        unsafe {
            let hwnd = GetForegroundWindow();
            let len = GetWindowTextLengthW(hwnd) + 1;
            let mut buffer: Vec<u16> = vec![0; len as usize];
            GetWindowTextW(hwnd, buffer.as_mut_ptr(), len);
            return OsString::from_wide(&buffer)
                .to_string_lossy()
                .trim()
                .to_string();
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let output = Command::new("xdotool")
            .args(["getwindowfocus", "getwindowname"])
            .output();

        match output {
            Ok(result) if result.status.success() => {
                String::from_utf8_lossy(&result.stdout).trim().to_string()
            }
            _ => "Unknown".to_string(),
        }
    }
}

fn get_clipboard_contents() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.get_text().unwrap_or_else(|_| "".to_string())
}
