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
