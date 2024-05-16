use crate::utl::DiatomicMolecule;

pub struct Population {
    pub temperature: f64,
    pub j_max: f64,
    pub band_origin: f64,
    molecule: DiatomicMolecule,
}

impl Population {
    pub fn new(temperature: f64, j_max: f64, band_origin: f64, rot_const: f64) -> Self {
        Self {
            temperature,
            j_max,
            band_origin,
            molecule: DiatomicMolecule::new(rot_const),
        }
    }

    pub fn rot_const(&self) -> f64 {
        self.molecule.rot_const
    }

    pub fn rot_const_ref(&mut self) -> &mut f64 {
        &mut self.molecule.rot_const
    }

    pub fn calc_spectrum(&self) -> (Vec<f64>, Vec<f64>) {
        let mut signal_x = vec![];
        let mut signal_y = vec![];
        let mut partition_func = 0.0;

        let j_max = self.j_max as i32;
        for j in 0..=j_max {
            for delta_j in [-1, 0, 1] {
                let (energy_1, energy_2) =
                    (self.molecule.energy(j), self.molecule.energy(j + delta_j));
                signal_x.push(energy_2 - energy_1 + self.band_origin);
                signal_y.push(
                    (j as f64 + 1.0) / (2.0 * j as f64 + 1.0)
                        * ((-energy_1 / self.temperature).exp()
                            - (-energy_2 / self.temperature).exp())
                        .abs(),
                );
            }

            partition_func +=
                (2.0 * j as f64 + 1.0) * (-self.molecule.energy(j) / self.temperature).exp();
        }

        for intensity in signal_y.iter_mut() {
            *intensity /= partition_func;
        }

        (signal_x, signal_y)
    }
}
