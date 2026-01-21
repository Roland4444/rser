use serde::{Serialize, Deserialize};
use bincode;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Employee {
    id: u32,
    name: String,
    salary: f64,
    is_active: bool,
}

impl Employee {
    fn new(id: u32, name: String, salary: f64, is_active: bool) -> Self {
        Employee { id, name, salary, is_active }
    }

    fn serialize_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        // В bincode 2.0 используется конфигурация
        let encoded: Vec<u8> = bincode::serialize(self)?;
        let mut file = File::create(filename)?;
        file.write_all(&encoded)?;
        Ok(())
    }

    fn deserialize_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let decoded: Employee = bincode::deserialize(&buffer)?;
        Ok(decoded)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let emp = Employee::new(1, "John Doe".to_string(), 50000.50, true);

    emp.serialize_to_file("employee.bin")?;
    println!("Data serialized to employee.bin");

    let emp2 = Employee::deserialize_from_file("employee.bin")?;
    println!("Deserialized data: {:?}", emp2);

    assert_eq!(emp, emp2);
    println!("Hello, world!");
    Ok(())
}