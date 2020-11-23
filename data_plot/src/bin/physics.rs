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
        if self.path.contains("Astro") {
            "astro.txt"
        } else if self.path.contains("CondMat") {
            "condmat.txt"
        } else {
            panic!()
        }
    }
}

fn main() -> Result<(), Error> {
    let args: Args = Args::from_args();
    let file_data = fs::read(args.path.as_str())?;
    let file_data = String::from_utf8(file_data).unwrap();

    let mut unique_authors = HashMap::new();
    for line in file_data.lines().filter(|line| !line.starts_with('#')) {
        if let Some(index) = line.find('\t') {
            let (id1, id2) = line.split_at(index);
            match id1.trim().parse::<usize>() {
                Ok(id) => *unique_authors.entry(id).or_insert(0) += 1,
                Err(msg) => panic!("Failed to parse {}. Reason {}", id1, msg),
            }
            match id2.trim().parse::<usize>() {
                Ok(id) => *unique_authors.entry(id).or_insert(0) += 1,
                Err(msg) => panic!("Failed to parse {}. Reason {}", id2, msg),
            }
        } else {
            panic!("Failed to split {} at '\'", line);
        }
    }
    let unique_author_count = unique_authors.len();
    println!("{} different authors", unique_author_count);
    let mut sorted: Vec<usize> = unique_authors.into_iter().map(|t| t.1).collect();
    sorted.sort_unstable_by(|v1, v2| (*v1).cmp(v2));

    let &max_node_degree = sorted.last().unwrap();
    println!("Maximum degree: {} for node", max_node_degree);
    let mut count_k_degree = Vec::with_capacity(max_node_degree + 1);
    let mut current_offset = 0;
    for current_degree in 0..=max_node_degree {
        let mut degree_count = 0;
        if current_degree == 1008 {
            println!("Offset: {}, Sorted: {}", current_offset, sorted[current_offset]);
        }
        while current_offset < sorted.len() && current_degree == sorted[current_offset] {
            degree_count += 1;
            current_offset += 1;
        }
        count_k_degree.push(degree_count as f64 / unique_author_count as f64);
    }

    plot_to_console(&count_k_degree[0..100], 1);
    plot_to_console_log(&count_k_degree[0..100], 5);
    plot_to_console_log_log(&count_k_degree[0..100], 5);

    let mut stringified = String::new();
    for i in 0..=max_node_degree {
        let line = format!("{} {}\n", i, count_k_degree[i]);
        stringified.push_str(line.as_str());
    }
    stringified.pop();

    fs::write(
        format!("../notebooks/dd/{}", args.output_path()),
        stringified.as_str(),
    )?;

    Ok(())
}
