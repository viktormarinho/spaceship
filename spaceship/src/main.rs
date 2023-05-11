use rand::{Rng, distributions::Alphanumeric};
use std::io::Write;


pub struct StoredData<T> {
    pub id: String,
    pub data: T
}

#[derive(Debug)]
pub enum StoreErr {
    UnknownError
}

#[derive(Debug)]
pub enum LoadErr {
    UnknownError
}

pub trait Storeable
where Self: Sized {
    fn store(&self) -> Result<StoredData<Self>, StoreErr>;
}

pub trait Loadable
where Self: Sized {
    fn load(id: String) -> Result<Option<Self>, LoadErr>;
}

struct Pessoa {
    nome: String
}

impl Pessoa {
    fn new(nome: String) -> Pessoa {
        Pessoa { nome }
    }
}

impl Storeable for Pessoa {
    fn store(&self) -> Result<StoredData<Pessoa>, StoreErr> {
        Err(StoreErr::UnknownError)
    }
}

impl Storeable for String {
    fn store(&self) -> Result<StoredData<Self>, StoreErr> {
        let id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect(); 

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("database")
            .unwrap();

        if let Err(e) = writeln!(file, "{}", format!("{id}%%%{self}").to_string()) {
            println!("Error occurred storing the string {self}");
            dbg!(e);
            return Err(StoreErr::UnknownError);
        }

        Ok(StoredData { id, data: self.to_string() })
    }
}

impl Loadable for String {
    fn load(id: String) -> Result<Option<Self>, LoadErr> {
        let data = match std::fs::read_to_string("database") {
            Err(_) => {
                println!("Error occurred loading the string with id {id}");
                return Err(LoadErr::UnknownError);
            },
            Ok(data) => data
        }; 

        let data = data.lines().find_map(|row| {
            if id == row.split("%%%").nth(0).unwrap() {
                return Some(row.split("%%%").nth(1).unwrap().to_string());
            }
            None
        });

        Ok(data)
    }
}

fn main() {
    // let nome_legal = "Baltazar".to_string();

    let registro = String::load("j02Knb5".to_string());

    if let Ok(registro) = registro {
        if let Some(string) = registro {
            println!("Achei a string: {}", string);
        } else {
            println!("Não achei este registro!");
        }
    }

    // let registro = match nome_legal.store() {
    //     Ok(data) => data,
    //     Err(err) => {
    //         println!("ACONTECEU UM ERRO:");
    //         dbg!(err);
    //         std::process::abort();
    //     },
    // };

    // println!("Armazenei a string! o id é {}", registro.id);
}