//This will provide us the fn for memory allocation on the heap
extern crate winapi;
use windows::Win32::System::Memory::{GetProcessHeap, HeapAlloc, HeapFree};
use windows::Win32::Foundation::HANDLE;
use std::slice::from_raw_parts;
pub fn mem_alloc(){
    unsafe{
        let heap = GetProcessHeap(); //this gives is the handle to the heap of the current process.
        //Before using the functions like HeapAlloc or other Heap related functions realted to Windows API
        //We have to use this fn to get the handle to the heap of the current running process.

        if heap.is_null(){
            eprintln!("Could not get the heap of the process!!");
            //prints the error if the handle is null
        }

        let p_address = HeapAlloc(heap,0x8,100);
        //similar to calloc which initializes the memory allocated with null bytes.

        if p_address.is_null(){
            eprintln!("Could not allocate memory on the heap!!");
            return
        }
        //We can use the expect keyword but it can only be used if the value is err not null or any other constant determined value.

        println!("Memory allocated at: {:?}",p_address);
        //The debug macro can print any format irrespective of ther format specifiers.
        // return p_address;

        //JUST FOR TEST

        let string = "John".as_ptr() as *const u8;
        //the as_ptr converts the string into a pointer to its first character
        //cosnt u8 converts that into a u8 pointer.

        std::ptr::copy_nonoverlapping(string,p_address as *mut u8,100);
        //overwrite the 100 bytes of memory from the string to p_address as a mutable type.

        let content = from_raw_parts(p_address as *mut u8,100); //This will print the string a bytes
        println!("Memory copied: {:?}",content);
        //This will be printed as an arrar of integers as it is a set of bytes inside a bytestring.

        HeapFree(heap,0,p_address);
        //Using the fn to free memory and ensure ni leakage of memory through dereferencing of pointers.


    }
}