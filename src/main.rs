use windows::{core::*, Win32::UI::WindowsAndMessaging::*};

fn main() -> Result<()> {
    println!("{}", get_mouse_speed()?);
    Ok(())
}

fn get_mouse_speed() -> Result<i32> {
    let mut speed = 0;
    let result = unsafe {
        SystemParametersInfoW(
            SPI_GETMOUSESPEED,
            0,
            &mut speed as *mut _ as *mut std::ffi::c_void,
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
    };
    result.ok().and(Ok(speed))
}

fn set_mouse_speed(speed: i32) -> Result<()> {
    let result = unsafe {
        SystemParametersInfoW(
            SPI_SETMOUSESPEED,
            0,
            speed as *mut () as *mut std::ffi::c_void,
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
    };
    result.ok()
}
