use keev::{new, store::Store, KeevDB};
use std::io;
fn main() -> anyhow::Result<()> {
    let mut db = new::<KeevDB>(keev::store::DBType::Persistant);
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let command: Vec<&str> = input.split_whitespace().collect();
                let operation = command[0];
                if command.len() == 2 {
                    match operation {
                        "get" => {
                            let result = db.get(command[1])?;
                            println!("{:?}", keev::deserialize(result));
                        }
                        "remove" => {
                            let result = db.remove(command[1])?;
                            println!("{:?}", keev::deserialize(&result));
                        }
                        _ => println!("Unsupported Operation"),
                    }
                } else if command.len() >= 3 {
                    match operation {
                        "set" => {
                            let result = db.set(command[1], &command[2..].join(" "))?;
                            println!("{:?}", keev::deserialize(&result));
                        }
                        _ => println!("Unsupported Operation"),
                    }
                } else {
                    break;
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
    Ok(())
}
