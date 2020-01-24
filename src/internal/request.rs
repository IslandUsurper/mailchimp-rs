use lazy_static::lazy_static;
use reqwest::header::HeaderMap;
use reqwest::{
    blocking::{Client, Response},
    Error, StatusCode, Url,
};
use serde::ser::Serialize;
use serde_json;

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

// import macro error
use log::error;

use super::error_type::MailchimpErrorType;

// Define un aleas generico al Result para MailchimpErrorType
pub type MailchimpResult<T> = Result<T, MailchimpErrorType>;

///
/// BasicAuth
///
#[derive(Debug, Clone)]
pub struct BasicAuth {
    pub username: String,
    pub api_token: String,
}

///
/// Definición que deben cumplir para poder extaer datos mediante HTTP
///
pub trait HttpReq {
    ///
    /// Función para leer los recursos desde el servidor
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: HeaderMap
    ///
    fn get(
        &self,
        url: Url,
        headers: HeaderMap,
        basic_auth: &Option<BasicAuth>,
    ) -> MailchimpResult<String>;
    ///
    /// Función para crear algún recurso en el servidor
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: Headers
    ///     payload: Datos a enviar a la URL especificada
    ///
    fn post<P>(
        &self,
        url: Url,
        headers: HeaderMap,
        payload: P,
        basic_auth: &Option<BasicAuth>,
    ) -> MailchimpResult<String>
    where
        P: Serialize;
    ///
    /// Función para Actualizar algún recurso en el servidor
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: Headers
    ///     payload: Datos a enviar a la URL especificada
    ///
    fn patch<P>(
        &self,
        url: Url,
        headers: HeaderMap,
        payload: P,
        basic_auth: &Option<BasicAuth>,
    ) -> MailchimpResult<String>
    where
        P: Serialize;
    ///
    /// Función para Actualizar algún recurso en el servidor
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: Headers
    ///     payload: Datos a enviar a la URL especificada
    ///
    fn put<P>(
        &self,
        url: Url,
        headers: HeaderMap,
        payload: P,
        basic_auth: &Option<BasicAuth>,
    ) -> MailchimpResult<String>
    where
        P: Serialize;
    ///
    /// Función para eliminar algun recursos en el servidor
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: HeaderMap
    ///
    fn delete(
        &self,
        url: Url,
        headers: HeaderMap,
        basic_auth: &Option<BasicAuth>,
    ) -> MailchimpResult<String>;
}

///
/// MailchimpRequest
///
#[derive(Debug, Clone)]
pub struct MailchimpRequest {}

impl MailchimpRequest {
    ///
    /// Devuelve una instancia nueva
    ///
    pub fn new() -> Self {
        MailchimpRequest {}
    }
}

impl HttpReq for MailchimpRequest {
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: HeaderMap
    ///
    fn get(
        &self,
        url: Url,
        headers: HeaderMap,
        basic_auth: &Option<BasicAuth>,
    ) -> MailchimpResult<String> {
        let builder = match basic_auth {
            Some(auth) => CLIENT
                .get(url)
                .basic_auth(auth.username.clone(), Some(auth.api_token.clone())),
            None => CLIENT.get(url),
        };

        let result = builder.headers(headers).send();
        self.process_response(result, "GET")
    }
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: HeaderMap
    ///     payload: Datos a enviar a la URL especificada
    ///
    fn post<P>(
        &self,
        url: Url,
        headers: HeaderMap,
        payload: P,
        basic_auth: &Option<BasicAuth>,
    ) -> MailchimpResult<String>
    where
        P: Serialize,
    {
        let builder = match basic_auth {
            Some(auth) => CLIENT
                .post(url)
                .basic_auth(auth.username.clone(), Some(auth.api_token.clone())),
            None => CLIENT.post(url),
        };
        let result = builder.headers(headers).json(&payload).send();
        self.process_response(result, "POST")
    }
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: HeaderMap
    ///     payload: Datos a enviar a la URL especificada
    ///
    fn patch<P>(
        &self,
        url: Url,
        headers: HeaderMap,
        payload: P,
        basic_auth: &Option<BasicAuth>,
    ) -> MailchimpResult<String>
    where
        P: Serialize,
    {
        let builder = match basic_auth {
            Some(auth) => CLIENT
                .patch(url)
                .basic_auth(auth.username.clone(), Some(auth.api_token.clone())),
            None => CLIENT.post(url),
        };
        let result = builder.headers(headers).json(&payload).send();
        self.process_response(result, "PATCH")
    }
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: HeaderMap
    ///     payload: Datos a enviar a la URL especificada
    ///
    fn put<P>(
        &self,
        url: Url,
        headers: HeaderMap,
        payload: P,
        basic_auth: &Option<BasicAuth>,
    ) -> MailchimpResult<String>
    where
        P: Serialize,
    {
        let builder = match basic_auth {
            Some(auth) => CLIENT
                .patch(url)
                .basic_auth(auth.username.clone(), Some(auth.api_token.clone())),
            None => CLIENT.post(url),
        };
        let result = builder.headers(headers).json(&payload).send();
        self.process_response(result, "PUT")
    }
    ///
    ///  Argumentos:
    ///     url: Url
    ///     headers: HeaderMap
    ///
    fn delete(
        &self,
        url: Url,
        headers: HeaderMap,
        basic_auth: &Option<BasicAuth>,
    ) -> MailchimpResult<String> {
        let builder = match basic_auth {
            Some(auth) => CLIENT
                .delete(url)
                .basic_auth(auth.username.clone(), Some(auth.api_token.clone())),
            None => CLIENT.get(url),
        };

        let result = builder.headers(headers).send();
        self.process_response(result, "DELETE")
    }
}

impl MailchimpRequest {
    fn process_response<'a>(
        &self,
        response: Result<Response, Error>,
        method: &'a str,
    ) -> MailchimpResult<String> {
        match response {
            Ok(resp) => match resp.status() {
                StatusCode::OK => match resp.text() {
                    Ok(txt) => Ok(txt),
                    Err(e) => {
                        error!(target: "mailchimp", "{:?}: Response Error Details: {:?}", method, e);
                        Err(MailchimpErrorType::default())
                    }
                },
                StatusCode::NO_CONTENT => Ok("".to_string()),
                status => match resp.text() {
                    Ok(txt) => match serde_json::from_str(&txt) {
                        Ok(value) => Err(value),
                        Err(e) => {
                            error!(
                                target: "mailchimp",
                                "{:?}: Response Error details: {:?} status {:?}",  method, e, status);
                            Err(MailchimpErrorType::default())
                        }
                    },
                    Err(e) => {
                        error!(
                            target: "mailchimp",
                            "{:?}: Response Error details: {:?} status {:?}",  method, e, status);
                        Err(MailchimpErrorType::default())
                    }
                },
            },
            Err(e) => {
                error!(target: "mailchimp", "{:?} {:?}", method, e);
                Err(MailchimpErrorType::default())
            }
        }
    }
}
