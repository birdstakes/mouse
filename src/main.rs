use clap::Parser;
use windows::{core::*, Win32::UI::WindowsAndMessaging::*};

#[derive(Parser)]
#[clap(about = "Get or set mouse speed")]
struct Args {
    /// New mouse speed
    speed: Option<i32>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let old_speed = get_mouse_speed()?;

    if let Some(new_speed) = args.speed {
        set_mouse_speed(new_speed)?;
        println!("Old mouse speed: {}", old_speed);
        println!("New mouse speed: {}", new_speed);
    } else {
        println!("Current mouse speed: {}", old_speed);
    }

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
