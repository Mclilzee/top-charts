use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use monthly_stat::MonthlyStat;

mod monthly_stat;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("top-stats")?;
    let mut buf = String::new();
    BufReader::new(file).read_to_string(&mut buf)?;

    let mut stats_map: HashMap<String, MonthlyStat> = HashMap::new();
    let stats = buf
        .split("==============")
        .map(MonthlyStat::parse)
        .collect::<Vec<MonthlyStat>>();

    let first_month = &stats.first().unwrap().month;
    let last_month = &stats.last().unwrap().month;

    stats.into_iter().for_each(|ns| {
        match stats_map.get_mut(&ns.month) {
            Some(s) => *s += ns,
            None => {
                stats_map.insert(ns.month.clone(), ns);
            }
        };
    });

    Ok(())
}
