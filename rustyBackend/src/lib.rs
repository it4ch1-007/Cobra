use lazy_static::lazy_static;
use rocket::form::name;
use std::sync::Mutex;

pub struct GetName{
    pub name: String,
    pub cntr: u32,
}

lazy_static! {
    pub static ref GLOBAL_NAME: Mutex<GetName> = Mutex::new(GetName{
        name: "Target-".to_string(),
        cntr: 0,
    });
}

pub fn assign_name() -> String {
    let mut name_instance = GLOBAL_NAME.lock().unwrap();
    name_instance.cntr+=1;
    format!("{}{}",name_instance.name,name_instance.cntr)
}