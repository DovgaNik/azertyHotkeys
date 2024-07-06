use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;
fn main() {
    let mut key_pressed: bool = false;
    loop {
        let alt_gr_pressed: bool; // 0xA5
        let w_pressed: bool; // 0x57
        let x_pressed: bool; // 0x58

        unsafe { 
            alt_gr_pressed = is_button_pressed(0xA5);
            w_pressed = is_button_pressed(0x57);
            x_pressed = is_button_pressed(0x58);
        };

        println!("Alt Gr pressed: {alt_gr_pressed}, W pressed {w_pressed}, X pressed {x_pressed}, state of var {key_pressed}");
        if alt_gr_pressed && w_pressed && x_pressed && !key_pressed{
            unsafe { 
                keybd_event(0x41, 0, KEYBD_EVENT_FLAGS(0), 0);
                keybd_event(0x41, 0, KEYBD_EVENT_FLAGS(2), 0);
                key_pressed = true;
            };
        }

        if !alt_gr_pressed && !w_pressed && !x_pressed {
            key_pressed = false;
        }
    }
}


unsafe fn is_button_pressed(vkey_code: i32) -> bool {
    let press_result: i16 = GetAsyncKeyState(vkey_code);

    if press_result != 0 {
        return true;
    } else {
        return false;
    }
}