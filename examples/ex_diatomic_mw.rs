use diatomic_simulator::microwave::Population;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn out_csv(x_data: &Vec<f64>, y_data: &Vec<f64>, path_to_file: &str) -> Result<(), Box<dyn Error>> {
    if x_data.len() != y_data.len() {
        panic!("Error: lenght of two vectors is not equal");
    }

    let mut file = File::create(path_to_file)?;

    for (x, y) in x_data.iter().zip(y_data.iter()) {
        writeln!(file, "{x}, {y}")?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() < 6 {
        panic!("Error: invalid args");
    }

    let (file_name, temperature, j_max, origin, rot_const) = (
        &argv[1],
        &argv[2].parse::<f64>()?,
        &argv[3].parse::<i32>()?,
        &argv[4].parse::<f64>()?,
        &argv[5].parse::<f64>()?,
    );

    let spec = Population::new(*temperature, *j_max, *origin, *rot_const);
    let (sig_x, sig_y) = spec.calc_spectrum();

    out_csv(&sig_x, &sig_y, file_name)?;

    Ok(())
}
