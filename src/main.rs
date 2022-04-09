#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Attr, Cell, color, format};
use chrono::Local;

use std::{io, fs};
use std::io::Write;
use std::collections::BTreeMap;

const FILEPATH: &str = "/home/phydon/main/attendance/logfile.txt";

fn main() {
    let datetime = Local::now().to_string();
    let container = check_attendance();
    let table = create_table(datetime, container);

    write_to_file(&table).expect("Failed to write to file");

    table.printstd();

    loop {
        println!("Done?");
        println!("Press \"q\" to quit!");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(error) => eprintln!("Failed to read input. {}", error),
        }
        
        let input = input.strip_suffix("\n").unwrap();

        if input.len() == 1 && input.contains(&String::from("q"))  {
            std::process::exit(0);
        }
    }
}

fn write_to_file(content: &Table) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILEPATH)?;

    writeln!(file, "{}", content)?;
    Ok(())
    }

fn check_attendance() -> BTreeMap<String, String> {
    let mut container: BTreeMap<_,_> = BTreeMap::new();
    let keys: Vec<String> = vec!["LOG".to_string(), "PF".to_string(), "TEF".to_string()];
    
    println!("Enter \"Y\" for YES or \"N\" for NO\n");

    for key in keys {
        loop {
            println!("{} attendant?", key);

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let input = input.strip_suffix("\n").unwrap().to_uppercase();

            if input.len() == 1 && input.contains(&String::from("Y")) {
                container.insert(key, "Y".to_string());
                break;
            } else if input.len() == 1 && input.contains(&String::from("N")) {
                container.insert(key, "N".to_string());
                break;
            } else {
                println!("Not valid: {}", input);
                println!("Enter \"Y\" for YES or \"N\" for NO\n");
            }
        }
    }

    container
}

fn create_table(datetime: String , container: BTreeMap<String, String>) -> Table {
    let mut idx: i32 = 0;
    let mut table = Table::new();

    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![
            Cell::new(&datetime)
                .with_style(Attr::ForegroundColor(color::RED))
                .with_hspan(3)]));
    table.add_row(row![FdBwbl->"Index", FdBwbc->"Attendance", FdBwbc->"Yes / No"]);

    for (key, value) in &container {
        table.add_row(row![idx, Fb->key, c->value]);
        idx += 1;
    }

    table
}
