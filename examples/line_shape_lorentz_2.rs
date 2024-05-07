use diatomic_simulator::utl::{convolute_lorentz_2, out_csv, LineShape};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() < 3 {
        panic!("Error: invalid args");
    }
    let (file_name, lorentz_width) = (&argv[1], &argv[2].parse::<f64>()?);

    let lorentz_line_shape = LineShape::new(*lorentz_width);
    let (x_signal_raw, y_signal_raw) = (
        vec![1.2, 1.5, 2.1, 2.3, 3.8, 4.1],
        vec![6.0, 6.2, 6.5, 2.5, 8.1, 5.1],
    );
    let (x_signal, y_signal) = convolute_lorentz_2(
        0.0,
        5.0,
        0.01,
        0.01,
        &lorentz_line_shape,
        (&x_signal_raw, &y_signal_raw),
    );

    out_csv(&x_signal, &y_signal, file_name)?;

    Ok(())
}
