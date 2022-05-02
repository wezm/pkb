#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    pkb::web::rocket()
}
