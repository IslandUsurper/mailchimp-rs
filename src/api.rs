use std::collections::HashMap;
use crate::internal::api::Api;
use crate::internal::request::MailchimpRequest;
use crate::internal::MailchimpErrorType;
use serde::de::DeserializeOwned;

///
/// Métodos de encuestas soportados por el API
///
pub enum RequestMethod {
    /// HTTP Request POST
    Post,
    /// HTTP Request GET
    Get,
}

///
/// # mailchimp_rs API
///
/// Permite el acceso al API de Mailchimp si conoces bien los diferentes
/// endpoints.
/// Para más información sobre los endpoints ir a la página de desarrollos
/// mailchimp [Developers Mailchimp](https://developer.mailchimp.com/)
///
/// ## Ejemplo
///
/// ```
/// extern crate mailchimp_rs;
/// use mailchimp_rs::MailchimpApi;
///
/// let api = MailchimpApi::new("<DC>", "<API Key>");
/// println!("Api version: {}", api.version());
/// println!("Api domain: {}", api.domain());
/// ```
///
#[derive(Debug, Clone)]
pub struct MailchimpApi {
    i_api: Box<Api<MailchimpRequest>>,
}

impl MailchimpApi {
    ///
    /// Crea la nueva instancia del API
    ///
    /// Argumentos
    ///     dc: Mailchimp Datacenter
    ///     api_key: Mailchimp API KEY
    ///     http_transport: Interfaz por donde se harían las peticiones Get y Post al servicio
    pub fn new<'a>(dc: &'a str, api_key: &'a str) -> Self {
        MailchimpApi {
            i_api: Box::new(
                Api::<MailchimpRequest>::new(
                    dc,
                    api_key,
                    Box::new(MailchimpRequest::new()),
                )
            ),
        }
    }
    ///
    /// Devuelve el dominio
    ///
    pub fn domain(&self) -> String {
        self.i_api.domain()
    }

    ///
    /// Devuelve la version del API
    ///
    pub fn version(&self) -> String {
        self.i_api.api_version()
    }

    ///
    /// Realiza una petición HTTP al servidor
    ///
    /// ```
    /// extern crate mailchimp_rs;
    /// use std::collections::HashMap;
    /// use mailchimp_rs::{MailchimpApi, RequestMethod};
    /// use mailchimp_rs::AuthorizedAppType;
    /// fn main() {
    ///     let api = MailchimpApi::new("usX", "aac1e319006883125e18a89e529b5abb73de4c81-usX");
    ///     let data = api.call::<AuthorizedAppType>(RequestMethod::Get, "authorized-apps", HashMap::new());
    ///     match data {
    ///         Ok(resp) => {
    ///             for app in resp.apps.iter() {
    ///                 println!("{:?}", app)
    ///             }
    ///         },
    ///         Err(e) => println!("Error Title: {:?} \n Error detail {:?}", e.title, e.detail)
    ///     }
    /// }
    /// ```
    /// #Argumentos
    ///     `method`: Enum que representa el método de encuesta al servidor: Get | Post
    ///     `endpoint`: Cadena de texto con el endpoint de la API al que se requiere acceder, no debe comenzar por "/"
    ///     `payload`: Listado llave valor de los parametros o data
    ///
    pub fn call<'a, T>(
        &self,
        method: RequestMethod,
        endpoint: &'a str,
        payload: HashMap<String, String>,
    ) -> Result<T, MailchimpErrorType>
    where
        T: DeserializeOwned,
    {
        match method {
            RequestMethod::Get => self.i_api.get_edge(endpoint, payload),
            RequestMethod::Post => self.i_api.post_edge(endpoint, payload),
        }
    }
}
