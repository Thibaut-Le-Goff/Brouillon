pub mod runst {
	pub fn weight_init(column: usize) -> Vec<f64> {
		use rand::{thread_rng, Rng};
				
		let mut m: Vec<f64> = vec![0.0; column];
		let mut rng = thread_rng();

		for i in 0..= (column - 1) {		
			let rand: f64 = rng.gen_range(0.0..=1.0);
			m[i] = rand;
		}
		m
	}
}

fn main() {
	let test: Vec<f64> = runst::weight_init(5);
	
	for i in &test {
		println!("{:?}", i);
	}	

    println!("tests de calcule :");
    println!(" + 1 :");
	for i in &test {
		println!("{:?}", (i + 1.0));
	}

    println!(" - 1 :");
	for i in &test {
		println!("{:?}", (i - 1.0));
	}	

    println!(" * 2 :");
	for i in &test {
		println!("{:?}", (i * 2.0));
	}

    println!(" / 2 :");
	for i in &test {
		println!("{:?}", (i / 2.0));
	}	
}
