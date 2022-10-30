use csv::{ReaderBuilder, StringRecord};
use str::{fs};

const FILENAME: &str = "history.csv";

struct DatoHistoria {
    tipo_data: String,
    texto: String,
    tag: String,
    vida: i32,
}

fn main() {
    let content = fs::read_to_String(FILENAME).unwrap();

    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        println!("{}", result.unwrap().get(2).unwrap().trim());
    }

    let dato = DatoHistoria {
        tipo_data: "Situacion".to_string(),
        texto: "text".to_String(),
        tag: "INICIO".to_string(),
        vida: 0,
    }
}
