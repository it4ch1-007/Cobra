
extern crate winapi;
extern crate widestring;

#[cfg(windows)]

// use widestring::WideCString;
use std::ffi::CString;
use std::ptr::null_mut;
use winapi::shared::minwindef::DWORD;
use winapi::um::winuser::FindWindowW;
use winapi::um::handleapi::CloseHandle;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
use winapi::um::processthreadsapi::{CreateRemoteThread, OpenProcess};
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE, PROCESS_ALL_ACCESS};
use winapi::shared::ntdef::NULL;
use winapi::um::winbase::INFINITE;
use winapi::um::winuser::{FindWindowA, GetWindowThreadProcessId};
use winapi::shared::windef::HWND;
use winapi::ctypes::c_void;

// Your function definitions and code here



pub fn inject_dll(processName:&String,dllName:&String){
    
    let class_name = CString::new("ApplicationFrameWindow").unwrap();
    // The window name might be "Calculator", but this can vary.
    let window_name = CString::new("Calculator").unwrap();
    unsafe{let proc_wnd:HWND = FindWindowA(class_name.as_ptr(),window_name.as_ptr());
    let mut pid:DWORD = 0;
    let threadId = GetWindowThreadProcessId(proc_wnd,&mut pid);
    let dllSize = dllName.len();
    let process = OpenProcess(PROCESS_ALL_ACCESS,false as i32,pid);
    
    if process == NULL{
        eprintln!("Failed to open the handle to the process!!");
        return
    }


    //To allocate the memory inside the memory of the process.
    let buffer = VirtualAllocEx(
        process,
        null_mut(),
        dllSize,
        MEM_COMMIT|MEM_RESERVE,
        PAGE_READWRITE,
    );

    if buffer == null_mut(){
        eprintln!("Failed to allocate the memory inside the remote process!!");
        return
    }

    WriteProcessMemory(
        process,
        buffer,
        dllName.as_ptr() as *const c_void,
        dllSize,
        null_mut()
    );

    //to write the whole dll as a binary into the allocated address 


    let kernel32 = GetModuleHandleA("kernel32.dll\0".as_ptr() as *const _);

    let load_lib_fn = GetProcAddress(kernel32,"LoadLibrary0".as_ptr() as *const _);

    let thread = CreateRemoteThread(
        process,
        null_mut(),
        0,
        Some(std::mem::transmute(load_lib_fn)),
        buffer,
        0,
        null_mut(),
    );

    if thread.is_null(){
        eprintln!("Failed to create thread in the process!!");
    }

    WaitForSingleObject(thread,INFINITE);
    //this will wait for infinite time until and unless the thread finishes its execution.
    CloseHandle(thread);
    CloseHandle(process);
    println!("DLL INJECTED SUCCESSFULLY....");
    return 
}

//In rust we also have to hook the fn inside the dll to execute it after injection only injection will not execute the fn of the dll of rust.
}