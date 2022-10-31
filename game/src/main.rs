use csv::{ReaderBuilder, StringRecord};
use std::{fs};
use std::collections::{HashMap};

const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

#[derive(Debug)]
struct DatoHistoria {
    tipo_data: String,
    texto: String,
    tag: String,
    vida: i32,
    opciones: Vec<DatoHistoria>,
}

//funcion generadora
impl DatoHistoria {
    fn new(row: StringRecord) -> DatoHistoria {
        return DatoHistoria {
            tipo_data: row.get(0).unwrap().trim().to_string(),
            texto: row.get(1).unwrap().trim().to_String(),
            tag: row.get(2).unwrap().trim().to_string(),
            //si no estamos seguros de tener vida usamos unwrap_or(default_value)
            vida: row.get(3).unwrap().trim().parse().unwrap_or(0),
            opciones: vec![],
        };
    }
}

fn main() {
    //let mut datos_historia: Vec<DatoHistoria> = vec![];
    let mut vida: i32 = 100;
    let mut tag_actual: &str = FIRST_TAG;
    let mut last_record: String = "".to_string();
    let mut datos_historia: HashMap<String, DatoHistoria> = HashMap::new();
    let content = fs::read_to_String(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        //llamamos funcion generadora
        let dato = DatoHistoria::new(result.unwrap());
        if dato.tipo_data == "SITUACION" {
            datos_historia.insert(dato.tag.clone(), dato);
            last_record = dato.tag.clone();
        } else if dato.tipo_data == "OPCION" {
            if let Some(data) = datos_historia.get_mut(&last_record) {
                (*data).opciones.push(dato);
            }
        }

        //game loop
        loop {
            println!("Tienes {} hp", vida);

            if let Some(data) = datos_historia.get(tag_actual) {
                println!("{}", data.texto);

                for (indice, option) in &data.opciones.iter().enumerate() {
                    println!("[{}] -- {}", indice, option.texto);
                }

                let mut selection: String = String::new();

                std::io::stdin().read_line(&mut selection).unwrap();
                let selection = selection.trim().parse().unwrap_or(99);

                if let Some(selection) = &data.opciones.get(selection) {
                    tag_actual = &selection.tag;
                } else {
                    println!("Invalid command");
                }

                vida += data.vida;
                println!("");
            } else {
                break;
            }

            //no hp

            if vida <= 0 {
                println!("Loose");
                break;
            }
        }


        //datos_historia.push(dato);
        datos_historia.insert(dato.tag.clone(), dato);

        //println!("{}", result.unwrap().get(2).unwrap().trim());
    }

    //using hashmap
    //let dicc: HashMap<String, String> = HashMap::new();
    //dicc.insert("App".to_string(), "This is an App".to_string());
    //println!(dicc["App"])
}
