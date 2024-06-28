use csv;
use std::error::Error;

// use std::fs::File;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    //Handling the errors explicitly:

    // let file = File::open(path);
    // let file = match file {
    //     Ok(file) => file,
    //     Err(e) => return Err(Box::new(e))
    // };

    // let mut reader = csv::Reader::from_reader(file);
    // let records = reader.records();

    // for result in records {
    //     let record = match result {
    //         Ok(record) => record,
    //         Err(e) => return Err(Box::new(e)),
    //     };
    // }

    // println!("{:?}", record);

    // Handling errors implicitly using ?:

    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;

        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("./customers.csv") {
        eprintln!("{}", e);
    }
}
