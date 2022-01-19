use std::env::args;
use wintendo_tools::woe::Woe;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyState, keybd_event,
    VIRTUAL_KEY, VK_CAPITAL, 
    KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP
};

fn get_key_state(vk: VIRTUAL_KEY) -> bool {
    let curr_state;
    unsafe {
        curr_state = GetKeyState(vk.into());
    }
    return curr_state & 1 == 1;
}

fn set_key_pressed(vk: VIRTUAL_KEY, pressed: bool) {
    let bscan = 0x45;
    unsafe {
        keybd_event(
            vk as u8, 
            bscan, 
            KEYEVENTF_EXTENDEDKEY | (if pressed {0} else {KEYEVENTF_KEYUP}), 
            0);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let target_state = match args().last().unwrap_or_default().as_str() {
        "enable" | "enabled" | "on" => Ok(true),
        "disable" | "disabled" | "off" => Ok(false),
        _ => Woe::result("Usage: shoutmode [enable | disable]"),
    }?;

    let mut curr_state = get_key_state(VK_CAPITAL);

        println!("Current Caps Lock state: {}", curr_state);

        if curr_state == target_state {
            println!("Nothing to do! Exiting.");
            return Ok(());
        }

        println!("Attempting to toggle state...");

        set_key_pressed(VK_CAPITAL, true);
        set_key_pressed(VK_CAPITAL, false);

        curr_state = get_key_state(VK_CAPITAL);

        println!("New Caps Lock state: {}", curr_state);
    
        if curr_state != target_state {
            return Woe::result("Failed to toggle state. Try running as administrator.")
        }

        Ok(())
}
