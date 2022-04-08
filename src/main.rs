#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Attr, Cell, color, format};
use std::{io, fs};
use std::io::Write;
use std::collections::BTreeMap;
use chrono::Local;

const FILEPATH: &str = "/home/phydon/main/attendance/logfile.txt";

fn main() {
    let datetime = Local::now().to_string();

    let mut container: BTreeMap<_,_> = BTreeMap::new();
    let keys = vec!["LOG", "PF", "TEF"];
    
    println!("Enter \"Y\" / \"y\" for YES or \"N\" / \"n\" for NO");

    for key in keys {
        println!("{} attendant?", key);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        container.insert(key, input.to_uppercase());
    }

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

    write_to_file(&mut table).expect("Failed to write to file");

    table.printstd();

    loop {
        println!("Done?");
        println!("Press \"q\" to quit!");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(error) => eprintln!("Failed to read input. {}", error),
        }

        if input.contains(&String::from("q"))  {
            std::process::exit(0);
        }
    }

}

fn write_to_file(content: &mut Table) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILEPATH)?;

    writeln!(file, "{}", content)?;
    Ok(())
    }
