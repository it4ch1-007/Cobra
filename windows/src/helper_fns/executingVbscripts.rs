use std::fs::File;
use std::io::Write;
use std::io;
use std::thread;
use std::time::Duration;
use std::process::Command;

//These functions are used to handle different activities regarding the visual basic scripts
pub fn executeVbs(scriptName:&str)-> (){
    let script_path = "temp_script.vbs";
    let mut file = File::create(script_path).unwrap();
    match file.write_all(scriptName.as_bytes()){
        Ok(()) => {println!("done")},
        _ => {eprintln!("Error")}
    };
    drop(file); //close the file so that the file can be then executed
    
    let op = Command::new("cscript")
    .arg("//NoLogo")
    .arg(script_path)
    .output().expect("Error while executing the file!!");

    println!("{:?}",op);
    
    //Deleting the file after making its use
    std::fs::remove_file(script_path).expect("Error while deleting the file!!!");


}