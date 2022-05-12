use std::{fs::File, io::Write};

pub mod bfs;
pub mod dfs;
pub mod astar;

fn write_result(filename: &str, res: &String) -> std::io::Result<()> {
    let mut buffer = File::create(filename)?;
    let mut s = String::new();

    if res == "-1" {
        s.push_str(&format!("{}\n", res));
    } else {
        s.push_str(&format!("{}\n", res.len()));
        s.push_str(&format!("{}", res));
    }

    buffer.write_all(s.as_bytes())?;

    Ok(())
}

fn write_stats(
    filename: &str,
    res: String,
    visited: usize,
    processed: usize,
    max_depth: usize,
    time: u128,
) -> std::io::Result<()> {
    let mut buffer = File::create(filename)?;
    let mut s = String::new();
    let t = (time as f64) / (1000 as f64);

    if res == "-1" {
        s.push_str(&format!("{}\n", res));
    } else {
        s.push_str(&format!("{}\n", res.len()));
    }
    s.push_str(&format!("{}\n", visited));
    s.push_str(&format!("{}\n", processed));
    s.push_str(&format!("{}\n", max_depth));
    s.push_str(&format!("{}", t));
    buffer.write_all(s.as_bytes())?;

    Ok(())
}
