use textplots::{Chart, Plot, Shape};

pub fn plot_to_console(data: &[f64], chunk_size: usize) {
    let data: Vec<_> = data
        .chunks(chunk_size)
        .map(|slice| slice.iter().sum::<f64>())
        .enumerate()
        .map(|(e, v)| (e as f32, v as f32))
        .collect();
    Chart::new(280, 80, 1.0, 50.0)
        .lineplot(&Shape::Bars(&data[..]))
        .nice();
}

pub fn plot_to_console_log(data: &[f64], chunk_size: usize) {
    let data: Vec<_> = data
        .chunks(chunk_size)
        .map(|slice| slice.iter().sum::<f64>())
        .enumerate()
        .map(|(e, v)| (e as f32, v.ln() as f32))
        .collect();
    Chart::new(280, 80, 0.0, 10.0)
        .lineplot(&Shape::Lines(&data[..]))
        .nice();
}

pub fn plot_to_console_log_log(data: &[f64], chunk_size: usize) {
    let data: Vec<_> = data
        .chunks(chunk_size)
        .map(|slice| slice.iter().sum::<f64>())
        .enumerate()
        .map(|(e, v)| ((e as f32).ln(), v.ln() as f32))
        .collect();
    Chart::new(280, 80, 0.0, 10.0)
        .lineplot(&Shape::Lines(&data[..]))
        .nice();
}
