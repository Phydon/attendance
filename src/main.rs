#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Attr, Cell, color, format};
use std::{io, fs};
use std::io::Write;
// use std::collections::BTreeMap;
use chrono::Local;

const FILEPATH: &str = "/home/phydon/main/attendance/logfile.txt";

fn main() {
    println!("Input: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("Your input was: {}", input),
        Err(error) => eprintln!("Failed to read input. {}", error),
    }

    // let mut container: BTreeMap<_,_> = BTreeMap::new();
    let datetime = Local::now().to_string();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![
            Cell::new(&datetime)
                .with_style(Attr::ForegroundColor(color::RED))
                .with_hspan(3)]));
    table.add_row(row![FdBwbc => "Index", "Attendance", "Yes / No"]);
    table.add_row(row!["1", Fb->"PF", c->"[ ]"]);
    table.add_row(row!["2", Fb->"LOG", c->"[ ]"]);

    write_to_file(&mut table).expect("Failed to write to file");

    table.printstd();
}

fn write_to_file(content: &mut Table) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILEPATH)?;

    writeln!(file, "{}", content)?;
    Ok(())
    }
