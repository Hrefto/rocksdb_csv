use csv::ReaderBuilder;
use rocksdb::{DB, Options};
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let csv_file_path = "/Users/vrn21/Developer/hrefto/shorterdb/examples/write_from_csv/data.csv";

    let tempdir = tempfile::Builder::new()
        .prefix("_path_for_rocksdb_storage")
        .tempdir()
        .expect("Failed to create temporary path for the _path_for_rocksdb_storage");
    let path = tempdir.path();

    let db = DB::open_default(path).expect("Failed to open RocksDB");

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open(csv_file_path).expect("Failed to open CSV file"));

    for result in rdr.records() {
        let record = result.expect("Failed to read record");
        if record.len() == 2 {
            let key = record.get(0).unwrap();
            let value = record.get(1).unwrap();
            db.put(key.as_bytes(), value.as_bytes())
                .expect("Failed to write to RocksDB");
            println!("Inserted Key: {}, Value: {}", key, value);
        } else {
            eprintln!("Invalid record format: {:?}", record);
        }
    }

    println!(
        "All records from {} have been written to the RocksDB.",
        csv_file_path
    );

    println!("Entering REPL mode. Type 'exit' to quit.");
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let args: Vec<&str> = input.split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        match args[0] {
            "set" if args.len() == 3 => {
                let key = args[1];
                let value = args[2];
                db.put(key.as_bytes(), value.as_bytes())
                    .expect("Failed to set key-value pair");
                println!("Key: {}, Value: {} Set", key, value);
            }
            "get" if args.len() == 2 => {
                let key = args[1];
                let start_time = std::time::Instant::now();
                match db.get(key.as_bytes()) {
                    Ok(Some(value)) => {
                        println!(
                            "Value for key: {} found: {:?} (Time taken: {:?})",
                            key,
                            String::from_utf8(value).unwrap(),
                            start_time.elapsed()
                        );
                    }
                    Ok(None) => {
                        println!("Value for Key: {} Not found!!", key);
                    }
                    Err(_) => {
                        println!("Error retrieving value for Key: {}", key);
                    }
                }
            }
            "delete" if args.len() == 2 => {
                let key = args[1];
                db.delete(key.as_bytes()).expect("Failed to delete key");
                println!("Key: {} deleted", key);
            }
            _ => println!(
                "Unknown command. Use 'set <key> <value>', 'get <key>', or 'delete <key>'."
            ),
        }
    }

    let _ = DB::destroy(&Options::default(), path);
    println!("RocksDB destroyed. Exiting.");
}

