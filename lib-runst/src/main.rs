pub mod runst {
	pub fn weight_init(row: usize, column: usize) -> Vec<Vec<f64>> {
		use rand::{thread_rng, Rng};
				
		let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; column]; row];
		let mut rng = thread_rng();

		for i in 0..= (row - 1) {	
            		for j in 0..= (column - 1) {
				let rand: f64 = rng.gen_range(0.0..=1.0);
            			matrix[i][j] = rand;
            		}
		}
		matrix
	}
}

fn main() {
	let row: usize = 3;
	let column: usize = 2;

	let test: Vec<Vec<f64>> = runst::weight_init(row, column);

    for i in 0..= (row - 1) {
        println!("{:?}", test[i]);
    }

	for i in 0..= (row - 1) {
		for j in 0..= (column - 1)  {
			println!("{:?}", test[i][j]);
		}
	}	
    
    println!("test de calcul :");

    println!(" + 1 :");
	for i in 0..= (row - 1) {
		for j in 0..= (column - 1)  {
			println!("{:?}", (test[i][j] + 1.0));
		}
	}	

    println!(" - 1 :");
	for i in 0..= (row - 1) {
		for j in 0..= (column - 1)  {
			println!("{:?}", (test[i][j] - 1.0));
		}
	}

    println!(" * 2 :");
	for i in 0..= (row - 1) {
		for j in 0..= (column - 1)  {
			println!("{:?}", (test[i][j] * 2.0));
		}
	}

    println!(" / 2 :");
	for i in 0..= (row - 1) {
		for j in 0..= (column - 1)  {
			println!("{:?}", (test[i][j] / 2.0));
		}
	}	
}
