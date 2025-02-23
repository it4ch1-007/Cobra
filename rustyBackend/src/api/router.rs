use rocket::{routes,main};
use super::connections;

#[main]
pub async fn start_router() -> Result<(), rocket::Error>  {
    let _rocket = rocket::build()
    .mount("/",routes![connections::list_connections])
    .launch()
    .await?;
    Ok(())
}