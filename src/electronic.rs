use crate::utl::DiatomicMolecule;

pub struct Population {
    pub temperature: f64,
    pub j_max: i32,
    pub band_origin: f64,
    molecule_ground: DiatomicMolecule,
    molecule_excited: DiatomicMolecule,
}

impl Population {
    pub fn new(
        temperature: f64,
        j_max: i32,
        band_origin: f64,
        rot_const_ground: f64,
        rot_const_excited: f64,
    ) -> Self {
        Self {
            temperature,
            j_max,
            band_origin,
            molecule_ground: DiatomicMolecule::new(rot_const_ground),
            molecule_excited: DiatomicMolecule::new(rot_const_excited),
        }
    }

    pub fn update_rot_cosnt(&mut self, rot_const_ground: f64, rot_const_excited: f64) {
        self.molecule_ground.rot_const = rot_const_ground;
        self.molecule_excited.rot_const = rot_const_excited;
    }

    pub fn calc_spectrum(&self) -> (Vec<f64>, Vec<f64>) {
        let mut signal_x = vec![];
        let mut signal_y = vec![];
        let (mut partition_func_ground, mut partition_func_excited) = (0.0, 0.0);

        for j in 0..=self.j_max {
            partition_func_ground +=
                (2.0 * j as f64 + 1.0) * (-self.molecule_ground.energy(j) / self.temperature).exp();
            partition_func_excited += (2.0 * j as f64 + 1.0)
                * (-self.molecule_excited.energy(j) / self.temperature).exp();
        }

        for j in 0..=self.j_max {
            for delta_j in [-1, 0, 1] {
                let (energy_ground, energy_excited) = (
                    self.molecule_ground.energy(j),
                    self.molecule_excited.energy(j + delta_j),
                );
                signal_x.push(energy_excited - energy_ground + self.band_origin);
                signal_y.push(
                    (j as f64 + 1.0) / (2.0 * j as f64 + 1.0)
                        * ((-energy_excited / self.temperature).exp() / partition_func_excited
                            - (-energy_ground / self.temperature).exp() / partition_func_ground)
                            .abs(),
                );
            }
        }

        (signal_x, signal_y)
    }
}
