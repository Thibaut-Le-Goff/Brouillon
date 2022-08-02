pub mod runst {
	//pub fn weight_init(column: usize) -> &'static usize {
	pub fn weight_init(column: usize) -> &'static Vec<f64> {
	//pub fn weight_init(column: usize) {
		use rand::{thread_rng, Rng};
				
		//let mut m: [f64; 4] = [0.0; 4];
		let m: Vec<f64> = vec![0.0; column];
		//let mut m = [0];
		
		let mut rng = thread_rng();

		for mut i in m {		
			let rand: f64 = rng.gen_range(0.0..=1.0);
			i = rand;
			//println!("{:?}", i);
		}
		
		&m
	}
}

fn main() {
	let test: &Vec<f64> = runst::weight_init(5);
	
	for i in test {
		println!("{:?}", i);
	}	
}
