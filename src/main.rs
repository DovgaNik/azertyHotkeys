use windows::Win32::UI::Input::KeyboardAndMouse::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    loop {
        let alt_gr_pressed: bool; // 0xA5

        unsafe { 
            alt_gr_pressed = is_button_pressed(0xA5);
        };

        if alt_gr_pressed {
            let w_pressed: bool;
            let x_pressed: bool;

            unsafe {
                w_pressed = is_button_pressed(0x57); // 0x57
                x_pressed = is_button_pressed(0x58); // 0x58
            };

            if w_pressed{
                unsafe { 
                    send_less(); 
                };
            } else if x_pressed{
                unsafe { 
                    send_more(); 
                };
            }

        }
        sleep(Duration::from_millis(33));
    }
}

unsafe fn is_button_pressed(vkey_code: i32) -> bool {
    let press_result: i16 = GetAsyncKeyState(vkey_code);

    if press_result == -32768 {
        return true;
    } else {
        return false;
    }
}

unsafe fn send_key_up(vkey_code: u8) {
    keybd_event(vkey_code,
        0,
        KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP,
        0);
}

unsafe fn send_key_down(vkey_code: u8) {
    keybd_event(vkey_code,
        0,
        KEYEVENTF_EXTENDEDKEY | KEYBD_EVENT_FLAGS(0x00),
        0);
}

unsafe fn send_less() {
    send_key_up(0xA5); // releasing the alt gr key as it interferes with input      

    send_key_down(0xE2);    
    send_key_up(0xE2);  

    send_key_down(0xA5);
}

unsafe fn send_more() {
    send_key_up(0xA5); // releasing the alt gr key as it interferes with input

    send_key_down(0x10);

    send_key_down(0xE2);
    send_key_up(0xE2);

    send_key_up(0x10);

    send_key_down(0xA5);
}
