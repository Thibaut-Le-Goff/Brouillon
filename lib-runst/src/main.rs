pub mod runst {
	pub fn weight_init(column: usize, row: usize) -> Vec<Vec<f64>> {//
	//pub fn weight_init(column: usize) -> Vec<f64> {
		use rand::{thread_rng, Rng};
				
		//let mut m: Vec<f64> = vec![0.0; column];
		let mut matrix = vec![];//
		let mut rng = thread_rng();

		for i in 0..= (column - 1) {	
            		let add_row: Vec<f64> = vec![0.0; row];//
            		matrix[i] = add_row;//
            		for j in 0..= (row - 1) {//
				let rand: f64 = rng.gen_range(0.0..=1.0);
				//m[i] = rand;
            			matrix[i][j] = rand;//
            		}//
		}
		//m
		matrix//
	}
}

fn main() {
	let test: Vec<Vec<f64>> = runst::weight_init(5, 3);

	for i in &test {
		println!("{:?}", i);
	}

//	for i in &test {
//		for j in &test {
//			println!("{:?}", test[i][j]);
//		}
//	}	

//	println!("tests de calcule :");
//	println!(" + 1 :");
//	for i in &test {
//		for j in &test {
//			println!("{:?}", (j + 1.0));
//		}
//	}

    	//println!(" - 1 :");
	//for i in &test {
	//	println!("{:?}", (i - 1.0));
//	}	

  //  	println!(" * 2 :");
//	for i in &test {
//		println!("{:?}", (i * 2.0));
//	}

  //  	println!(" / 2 :");
//	for i in &test {
//		println!("{:?}", (i / 2.0));
//	}	
}
