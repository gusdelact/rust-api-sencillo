extern crate rustless;
extern crate iron;
extern crate url;
extern crate serde;
extern crate serde_json;
extern crate valico;
extern crate unicase;

//libreria de adaptadores
extern crate adaptadores;

use valico::json_dsl;
use rustless::batteries::swagger;
use rustless::{Nesting};
use iron::headers::{ AccessControlAllowOrigin };
use iron::headers::{ AccessControlAllowMethods };
use iron::headers::{ AccessControlAllowHeaders };
use iron::headers::{ AccessControlExposeHeaders };
use unicase::UniCase;
use iron::prelude::*;
use iron::middleware::AfterMiddleware;
use  iron::method::Method;

struct CorsMiddleware;

impl AfterMiddleware for CorsMiddleware {
    fn after(&self, req: &mut Request, mut res: Response) -> 
          IronResult<Response> {
        res.headers.set(AccessControlAllowOrigin::Any);
        res.headers.set(AccessControlAllowMethods(
               vec![
                    Method::Options,
                    Method::Get, 
                    Method::Post, 
                    Method::Put,
                    Method::Delete,
                    Method::Head,
                    Method::Trace,
                    Method::Connect,
           ]));
        res.headers.set( AccessControlAllowHeaders(
               vec![
                UniCase("Origin".to_owned()),
                UniCase("Content-Type".to_owned()),
                UniCase("X-Amz-Date".to_owned()),
                UniCase("Authorization".to_owned()),
                UniCase("X-Api-Key".to_owned()),
                UniCase("X-Amz-Security-Token".to_owned()),
                UniCase("accept".to_owned()),
                UniCase("X-Auth-Token".to_owned()),
                UniCase("Access-Control-Allow-Origin".to_owned()),
                UniCase("Access-Control-Allow-Headers".to_owned()),
                UniCase("Access-Control-Allow-Methods".to_owned()),
                UniCase("Access-Control-Request-Headers".to_owned()),
                UniCase("Access-Control-Request-Method".to_owned()),
                UniCase("etag".to_owned()),
                UniCase("content-length".to_owned()),
                UniCase("X-PINGOTHER".to_owned()),
           ]));

        res.headers.set( AccessControlAllowHeaders(
               vec![
                UniCase("Origin".to_owned()),
                UniCase("Content-Type".to_owned()),
                UniCase("X-Amz-Date".to_owned()),
                UniCase("Authorization".to_owned()),
                UniCase("X-Api-Key".to_owned()),
                UniCase("X-Amz-Security-Token".to_owned()),
                UniCase("accept".to_owned()),
                UniCase("X-Auth-Token".to_owned()),
                UniCase("Access-Control-Allow-Origin".to_owned()),
                UniCase("Access-Control-Allow-Headers".to_owned()),
                UniCase("Access-Control-Allow-Methods".to_owned()),
                UniCase("Access-Control-Request-Headers".to_owned()),
                UniCase("Access-Control-Request-Method".to_owned()),
                UniCase("etag".to_owned()),
                UniCase("content-length".to_owned()),
                UniCase("X-PINGOTHER".to_owned()),
           ]));
        Ok(res)
    }
}


fn main() {

    let mut app = rustless::Application::new(rustless::Api::build(|api| {
//El espacio de nombres es /api/v1
        api.prefix("api");
        api.version("v1", rustless::Versioning::Path);

        api.mount(swagger::create_api("api-docs"));
        println!("Montado swagger");

//un friki envia un saludo
//se invoca con POST y el URI /api/v1/friki/:nombre
        api.post("friki/:nombre", |endpoint| {
//documentacion para Swagger
            endpoint.summary("Enviar un saludo");
            endpoint.desc("Utilizado para que un friki envie un mensaje");
//indicar los parametros a recibir y sus tipos de datos
            endpoint.params(|params| {
                params.req_typed("nombre", json_dsl::string());
                params.req_typed("saludo", json_dsl::string());
            });
//manejo de la peticion
            endpoint.handle(|client, params| {
                println!("POST friki {}",params);
//leer los parametros
                let saludo =  params.find("saludo").unwrap().to_string();
                let nombre =   params.find("nombre").unwrap().to_string();
                let url =
    "https://j8dkjo4657.execute-api.us-east-1.amazonaws.com/prod/entries";
//invocar al servicio de guardado de datos
               let fecha=adaptadores::write_entry(url.to_string(), 
                   adaptadores::Entry {
                  date: 0,
                  user: nombre,
                  greeting: saludo,
                 } );
               //escribir la respuesta 
               client.text( format!("{}", fecha) )
            })
        });//api.post

//obtener la lista de todos los frikis y sus comentarios
//se invoca como /api/v1/frikis
        api.get("frikis", |endpoint| {
//documentacion para Swagger
            endpoint.summary("Lista de todos los mensajes");
            endpoint.desc("Utilizar para lista todos los mensajes de todos los frikis");
//manejar la peticion
            endpoint.handle(|client, params| {
                println!("GET frikis {}",params);
                let url =
         "https://j8dkjo4657.execute-api.us-east-1.amazonaws.com/prod/entries";
//invocar al servicio de consulta de datos
                let entradas:Vec<adaptadores::Entry>=
                 adaptadores::read_entries(url.to_string());
                println!("{:?}",entradas[0].greeting);
                println!("{:?}",entradas[0].date);
                let json_entradas = serde_json::to_string(&entradas).unwrap();
                client.text(json_entradas)
            })//handle
        });//api.get

    }));

//documentacion del API en formato Swagger
    swagger::enable(&mut app, swagger::Spec {
        info: swagger::Info {
            title: "Ejempo de API Gateway en Rust".to_string(),
            description: Some("Demostracion de un API Gateway que interactua con AWS y DynamoDB".to_string()),
            contact: Some(swagger::Contact {
                name: "Gustavo De la Cruz Tovar".to_string(),
                url: Some("https://www.linkedin.com/in/gusdelact/".to_string()),
                ..std::default::Default::default()
            }),
            license: Some(swagger::License {
                name: "MIT".to_string(),
                url: "http://opensource.org/licenses/MIT".to_string()
            }),
            ..std::default::Default::default()
        },
        ..std::default::Default::default()
    });
    let mut chain = iron::Chain::new(app);
    chain.link_after(CorsMiddleware);
//ejecutar el servidor Web con Iron
    iron::Iron::new(chain).http("127.0.0.1:4000").unwrap();
}
