// extern crate winapi;


// use std::mem;
// use std::process;
// use std::ptr;
// use std::ptr::null_mut;
// use windows_sys::Win32::System::Threading::PEB;
// use winapi::um::winnt::{PROCESS_ALL_ACCESS,PROCESS_QUERY_INFORMATION,PROCESS_VM_READ};
// use winapi::um::heapapi::{GetProcessHeap,HeapAlloc};
// use winapi::um::processthreadsapi::OpenProcess;
// use winapi::um::memoryapi::ReadProcessMemory;

// pub fn check_debug(){
//     unsafe{
//     let mut hProcess = process::id();//this gets the handle to the current process.
//     let mut lpPeb = null_mut();//This will be the pointer to the PEB 
//     //Open the process using OpenProcess

//     //This is the handle when I open the process using OpenProcess
//     let mut pHandle = OpenProcess(
//         PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
//         0,
//         hProcess,
//       );
//     if pHandle != ptr::null_mut(){
//         println!("openprocess successful");
//     }
    
//     //Now to get the address of the PEB
//     // GetProcessHeap();
//     let handle = GetProcessHeap();
//     lpPeb = HeapAlloc(handle,0,mem::size_of::<PEB> as usize);
//     ReadProcessMemory(pHandle,ptr::null_mut(),lpPeb,mem::size_of::<PEB> as usize,ptr::null_mut());

//     if lpPeb != ptr::null_mut() {
//         let pPeb = unsafe{&*(lpPeb as *mut PEB)};
//         //this is to treat the address of the lpPeb as a pointer of PEB type and then get me the value of that pointer's address.
//         //it is unsafe because the pointer dereferenced here which is ultimately the address stored inside the lpPeb variable can be null 

//         println!("BeingDebugged value: {:?}",pPeb.BeingDebugged);

//     }
//     else {
//         println!("Failed to get the PEB");
//     }
// }

// }


// extern crate winapi;
// use windows_sys::Win32::System::Threading::{PEB,NtCurrentTeb};
// use windows_sys::Win32::System::Threading::TEB;


// #[link(name = "ntdll")]
// extern "system" {
//     fn NtCurrentTeb() -> *mut TEB;
// }

// pub fn check_debug(){
//     unsafe{
//     let teb = NtCurrentTeb();
//     let peb = (*teb).ProcessEnvironmentBlock;
//     println!("Being Debugged Value: {:?}",(*peb).BeingDebugged);
//     }
// }


use std::arch::asm;

#[repr(C)]
struct PEB {
    reserved1: [u8; 2], 
    being_debugged: u8,
    reserved2: [u8; 57], 
}
pub unsafe fn check_debug() -> bool{
    let p_peb: *const PEB;
    
        asm!(
            "mov rax, gs:[0x60]",
            out("rax") p_peb,
            options(nomem,nostack)
        );
    
    
    (*p_peb).being_debugged!=0 //The parenthesis are reqd so that  the being_debugged is not deciphered before deref the pointer.
}