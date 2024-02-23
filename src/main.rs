use std::{io::stdin, thread};

use rand::Rng;

struct Row {
    data: Vec<u8>,
}

impl Row {
    fn new(data: Vec<u8>) -> Self { Self { data } }
}

struct Session {
    rows: Vec<Row>,
}

impl Session {
    fn new() -> Self { 
        let mut rows: Vec<Row> = Vec::with_capacity(1000);
        for _ in 0..1000 {
            rows.push(Row::new(vec![0;100000]));
        }
        Self { rows } 
    }

    fn append(&mut self, byte: u8) {
        let x: usize = rand::thread_rng().gen_range(0..1000);
        let row = &mut self.rows[x];
        let position: usize = rand::thread_rng().gen_range(0..1000);

        let mut new_data = Vec::with_capacity(1001);
        let data = &mut row.data;

        new_data.append(&mut data[..position].to_vec());
        new_data.push(byte);
        new_data.append(&mut data[position..].to_vec());

        row.data = new_data;
    }
    
    fn remove(&mut self) {
        let x: usize = rand::thread_rng().gen_range(0..1000);
        let row = &mut self.rows[x];
        let position: usize = rand::thread_rng().gen_range(0..row.data.len()-1);

        let data = &mut row.data;

        let mut new_data = Vec::with_capacity(1000);

        new_data.append(&mut data[..position].to_vec());
        new_data.append(&mut data[position+1..].to_vec());
        data.clear(); 

        row.data = new_data;    
        row.data.shrink_to_fit();
    }

    fn clear(&mut self) {
        for row in &mut self.rows {
            row.data.clear();
            row.data.shrink_to_fit();
        }
    }
}

fn main() {
    let mut val = rand::thread_rng();
    let mut buf = "".to_owned();
    {
    stdin().read_line(&mut buf).unwrap();
    let mut session = Session::new();
    println!("Initialized 100k bytes 1000 times.");
    stdin().read_line(&mut buf).unwrap();

    for _ in 0..10000 {
        session.append(val.gen());
        session.remove();
    }

    println!("Added and removed 10k bytes.");
    stdin().read_line(&mut buf).unwrap();
    session.clear();
    println!("Cleared session.");
    stdin().read_line(&mut buf).unwrap();
    }
    println!("Dropp session.");
    stdin().read_line(&mut buf).unwrap();
    println!("Essa xd");
}
