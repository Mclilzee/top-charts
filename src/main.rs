use std::{
    fs::File,
    io::{BufReader, Read},
};

use monthly_stat::MonthlyStat;

mod monthly_stat;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("top-stats")?;
    let mut buf = String::new();
    BufReader::new(file).read_to_string(&mut buf)?;
    let month = buf
        .split("==============")
        .map(MonthlyStat::parse)
        .nth(1)
        .unwrap();

    println!("{month:?}");
    Ok(())
}
