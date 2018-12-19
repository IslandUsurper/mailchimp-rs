extern crate mailchimp;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

use std::collections::HashMap;
use mailchimp::{MailchimpApi, RequestMethod};
use mailchimp::AuthorizedAppsType;

fn main() {
    // Inicializando el dotenv
    dotenv().ok();
    // Obteniendo las variables de entornos con las credenciales de
    // mailchimp
    let mut env_mailchimp = env::vars().filter(|e| {
        e.0.to_string().contains("MAILCHIMP_")
    });
    let dc = env_mailchimp.next().unwrap().1;
    let apk = env_mailchimp.next().unwrap().1;
    // Inicializando el API, con las credenciales
    let api = MailchimpApi::new(&dc, &apk);
    // Se realiza una petición al endpoint /authorized-apps
    let data = api.call::<AuthorizedAppsType>(RequestMethod::Get, "authorized-apps", HashMap::new());

    match data {
        Ok(resp) => {
            // Se recorren todas las aplicaciones que tienen acceso al Mailchimp
            for app in resp.apps.iter() {
                println!("{:?}", app)
            }
        },
        Err(e) => println!("Error Title: {:?} \ndetail {:?}", e.title, e.detail)
    }
}