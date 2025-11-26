use std::error::Error;
use std::fs::File;
use std::io::Read;
use csv::Reader;
fn main() -> Result <(), Box<dyn Error>> {
    let mut file = File::open("data/heart_attack_prediction_dataset.csv")?;

    let mut reader = Reader::from_reader(file);
    for result in reader.records(){
        let record = result?;
        println!{"{:?}", record};
    }
    Ok(())
}
