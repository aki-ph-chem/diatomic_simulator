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
}
