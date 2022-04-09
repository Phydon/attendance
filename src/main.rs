#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Attr, Cell, color, format};
use chrono::Local;

use std::process::Command;
use std::time::Duration;
use std::{io, fs, thread};
use std::io::Write;
use std::collections::BTreeMap;

const FILEPATH: &str = "/home/phydon/main/attendance/logfile.txt";

fn main() {
    let datetime = Local::now().to_string();
    let container = check_attendance();
    let table = create_table(datetime, container);

    write_to_file(&table).expect("Failed to write to file");

    exec_clear().expect("Failed to clear screen");
    table.printstd();

    are_u_done();
}

fn exec_clear() -> io::Result<()> {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "clear"]).status()?;
        Ok(())
    } else {
        Command::new("clear").status()?;
        Ok(())
    }
}

fn check_attendance() -> BTreeMap<String, String> {
    let mut container: BTreeMap<_,_> = BTreeMap::new();
    let keys: Vec<String> = vec!["LOG".to_string(), "PF".to_string(), "TEF".to_string()];
    
    for key in keys {
        loop {
            exec_clear().expect("Failed to clear screen");
            println!("Enter \"Y\" for YES or \"N\" for NO\n");
            println!("{} attendant?", key);

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let input = input.strip_suffix("\n").unwrap().to_uppercase();

            let yes: String = "Y".to_string();
            let no: String = "N".to_string();

            if input.eq(&yes) {
                container.insert(key, yes);
                break;
            } else if input.eq(&no) {
                container.insert(key, no);
                break;
            } else {
                println!("Not valid: {}", input);
                thread::sleep(Duration::from_millis(1500));
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

fn are_u_done() {
    loop {
        println!("Done?");
        println!("Press \"q\" to quit!");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        exec_clear().expect("Failed to clear screen");
        
        let input = input.strip_suffix("\n").unwrap();
        let quit: String = "q".to_string();

        if input.eq(&quit)  {
            std::process::exit(0);
        }
    }
}
