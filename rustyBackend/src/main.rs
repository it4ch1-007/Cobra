<<<<<<< HEAD
//This will be used for connection among all the peers and will allow the malwares in the peer targetted networks to share the information among each other too.
//The malware will send the info of the user and it will be sent to the room name on that basis only.

use std::{borrow::BorrowMut, collections::HashMap, hash::Hash, net::{TcpListener, TcpStream}, ptr::read, sync::RwLock};
use futures::{channel::mpsc::Sender, io::Lines};
use rocket::form::name::Name;
use rustyBackend::assign_name;
use tokio::{self, sync::broadcast};
use tokio_utils::codec::{FramedRead,FramedWrite,LinesCodec};

struct Room{
    tx: Sender<String>,
}
impl Room{
    fn new() -> Self{
        let (mut tx,_) = broadcast::channel(100);
        Self { tx: tx }
    }
}
struct Rooms(Arc<RwLock<Hashmap<String,Room>>>);

impl Rooms{
    fn join_room(self,room_name: &str){
        let read_guard = self.0.read().unwrap();
        if let(Some(room)) = read_guard.get(room){
            room.tx.clone()
        }

        let write_guard = self.0.write().unwrap();
        if let(Some(room)) = write_guard.entry(room_name).or_insert(Room::new()){
            room.tx.clone()
        }
    }
    fn list_rooms(self){
        let mut list: Vec<_>  = self.0.read().unwrap().iter().map(|(name,room)| (name.to_owned(),room.tx.receiver_count())).collect();
        //Getting the| receivers of a transmitter that is initialized inside a room will give us the number of clients inside the room.
        list.sort_by(|a,b,|{
            use std::cmp::Ordering::*;
            //This is sorted in order so that the names and the participants number sync with each other.
            match b.1.cmp(&a.1){
                Equal => a.0.cmp(&b.0),
                ordering => ordering,
            }
        });
        list
    }
}



struct Names(Arc<Mutex<HashSet<String>>>);

impl Names{
    fn new() {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }
    fn insert(&self,name:&str) {
        self.0.lock().insert(name)
    }
    fn delete(&self,name:&str) {
        self.0.lock().remove(name)
    }
    fn get_name(&self) -> String{
        let mut name = assign_name();
        let mut guard = self.0.lock().unwrap();
        while !guard.insert(name){
            name = assign_name();
        }
        name
    }
}

#[tokio::main]
async fn main(){
    let mut server = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let (mut tx,_) = broadcast::channel(100);
    let rooms = Rooms::new();
    let names = Names::new();
    loop{
        let (mut tx,_) = server.accept().await.unwrap();
        tokio::spawn(handle_connection(tcp,tx.clone(),names,rooms))
    }
}

async fn handle_connection(tcp:TcpStream,tx:Sender<String>,names:Names,rooms:Rooms){
    let (mut reader,mut writer) = tcp.split();
    let mut stream = FramedRead::new(reader,LinesCodec::new());
    let mut sink = FramedWrite::new(writer,LinesCodec::new());
    let mut name = names.get_name();
    sink.send("You are {}",name);
    let mut room_name = "main".to_string();
    let mut room_tx = rooms.join_room(&room_name);
    let mut room_rx = room_tx.subscribe();
    loop{
        tokio::select! {
            msg = stream.next() =>{
                let user_msg = match msg{
                    Some(msg) => msg.unwrap(),
                    None => break, 
                };
                if user_msg.starts_with("/rooms"){
                    rooms.list_rooms();
                }
                else if user_msg.starts_with("/name"){
                    let new_name = user_msg
                    .split_ascii_whitespaces()
                    .nth(1)
                    .unwrap();
                let mut status = names.insert(new_name);
                if status{
                    room_tx.send(format!("{} is now {}",name,new_name));
                }
                else{
                    sink.send(format!("No this name: {} is taken already",new_name));
                    }
                }
                else if user_msg.starts_with("/room"){
                    let new_room = user_msg
                    .split_ascii_whtespaces()
                    .nth(1)
                    .unwrap()
                    .to_string();
                if room_name == new_room{
                    sink.send(format!("You are already in the room: {}",room_name));
                    continue;
                }
                room_tx.send(format!("{} left room: {}",name,room_name));

                //Changing the transmitter
                room_tx = rooms.join_room(new_room);
                room_name = new_room.to_string();
                room_rx = room_tx.subscribe();
                sink.send("You joined {}",room_name);
                room_rx.send("{} joined {}",name,room_name);

                }
            }

            peer_msg = roomtx.recv() => {
               sink.send(peer_msg.unwrap()).await.unwrap();
            }

        };
        room_tx.send("{} left the room: {}",name,room_name);
        names.remove(&name);
    }
=======
mod cmd;
mod api;
fn main(){
    cmd::parse_cmdline_args()
>>>>>>> parent of bf02cb4 (Fixes)
}