pub mod runst {
	pub fn weight_init() {
		use rand::{thread_rng, Rng};

		//let mut m: [f64; 4] = [0.0; 4];
		let mut m = [[0.0; 2], [0.0; 2], [0.0; 2], [0.0; 2]];
		//let mut we =  
		let mut rng = thread_rng();

		for x in 0..= 3 {
			for y in 0..= 1 {
				let saut: f64 = rng.gen_range(0.0..=1.0);
				m[x][y] = saut;
			}
		}

		for x in 0..= 3 {
			println!("{:?}", m[x]); 
		}
	}
}

fn main() {
	let test = runst::weight_init();
	println!("{:?}", test);
}
