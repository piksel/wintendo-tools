
use std::ffi::{OsStr, OsString};
use std::fmt::Pointer;
use std::mem::size_of;
use std::os::windows::prelude::OsStringExt;
use std::ptr;

use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Gdi::{EnumDisplayMonitors, GetMonitorInfoW, HMONITOR, HDC, MONITORINFO, MONITORINFOEXW};
use windows::Win32::Foundation::{LPARAM, BOOL};



fn get_screens(sink: ExtEventSink) -> Result<()> {

    unsafe {
        let sink_ptr = ptr::addr_of!(sink);
        EnumDisplayMonitors(None, ptr::null(), Some(get_screens_callback) , LPARAM(sink_ptr as isize));
    }
    Ok(())
}

unsafe extern "system" fn get_screens_callback(handle: HMONITOR, _: HDC, rect: *mut RECT, lparam: LPARAM) -> BOOL {

    // let sink = *(lparam.0 as *const ExtEventSink);

    let mut info = MONITORINFOEXW::default();
    info.monitorInfo.cbSize = size_of::<MONITORINFOEXW>() as u32;
    GetMonitorInfoW(handle, ptr::addr_of_mut!(info) as *mut MONITORINFO);

    let r = *rect;
    println!("Monitor size: {}x{}, position: {},{}", r.right, r.bottom, r.left, r.top);
    let device_name_raw = OsString::from_wide(&info.szDevice);
    let device_name = device_name_raw.to_str().unwrap_or("?");
    println!("Monitor device: {}", device_name);

    // sink.submit_command(selector, payload, target)

    BOOL::from(true)
}