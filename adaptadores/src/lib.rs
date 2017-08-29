extern crate curl;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use curl::easy::Easy;
use curl::easy::List;
use serde_json::{Value};
#[derive(Serialize, Deserialize,Debug)]
pub struct Entry {
  pub user: String,
  pub greeting: String, 
  pub date: u64,
}

pub fn read_entries(url:String) -> Vec<Entry> {
     //buffer para almacenar resultado de la consulta
     let mut data = Vec::new();
     //para manejar la conexion via cURL
     let mut handle = Easy::new();
     //fijar el URL del servicio    
     handle.url(&url).unwrap();
     handle.verbose(true).unwrap();
     {
      //realizar la conexion con el servicio, via la estructura Transfer
      let mut transfer = handle.transfer();
      transfer.write_function(|new_data| {
             data.extend_from_slice(new_data);
             Ok(new_data.len())
       }).unwrap();
       transfer.perform().unwrap();
     }//bloque local
     println!("{:?}", data);
     //convertir el buffer de bytes a cadena de caracter
     let body= String::from_utf8(data).expect("Found invalid UTF-8");
     println!("{}", body);
    
     let json:Value; //deserializar de string a JSON
     let mut entries:Vec<Entry>=Vec::new();
     //tomar de la estructura JSON a la estructura Entry
     match serde_json::from_str(&body) {
         Ok(v) => {
               json=v;  
               let mut indice=0;
               //tratar los Items como arreglo
               let items=json["Items"].as_array().unwrap(); 
/******
               println!("items:{:?}",items);
               println!("items.len:{:?}",items.len());
               println!("items[0]:{:?}",items[indice]);
               println!("items[0]:{:?}",items[indice].get("date").unwrap());
               println!("items[0]:{:?}",items[indice].get("greeting").unwrap());
               println!("items[0]:{:?}",items[indice].get("user").unwrap());
****/
               while indice < items.len() {
                 let u= items[indice].get("user")
                                  .unwrap().to_string();
                 let g= items[indice].get("greeting")
                                  .unwrap().to_string();
                 let d:u64= items[indice].get("date").unwrap().as_u64()
                                  .unwrap();
/******
                 println!("user {} ",u); 
                 println!("greeting {} ",g); 
                 println!("date {} ",d);
****/
                 entries.push(
                  Entry {
                    date : d,
                    user :  u,
                    greeting  :  g,
                  }
                 );
                 indice=indice+1;
               }//while
            },
        Err(_) => panic!("json con formato incorrecto"),
      } //match
          entries 
    }//read_entries
    
pub fn write_entry(url:String, entry: Entry) -> u64 {
     //referncia para manejar cURL
     let mut handle = Easy::new();
//serializar a JSON     
     let json = serde_json::to_string(&entry).unwrap();
//     println!("{}",json);
//configurar los HEADERS de HTTP    
     let mut list = List::new();
     list.append("content-type: application/json").unwrap();
     handle.http_headers(list).unwrap();
//indicar el url
     handle.url(&url).unwrap();
//indicar que se va a realizar un POST
     handle.post(true).unwrap();
//indicar los datos a transmitir, formato de bytes
     handle.post_fields_copy(json.as_bytes());
//buffer para recibir la respuesta
     let mut data = Vec::new();
     {
//realizar la invocacion al servicio a través del URL
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
               data.extend_from_slice(new_data);
               Ok(new_data.len())
         }).unwrap();
        transfer.perform().unwrap();
     }
//     println!("{:?}", data);
     let body= String::from_utf8(data).expect("Found invalid UTF-8");
//     println!("{}", body);
     let json:Value;
     let mut resultado: u64=0;
     match serde_json::from_str(&body) {
         Ok(v) => {
               json=v;
               println!("date: {}", json["date"]);
               println!("date: {}", json["date"].is_u64());
               resultado= json["date"].as_u64().unwrap();
    
         },
         Err(_) => panic!("json con formato incorrecto"),
     }
     resultado
} //write_entries
//pruebas unitarias 
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_entries() {
       let url =
         "https://j8dkjo4657.execute-api.us-east-1.amazonaws.com/prod/entries";
        let entr:Vec<Entry>=read_entries(url.to_string());
        println!("{:?}",entr[0].greeting);
        println!("{:?}",entr[0].date);
    }
    #[test]
    fn test_write_entries() {
       let url =
         "https://j8dkjo4657.execute-api.us-east-1.amazonaws.com/prod/entries";
       let  entry =  Entry {
         date: 0,
         user: "Vegetta".to_string(),
         greeting: "Sayayin!".to_string(),
        };
       let fecha=write_entry(url.to_string(), entry);
       println!("Insertado con llave {}",fecha);
    }
}
