use serde::{Serialize, Deserialize};
use std::fs::{OpenOptions, read_to_string};
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Lift {
    lift_type: String,
    weight: f32,
    sets: u32,
    reps: u32,
    date: String,
}
fn add_new_lift() -> Lift {
    let mut lift_type = String::new();
    let mut weight = String::new();
    let mut sets = String::new();
    let mut reps = String::new();
    let mut date = String::new();

    println!("Enter lift type:");
    io::stdin().read_line(&mut lift_type).expect("Failed to read line");

    println!("Enter weight:");
    io::stdin().read_line(&mut weight).expect("Failed to read line");

    println!("Enter sets:");
    io::stdin().read_line(&mut sets).expect("Failed to read line");

    println!("Enter reps:");
    io::stdin().read_line(&mut reps).expect("Failed to read line");

    println!("Enter date (YYYY-MM-DD):");
    io::stdin().read_line(&mut date).expect("Failed to read line");

    Lift {
        lift_type: lift_type.trim().to_string(),
        weight: weight.trim().parse().expect("Please type a number!"),
        sets: sets.trim().parse().expect("Please type a number!"),
        reps: reps.trim().parse().expect("Please type a number!"),
        date: date.trim().to_string(),
    }
}
fn save_lift_to_file(lift: &Lift, file_path: &Path) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)?;

    let lift_json = serde_json::to_string(&lift).expect("Error serializing lift data");
    writeln!(file, "{}", lift_json)?;

    Ok(())
}

fn main() {
    let lift = add_new_lift();
    let file_path = Path::new("lifts.json");

    match save_lift_to_file(&lift, file_path) {
        Ok(_) => println!("Lift saved successfully!"),
        Err(e) => eprintln!("Failed to save lift: {}", e),
    }
}
