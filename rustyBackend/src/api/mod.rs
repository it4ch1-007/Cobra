mod router;
mod connections;

pub fn start_api_server(){
    println!("Starting API server");
    router::start_router();
}