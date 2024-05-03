use std::error::Error;
use std::fs::File;
use std::io::Write;

pub struct DiatomicMolecule {
    pub rot_const: f64,
}

impl DiatomicMolecule {
    pub fn new(rot_const: f64) -> Self {
        Self { rot_const }
    }

    pub fn energy(&self, j: i32) -> f64 {
        self.rot_const * (j * j + j) as f64
    }
}

pub struct LineShape {
    width_lorentz: f64,
}

impl LineShape {
    pub fn new(width_lorentz: f64) -> Self {
        Self { width_lorentz }
    }

    pub fn lorentz(&self, x: f64, x_centor: f64) -> f64 {
        let pi = std::f64::consts::PI;
        (self.width_lorentz / 2.0 * pi)
            / ((x - x_centor).powi(2) + (self.width_lorentz / 2.0).powi(2))
    }
}

pub fn convolute_lorentz(
    x_ini: f64,
    x_fin: f64,
    x_step: f64,
    line_profile: &LineShape,
    raw_signal: (&Vec<f64>, &Vec<f64>),
) -> (Vec<f64>, Vec<f64>) {
    let len_signal = ((x_fin - x_ini) / x_step) as usize;
    let x_signal = (0..len_signal)
        .map(|x| x_ini + x as f64 * x_step)
        .collect::<Vec<f64>>();
    let mut y_signal = vec![0.0; len_signal];

    for i in 0..raw_signal.0.len() {
        for j in 0..x_signal.len() {
            y_signal[j] += raw_signal.1[i] * line_profile.lorentz(x_signal[j], raw_signal.0[i]);
        }
    }

    (x_signal, y_signal)
}

pub fn out_csv(
    x_data: &Vec<f64>,
    y_data: &Vec<f64>,
    path_to_file: &str,
) -> Result<(), Box<dyn Error>> {
    if x_data.len() != y_data.len() {
        panic!("Error: lenght of two vectors is not equal");
    }

    let mut file = File::create(path_to_file)?;

    for (x, y) in x_data.iter().zip(y_data.iter()) {
        writeln!(file, "{x}, {y}")?;
    }

    Ok(())
}
