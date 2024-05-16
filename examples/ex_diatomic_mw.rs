use diatomic_simulator::microwave::Population;
use diatomic_simulator::utl::{convolute_lorentz, out_csv, LineShape};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() < 6 {
        panic!("Error: invalid args");
    }

    let (file_name, temperature, j_max, origin, rot_const) = (
        &argv[1],
        &argv[2].parse::<f64>()?,
        &argv[3].parse::<f64>()?,
        &argv[4].parse::<f64>()?,
        &argv[5].parse::<f64>()?,
    );

    let lorentz_line_shape = LineShape::new(0.04);
    let spec = Population::new(*temperature, *j_max, *origin, *rot_const);
    let (signal_raw_x, signal_raw_y) = spec.calc_spectrum();

    let (x_signal, y_signal) = convolute_lorentz(
        1000.0,
        1400.0,
        0.01,
        &lorentz_line_shape,
        (&signal_raw_x, &signal_raw_y),
    );

    out_csv(&x_signal, &y_signal, file_name)?;

    Ok(())
}
