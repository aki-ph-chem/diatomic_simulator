use crate::utl::DiatomicMolecule;

pub struct Population {
    pub temperature: f64,
    pub j_max: i32,
    pub band_origin: f64,
    molecule: DiatomicMolecule,
}

impl Population {
    pub fn new(temperature: f64, j_max: i32, band_origin: f64, rot_const: f64) -> Self {
        Self {
            temperature,
            j_max,
            band_origin,
            molecule: DiatomicMolecule::new(rot_const),
        }
    }

    pub fn calc_spectrum(&self) -> (Vec<f64>, Vec<f64>) {
        let mut signal_x = vec![0.0; self.j_max as usize + 1];
        let mut partition_func = 0.0;

        for j in 0..=self.j_max {
            // j -> j + 1 only
            signal_x[j as usize] = 2.0 * self.molecule.rot_const * (j as f64 + 1.0);
            partition_func +=
                (2.0 * j as f64 + 1.0) * (-self.molecule.energy(j) / self.temperature).exp();
        }

        let signal_y = (0..=self.j_max)
            .map(|j| {
                (j as f64 + 1.0) / (2.0 * j as f64 + 1.0)
                    * ((-self.molecule.energy(j) / self.temperature).exp()
                        - (-self.molecule.energy(j + 1) / self.temperature).exp())
                    .abs()
                    / partition_func
            })
            .collect();

        (signal_x, signal_y)
    }
}
