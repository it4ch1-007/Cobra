use rocket::get;

#[get("/listconnections")]
pub fn list_connections()->&'static str{
    "Listing connections"
}