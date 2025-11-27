use std::error::Error;
use std::fs::File;
use serde::{Deserialize, Serialize};
use serde::de::{Deserializer, Error as DeError};
use csv::{Reader, ReaderBuilder};

// ==== UNIVERSAL BOOL PARSER for "0"/"1"/"true"/"false" ====
fn from_str_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let v: String = String::deserialize(deserializer)?;
    let v = v.trim().to_lowercase();

    match v.as_str() {
        "1" => Ok(true),
        "0" => Ok(false),
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(DeError::custom(format!("invalid bool: {}", other))),
    }
}

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Patient ID")]
    patient_id: String,

    #[serde(rename = "Age")]
    age: u32,

    #[serde(rename = "Sex")]
    sex: String,

    #[serde(rename = "Cholesterol")]
    cholesterol: u32,

    #[serde(rename = "Blood Pressure")]
    blood_pressure: String,

    #[serde(rename = "Heart Rate")]
    heart_rate: u32,

    #[serde(rename = "Diabetes", deserialize_with = "from_str_to_bool")]
    diabetes: bool,

    #[serde(rename = "Family History", deserialize_with = "from_str_to_bool")]
    family_history: bool,

    #[serde(rename = "Smoking", deserialize_with = "from_str_to_bool")]
    smoking: bool,

    #[serde(rename = "Obesity", deserialize_with = "from_str_to_bool")]
    obesity: bool,

    #[serde(rename = "Alcohol Consumption", deserialize_with = "from_str_to_bool")]
    alcohol_consumption: bool,

    #[serde(rename = "Exercise Hours Per Week")]
    exercise_hours_per_week: f32,

    #[serde(rename = "Diet")]
    diet: String,

    #[serde(rename = "Previous Heart Problems", deserialize_with = "from_str_to_bool")]
    previous_heart_problems: bool,

    #[serde(rename = "Medication Use", deserialize_with = "from_str_to_bool")]
    medication_use: bool,

    #[serde(rename = "Stress Level")]
    stress_level: u32,

    #[serde(rename = "Sedentary Hours Per Day")]
    sedentary_hours_per_day: f32,

    #[serde(rename = "Income")]
    income: u32,

    #[serde(rename = "BMI")]
    bmi: f32,

    #[serde(rename = "Triglycerides")]
    triglycerides: u32,

    #[serde(rename = "Physical Activity Days Per Week")]
    physical_activity_days_per_week: u32,

    #[serde(rename = "Sleep Hours Per Day")]
    sleep_hours_per_day: u32,

    #[serde(rename = "Country")]
    country: String,

    #[serde(rename = "Continent")]
    continent: String,

    #[serde(rename = "Hemisphere")]
    hemisphere: String,

    #[serde(rename = "Heart Attack Risk", deserialize_with = "from_str_to_bool")]
    heart_attack_risk: bool,
}


fn main() -> Result <(), Box<dyn Error>> {
    println!("--- Work with all records ---");

    let mut file = File::open("data/heart_attack_prediction_dataset.csv")?;
    let mut reader = Reader::from_reader(file);

    for result in reader.records(){
        let record = result?;
        println!{"{:?}", record};
    }

    println!("--- Work with headers ---");

    // Перечитываем файл, иначе reader.records() уже исчерпан
    let mut file = File::open("data/heart_attack_prediction_dataset.csv")?;

    let headers = reader.headers()?.clone();
    println!("Headers: {:?}", headers);

    let idx_heart_rate = headers
        .iter()
        .position(|h| h == "Heart Rate")
        .expect("No 'heart_rate' column");

    for result in reader.records() {
        let record = result?;
        let heart_rate: u32 = record[idx_heart_rate].parse()?;
        println!("Heart Rate: {}", heart_rate);
    }

    println!("--- Storing Records in Data Structure ---");

    let file = File::open("data/heart_attack_prediction_dataset.csv")?;
    let mut reader = ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(file);
    let mut records: Vec<Record> = Vec::new();

    for result in reader.deserialize() {
        match result {
            Ok(record) => records.push(record),
            Err(err) => {
                println!("❌ Parse error: {}", err);
                continue; // skip bad rows
            }
        }
    }

    println!("✔ Parsed {} records", records.len());

    // Example: print first 1
    for r in records.iter().take(1) {
        println!("{:#?}", r);
    }

    Ok(())
}
