use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::*,
    Win32::UI::WindowsAndMessaging::*,
};

// Add these two lines for mouse input detection
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LBUTTON};

// Add these for sleep and duration
use std::thread;
use std::time::Duration;

const CLASS_NAME: &str = "StealthOverlayWindow";

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            let mut ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
            ex_style |= (WS_EX_LAYERED.0 | WS_EX_TRANSPARENT.0) as i32;
            SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style);
            SetLayeredWindowAttributes(hwnd, COLORREF(0), 180, LWA_ALPHA);
            LRESULT(0)
        }
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            let text = to_wide("Stealth Overlay Active");
            TextOutW(hdc, 10, 10, &text); // ✅ fixed
            EndPaint(hwnd, &ps);
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn to_wide(s: &str) -> Vec<u16> {
    use std::os::windows::prelude::*;
    std::ffi::OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

fn main() -> Result<()> {
    unsafe {
        let h_instance = GetModuleHandleW(None)?;

        let class_name = to_wide(CLASS_NAME);

        let wc = WNDCLASSW {
            hInstance: h_instance.into(),
            lpszClassName: PCWSTR(class_name.as_ptr()),
            lpfnWndProc: Some(wnd_proc),
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as isize), // ✅ visible background
            ..Default::default()
        };

        RegisterClassW(&wc);

        let title: Vec<u16> = "Overlay\0".encode_utf16().collect();

        let hwnd = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOOLWINDOW | WS_EX_TOPMOST,
            PCWSTR(class_name.as_ptr()),
            PCWSTR(title.as_ptr()),
            WS_POPUP,
            100, 100, 600, 400,
            None,
            None,
            h_instance,
            None,
        );

        if hwnd.0 == 0 {
            panic!("CreateWindowExW failed");
        }

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

        let mut msg = MSG::default();

        let mut dragging = false;
        let mut drag_start = POINT::default();
        let mut window_start = RECT::default();

        loop {
            while PeekMessageW(&mut msg, HWND(0), 0, 0, PM_REMOVE).into() {
                if msg.message == WM_QUIT {
                    return Ok(());
                }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            let left_down = (GetAsyncKeyState(VK_LBUTTON.0 as i32) as u16 & 0x8000) != 0;

            if left_down {
                if !dragging {
                    dragging = true;
                    let _ = GetCursorPos(&mut drag_start);
                    let _ = GetWindowRect(hwnd, &mut window_start);
                } else {
                    let mut current = POINT::default();
                    let _ = GetCursorPos(&mut current);

                    let dx = current.x - drag_start.x;
                    let dy = current.y - drag_start.y;

                    let new_left = window_start.left + dx;
                    let new_top = window_start.top + dy;

                    let _ = SetWindowPos(
                        hwnd,
                        HWND_TOPMOST,
                        new_left,
                        new_top,
                        0,
                        0,
                        SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
                    );
                }
            } else {
                dragging = false;
            }

            thread::sleep(Duration::from_millis(10));
        }
    }
}
