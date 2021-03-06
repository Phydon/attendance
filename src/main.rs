#[macro_use] 
extern crate prettytable;

use prettytable::{Table, Row, Attr, Cell, color, format};
use chrono::Local;
use indicatif::{ProgressBar, ProgressStyle};

use std::cmp::min;
use std::process::Command;
use std::time::Duration;
use std::{io, fs, thread};
use std::io::Write;
use std::collections::BTreeMap;

const FILEPATH: &str = "./logfile.txt";

fn main() {
    loop {
        let datetime = Local::now().to_string();
        let container = check_attendance();
        let table = create_table(datetime, container);


        let done = are_u_done(&table);
        if done {
            write_to_file(&table).expect("Failed to write to file");
            break;
        }
    }

    exec_clear().expect("Failed to clear screen");
    std::process::exit(0);
}

fn exec_clear() -> io::Result<()> {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status()?;
        Ok(())
    } else {
        Command::new("clear").status()?;
        Ok(())
    }
}

fn sleep(num: u64) {
    thread::sleep(Duration::from_millis(num));
}

fn progress_bar() {
    println!("\n");

    let mut idx = 0;
    let end = 750;

    let pb = ProgressBar::new(end);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{wide_bar:.cyan/blue}] ({eta})")
        .progress_chars("#>-"));

    while idx < end {
        let new = min(idx + 10, end);
        idx = new;
        pb.set_position(new);
        sleep(15);
    }

    pb.finish_with_message("done");
}

fn check_attendance() -> BTreeMap<String, String> {
    let mut container: BTreeMap<_,_> = BTreeMap::new();
    let keys: Vec<String> = vec![
        "A".to_string(), 
        "B".to_string(), 
        "C".to_string(),
        "D".to_string(),
        "E".to_string(),
        "F".to_string(),
        "G".to_string(),
        "H".to_string(),
        "I".to_string()
    ];
    
    for key in keys {
        loop {
            exec_clear().expect("Failed to clear screen");

            println!("Enter \"Y\" for YES or \"N\" for NO\n");
            println!("{} attendant?", key);

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");

            match input.trim() {
                "y" | "Y" | "n" | "N" => {
                    container.insert(key, input.to_uppercase());
                    break;
                },
                _ => {
                    println!("Not valid");
                    progress_bar();
                },
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

fn write_to_file(content: &Table) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILEPATH)?;

    writeln!(file, "{}", content)?;
    Ok(())
    }

fn are_u_done(table: &Table) -> bool {
    loop {
        exec_clear().expect("Failed to clear screen");

        table.printstd();

        println!("Done?");
        println!("Press \"Y\" to quit or \"N\" to make changes!");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().to_uppercase().as_str() {
            "y" | "Y" => return true,
            "n" | "N" => return false,
            _ => {
                println!("Not valid");
                progress_bar();
            },
        }
    }
}
