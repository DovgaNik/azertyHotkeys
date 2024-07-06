use windows::Win32::UI::Input::KeyboardAndMouse::*;
use std::thread::sleep;
use std::time::Duration;

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


        if w_pressed && alt_gr_pressed && !key_pressed{
            unsafe { 
                send_less(); 
            };
            key_pressed = true;
        }

        if x_pressed && alt_gr_pressed && !key_pressed{
            unsafe { 
                send_more(); 
            };
            key_pressed = true;
        }

        if !alt_gr_pressed && !w_pressed && !x_pressed {
            key_pressed = false;
        }
        sleep(Duration::from_millis(50));
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
}

unsafe fn send_more() {
    send_key_up(0xA5); // releasing the alt gr key as it interferes with input

    send_key_down(0x10);

    send_key_down(0xE2);
    send_key_up(0xE2);

    send_key_up(0x10);
}
