mod titlebar;
mod utils;
use windows::{
    core::*,
    // w,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    // Win32::Graphics::Gdi::COLORREF,
    Win32::UI::Input::KeyboardAndMouse::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW, // ✅ This is what was missing
};

use titlebar::*;

fn rgb(r: u8, g: u8, b: u8) -> COLORREF {
    COLORREF((r as u32) | ((g as u32) << 8) | ((b as u32) << 16))
}


fn main() -> Result<()>{
    unsafe {
        let h_instance = GetModuleHandleW(None)?;

        let class_name = w!("StealthWindow");

        let wc = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hInstance: h_instance.into(),
            lpszClassName: class_name,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            ..Default::default()
        };

        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TOOLWINDOW | WS_EX_TOPMOST,
            class_name,
            w!(""),
            WS_POPUP,
            0,
            0,
            600,
            400,
            None,
            None,
            h_instance,
            Some(std::ptr::null()),
        );

        RegisterHotKey(hwnd, 1, HOT_KEY_MODIFIERS(MOD_CONTROL.0 as u32), 'Q' as u32);
        RegisterHotKey(hwnd, 2, HOT_KEY_MODIFIERS(0), VK_ESCAPE.0 as u32);

        let _ = SetLayeredWindowAttributes(hwnd, rgb(0, 255, 0), 128, LWA_ALPHA);
        
        let _ = ShowWindow(hwnd, SW_SHOW);
        let _ = UpdateWindow(hwnd);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, HWND(0), 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    Ok(())
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => {
            unsafe {
                UnregisterHotKey(hwnd, 1);
                UnregisterHotKey(hwnd, 2);
                // PostQuitMessage(0);
            };
            LRESULT(0)
        }
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
            let mut rect = RECT::default();

            unsafe {
                GetClientRect(hwnd, &mut rect); 
                let width = rect.right - rect.left;
            
                draw_titlebar(hwnd, hdc, width);
                EndPaint(hwnd, &ps); 
            }
            LRESULT(0)
        }
        WM_LBUTTONDOWN => {
            // let x = (lparam.0 & 0xFFFF) as i32;
            // let y = ((lparam.0 >> 16) & 0xFFFF) as i32;

            // handle_titlebar_click(x, y, hwnd);

            // if y <= 40 {
                unsafe {
                    ReleaseCapture();
                    SendMessageW(hwnd, WM_NCLBUTTONDOWN, WPARAM(HTCAPTION as usize), LPARAM(0));
                }
            // }

            LRESULT(0)
        }
        WM_HOTKEY => {
            match wparam.0 {
                1 => {
                    println!("Ctrl + Q pressed → quitting");
                    unsafe {
                        let is_visible = IsWindowVisible(hwnd).as_bool();
                        ShowWindow(hwnd, if is_visible { SW_HIDE } else { SW_SHOW });
                    }
                },
                2 => {
                    println!("ESC pressed → quitting");
                    unsafe {
                        PostQuitMessage(0);
                    }
                },
                _ => {}
            }
            LRESULT(0)

        }
        _ => unsafe {
            DefWindowProcW(hwnd, msg, wparam, lparam)
        },
    }
}
