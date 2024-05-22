use rocket::Rocket;
use rocket::Build;
use rocket::response::content;
use crate::CONFIG;

#[get("/world")]
fn world() -> content::RawHtml<String> {
    let mut localinventory = "disabled";
    let mut networkdiscovery = "disabled";
    let mut networkinventory = "disabled";
    let mut deploy = "disabled";
    if CONFIG.localinventory.enabled {
        localinventory = "enabled";
    }
    if CONFIG.networkdiscovery.enabled {
        networkdiscovery = "enabled";
    }
    if CONFIG.networkinventory.enabled {
        networkinventory = "enabled";
    }
    if CONFIG.deploy.enabled {
        deploy = "enabled";
    }

    content::RawHtml(format!("<h1>Hello, world!</h1><br>
Localinventory: {localinventory}<br>
Networkdiscovery: {networkdiscovery}<br>
Networkinventory: {networkinventory}<br>
Deploy: {deploy}<br>
"))
}

#[rocket::launch]
pub fn rocket() -> Rocket<Build> {
    let figment = rocket::Config::figment()
        .merge(("port", 62354));
println!("CONFIG: {:?}", figment);
    rocket::custom(figment)
        .mount("/hello", routes![world])
}
