// src/titlebar.rs
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use crate::utils::to_wide;

fn rgb(r: u8, g: u8, b: u8) -> COLORREF {
    COLORREF((r as u32) | ((g as u32) << 8) | ((b as u32) << 16))
}

pub fn draw_titlebar(hwnd: HWND, hdc: HDC, width: i32) {
    unsafe {
        // Full-width background
        let bg_brush = CreateSolidBrush(rgb(255, 255, 255));
        let rect = RECT { left: 0, top: 0, right: width, bottom: 40 };
        FillRect(hdc, &rect, bg_brush);
        DeleteObject(bg_brush);

        // App Title
        let title = to_wide("SilentGlass");
        SetBkMode(hdc, TRANSPARENT);
        SetTextColor(hdc, rgb(32, 32, 32));
        TextOutW(hdc, 20, 12, &title);

        // Modern-style buttons with spacing
        draw_button(hdc, width - 90, 10, 20, 20, rgb(232, 17, 35));  // Close
        draw_button(hdc, width - 60, 10, 20, 20, rgb(255, 185, 0));   // Maximize
        draw_button(hdc, width - 30, 10, 20, 20, rgb(0, 120, 212));   // Minimize
    }
}

fn draw_button(hdc: HDC, x: i32, y: i32, w: i32, h: i32, color: COLORREF) {
    unsafe {
        let brush = CreateSolidBrush(color);
        let old = SelectObject(hdc, brush);
        Rectangle(hdc, x, y, x + w, y + h);
        SelectObject(hdc, old);
        DeleteObject(brush);
    }
}

fn draw_circle(hdc: HDC, x: i32, y: i32, color: COLORREF) {
    unsafe {
        let brush = CreateSolidBrush(color);
        let old_brush = SelectObject(hdc, brush);
        Ellipse(hdc, x, y, x + 12, y + 12);
        SelectObject(hdc, old_brush);
        DeleteObject(brush);
    }
}

pub fn handle_titlebar_click(x: i32, y: i32, hwnd: HWND) {
    if y >= 10 && y <= 30 {
        if x >= 510 && x <= 530 {
            unsafe { PostQuitMessage(0); } // Close
        } else if x >= 540 && x <= 560 {
            unsafe { ShowWindow(hwnd, SW_MINIMIZE); } // Minimize
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
