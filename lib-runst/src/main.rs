pub mod runst {
	pub mod weight_init {
		pub fn normal_dis(column: usize, row: usize) -> Vec<Vec<f64>> {
			use rand::{thread_rng, Rng};
				
			let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; column]; row];
			let mut rng = thread_rng();
            		let a: f64 = 0.0;
            		let b: f64 = 1.0;

	    		for i in 0..= (row - 1) {	
                		for j in 0..= (column - 1) {
					let rand: f64 = rng.gen_range(a..=b);
        	    			matrix[i][j] = rand;
        			}
		    	}
			 return matrix;
        	}
		pub fn uniform_dis(column: usize, row: usize) -> Vec<Vec<f64>> {
			use rand::{thread_rng, Rng};
				
		    	let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; column]; row];
		    	let mut rng = thread_rng();
            		let a: f64 = -1.0 / (column as f64).sqrt();
            		let b: f64 = 1.0 / (column as f64).sqrt();
			// .sqrt() works only with float
	
	    		for i in 0..= (row - 1) {	
            			for j in 0..= (column - 1) {
				    let rand: f64 = rng.gen_range(a..=b);
        	    		matrix[i][j] = rand;
        			}
		    	}	
		    	return matrix;
        	}
    	}
}

fn main() {
	let row: usize = 3; // nb neurones layer n
	let column: usize = 2; // nb neurones layer n-1, fan_in

	let test: Vec<Vec<f64>> = runst::weight_init::uniform_dis(column, row);

	for i in 0..= (row - 1) {
        	println!("{:?}", test[i]);
    	}

	for i in 0..= (row - 1) {
		for j in 0..= (column - 1)  {
			println!("{}", test[i][j]);
		}
	}	
    
    	println!("test de calcul :");

    	println!(" + 1 :");
	for i in 0..= (row - 1) {
		for j in 0..= (column - 1)  {
			println!("{}", (test[i][j] + 1.0));
		}
	}	

    	println!(" - 1 :");
	for i in 0..= (row - 1) {
		for j in 0..= (column - 1)  {
			println!("{}", (test[i][j] - 1.0));
		}
	}

    	println!(" * 2 :");
	for i in 0..= (row - 1) {
		for j in 0..= (column - 1)  {
			println!("{}", (test[i][j] * 2.0));
		}
	}

    	println!(" / 2 :");
	for i in 0..= (row - 1) {
		for j in 0..= (column - 1)  {
			println!("{}", (test[i][j] / 2.0));
		}
	}	
}
