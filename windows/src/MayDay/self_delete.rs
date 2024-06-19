use std::env::current_exe;
use::std::mem::{size_of,size_of_val};
use::std::ptr::null_mut;
use::winapi::ctypes::c_void;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{CreateFileW,SetFileInformationByHandle,FILE_RENAME_INFO};
use winapi::um::handleapi::CloseHandle;
use winapi::um::heapapi::HeapFree;
use winapi::um::minwinbase::{FileDispositionInfo,FileRenameInfo};
use winapi::um::winnt::HEAP_ZERO_MEMORY;
use winapi::um::{fileapi::FILE_DISPOSITION_INFO,heapapi::{GetProcessHeap,HeapAlloc}};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;

pub const NULL: *mut c_void = 0 as *mut c_void;

pub fn self_del(){
    unsafe{
        let stream = "yeah";
        let stream_wide :Vec<u16>=stream.encode_utf16().chain(std::iter::once(0)).collect();
        let mut delete_file: FILE_DISPOSITION_INFO = std::mem::zeroed();
        //This struct is especially used to delete the files
        delete_file.DeleteFile = true.into();
        //into is the method to convert the given variable into the type it is supposed to be converted into.

        let length = size_of::<FILE_RENAME_INFO>();

        let rename_info = HeapAlloc(
            GetProcessHeap(),
            HEAP_ZERO_MEMORY,
            length as usize
        ) as *mut FILE_RENAME_INFO;

        (*rename_info).FileNameLength = (stream_wide.len()*size_of::<u16>()) as u32 - 2;

        std::ptr::copy_nonoverlapping(
            stream_wide.as_ptr(),
            (*rename_info).FileName.as_mut_ptr(),
            stream_wide.len()
        );

        let path = current_exe().unwrap();
        let path_str = path.to_str().unwrap();
        let mut full_path: Vec<u16> = path_str.encode_utf16().collect();
        full_path.push(0);
        //To make this a string of hexadecimal numerals


        let hFile = CreateFileW(
            full_path.as_ptr(),
            0x10000 | 0x100000,
            0x1,
            null_mut(),
            3 as u32,
            0,
            null_mut(),
        );

        if hFile == INVALID_HANDLE_VALUE{
            eprintln!("Error in creating file {:?}",GetLastError());
        }

        println!("Created the file : {:?} ",hFile);


        SetFileInformationByHandle(
            hFile,
            FileRenameInfo,
            rename_info as *mut _,
            length as u32,
        );
        CloseHandle(hFile);

        let hFile = CreateFileW(
            full_path.as_ptr(),
            0x10000 | 0x100000,
            0x1,
            null_mut(),
            3 as u32,
            0,
            null_mut(),
        );

        if hFile == INVALID_HANDLE_VALUE{
            eprintln!("Error in creating file {:?}",GetLastError());
        }

        println!("Created the file : {:?} ",hFile);


        SetFileInformationByHandle(
            hFile,
            FileDispositionInfo,
            &delete_file as *const _ as *mut _,
            size_of_val(&delete_file) as u32,
        );
        CloseHandle(hFile);
        HeapFree(
            GetProcessHeap(),
            0,
        rename_info as *mut c_void,
                    );    

            //This file first renames the file and then deletes it .    
    };
}