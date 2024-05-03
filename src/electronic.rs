use crate::utl::DiatomicMolecule;

pub struct Populatoin {
    pub temperature: f64,
    pub j_max: i32,
    pub band_origin: f64,
    molecule_grownd: DiatomicMolecule,
    molecule_excited: DiatomicMolecule,
}

impl Populatoin {
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
            molecule_grownd: DiatomicMolecule::new(rot_const_ground),
            molecule_excited: DiatomicMolecule::new(rot_const_excited),
        }
    }

    pub fn update_rot_cosnt(&mut self, rot_const_ground: f64, rot_const_excited: f64) {
        self.molecule_grownd.rot_const = rot_const_ground;
        self.molecule_excited.rot_const = rot_const_excited;
    }

    /*
    pub fn spectrum(&self) -> Vec<(f64, f64)> {
        let mut signal_x = vec![0.0; self.j_max as usize + 1];
        let mut partition_func = 0.0;

        for j in 0..=self.j_max {
            for delta_j in [-1, 0, 1] {
            }
        }
    }
    */
}
