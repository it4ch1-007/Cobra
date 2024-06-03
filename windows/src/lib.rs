use std::{ffi::CString,ptr::null_mut};
use winapi::um::winuser::{MessageBoxA,MB_OK};

//The no_mangle macro allows us to have no mangling while making the compiled function and object names makin it easier to interact with other langauges like C or C++

//The extern keyword specifies the calling convention of the function.
#[no_mangle]
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

//The DLLs can be built by getting them in the path src/lib.rs where the --release option have to be used to compile a library.