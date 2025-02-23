use std::ptr::null_mut;
use winapi::um::winuser::{ExitWindowsEx, EWX_LOGOFF};
use winapi::um::winnt::{TOKEN_ADJUST_PRIVILEGES, TOKEN_QUERY};
use winapi::shared::minwindef::FALSE;

pub fn logWindowsOff() -> Result<(),String>{ //This takes the type of the error that will occur
    unsafe{
        if ExitWindowsEx(EWX_LOGOFF,0) == 0{
            //Get error
           eprintln!("Error while logging Windows off!!!");
        }
    }
    Ok(())
}