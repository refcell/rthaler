
#[derive(Debug, Clone)]
pub struct Prover {
	pub g: MultiPoly,
	pub r_vec: Vec<ScalarField>,
}

impl Prover {
	pub fn new(g: &MultiPoly) -> Self {
		Prover {
			g: g.clone(),
			r_vec: vec![],
		}
	}
    // More Prover functions here
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
