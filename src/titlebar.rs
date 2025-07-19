// src/titlebar.rs
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use crate::utils::to_wide;

fn rgb(r: u8, g: u8, b: u8) -> COLORREF {
    COLORREF((r as u32) | ((g as u32) << 8) | ((b as u32) << 16))
}

pub fn draw_titlebar(_hwnd: HWND, hdc: HDC, width: i32) {
    unsafe {
        // Full-width background
        let bg_brush = CreateSolidBrush(rgb(255, 255, 245));
        let rect = RECT { left: 0, top: 0, right: width, bottom: 40 };
        FillRect(hdc, &rect, bg_brush);
        DeleteObject(bg_brush);

        // App Title
        let title = to_wide("SilentGlass");
        let mut size = SIZE::default();

        GetTextExtentPoint32W(hdc, &title, &mut size);
        SetBkMode(hdc, TRANSPARENT);

        let text_x = (width - size.cx) / 2;

        SetTextColor(hdc, rgb(32, 32, 32));
        TextOutW(hdc, text_x, 12, &title);
    }
}

pub fn handle_titlebar_click(x: i32, y: i32, hwnd: HWND) {
    if y >= 10 && y <= 30 {
        if x >= 510 && x <= 530 {
            unsafe { PostQuitMessage(0); } // Close
        } else if x >= 540 && x <= 560 {
            toggle_maximize(hwnd); // Maximize or restore
        } else if x >= 570 && x <= 590 {
            toggle_opacity(hwnd); // Stealth mode
        }
    }
}


fn toggle_opacity(hwnd: HWND) {
    use std::cell::RefCell;
    thread_local!{
        static VISIBLE: RefCell<bool> = RefCell::new(true);
    }

    VISIBLE.with(|v| {
        let mut visible = v.borrow_mut();
        *visible = !*visible;
        unsafe {
            SetLayeredWindowAttributes(hwnd, rgb(0,0,0), if *visible { 255 } else { 20 }, LWA_ALPHA);
        }
    })
}

fn toggle_maximize(hwnd: HWND) {
    use std::cell::RefCell;
    thread_local! {
        static IS_MAXIMIZED: RefCell<bool> = RefCell::new(false);
    }

    IS_MAXIMIZED.with(|state| {
        let mut is_maximized = state.borrow_mut();
        unsafe {
            if *is_maximized {
                ShowWindow(hwnd, SW_RESTORE);
            } else {
                ShowWindow(hwnd, SW_MAXIMIZE);
            }
        }
        *is_maximized = !*is_maximized;
    });
}
