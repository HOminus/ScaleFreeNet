use std::collections::HashMap;
use std::fs;
use std::io::Error;

use plot_help::{plot_to_console, plot_to_console_log, plot_to_console_log_log};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "arguments")]
struct Args {
    #[structopt(short, long)]
    path: String,
}

impl Args {
    fn output_path(&self) -> &'static str {
        "beer.txt"
    }
}

fn main() -> Result<(), Error> {
    let args: Args = Args::from_args();
    let file_data = fs::read(args.path.as_str())?;
    let file_data = String::from_utf8(file_data).unwrap();

    let mut unique_beers = HashMap::new();
    for line in file_data
        .lines()
        .filter(|line| line.starts_with("beer/beerId:"))
    {
        if let Some(index) = line.find(' ') {
            let (_, id) = line.split_at(index + 1);
            match id.parse::<usize>() {
                Ok(id) => *unique_beers.entry(id).or_insert(0) += 1,
                Err(msg) => panic!("Failed to parse {}. Reason {}", id, msg),
            }
        } else {
            panic!("Failed to split {} at ' '", line);
        }
    }
    let unique_beer_count = unique_beers.len();
    println!("{} different beers", unique_beer_count);
    let mut sorted: Vec<usize> = unique_beers.into_iter().map(|t| t.1).collect();
    sorted.sort_unstable_by(|v1, v2| (*v1).cmp(v2));

    let max_node_degree = *sorted.last().unwrap();
    println!("Maximum degree: {}", max_node_degree);
    let mut count_k_degree = Vec::with_capacity(max_node_degree + 1);
    let mut current_offset = 0;
    for current_degree in 0..=max_node_degree {
        let mut degree_count = 0;
        while current_offset < sorted.len() && current_degree == sorted[current_offset] {
            degree_count += 1;
            current_offset += 1;
        }
        count_k_degree.push(degree_count as f64 / unique_beer_count as f64);
    }
    // println!("{:?}", &count_k_degree[0..10]);

    plot_to_console(&count_k_degree[0..50], 2);
    plot_to_console_log(&count_k_degree[0..500], 10);
    plot_to_console_log_log(&count_k_degree[0..500], 10);

    let mut stringified = String::new();
    for i in 0..=max_node_degree {
        let line = format!("{} {}\n", i, count_k_degree[i]);
        stringified.push_str(line.as_str());
    }
    stringified.pop();

    let _ = fs::write(
        format!("../notebooks/dd/{}", args.output_path()),
        stringified.as_str(),
    );

    Ok(())
}
