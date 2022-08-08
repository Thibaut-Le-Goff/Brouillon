pub mod runst {
	pub fn multiply(matrix: Vec<Vec<f64>>, vector: Vec<f64>) -> Vec<f64> {
        let mut result: Vec<f64> = vec![0.0; matrix.len()];

        for i in 0..= (matrix.len() - 1) { // row
            let mut x: f64 = 0.0;
            for j in 0..= (vector.len() - 1) { // column
                x = (vector[j] * matrix[i][j]) + x;
            }
            result[i] = x;
        }
        return result;
    }

	pub mod weight_init {
        pub fn random(column: usize, row: usize, a: f64, b: f64) -> Vec<Vec<f64>> {
			use rand::{thread_rng, Rng};
				
			let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; column]; row];
			let mut rng = thread_rng();

	    	for i in 0..= (row - 1) {	
                for j in 0..= (column - 1) {
					let rand: f64 = rng.gen_range(a..=b);
        	    	matrix[i][j] = rand;
        		}
		    }
			return matrix;
        }

		pub fn normal_dis(column: usize, row: usize) -> Vec<Vec<f64>> {
            let a: f64 = 0.0;
            let b: f64 = 1.0;
	        let matrix: Vec<Vec<f64>> = random(column, row, a, b);

			return matrix;
        }

		pub fn uniform_dis(column: usize, row: usize) -> Vec<Vec<f64>> {
            let a: f64 = -1.0 / (column as f64).sqrt();
            let b: f64 = 1.0 / (column as f64).sqrt();
            // ;sqrt() only works with float
	        let matrix: Vec<Vec<f64>> = random(column, row, a, b);
		    
            return matrix;
        }

		pub fn xav_gro_normal_dis(column: usize, row: usize, fan_out: usize) -> Vec<Vec<f64>> {
            let a: f64 = 0.0;
            let b: f64 = (2.0 / (column as f64) + (fan_out as f64)).sqrt();
	        let matrix: Vec<Vec<f64>> = random(column, row, a, b);
		    
            return matrix;
        }

		pub fn xav_gro_uniform_dis(column: usize, row: usize, fan_out: usize) -> Vec<Vec<f64>> {
            let a: f64 = -(6_f64.sqrt()) / ((column as f64) + (fan_out as f64)).sqrt();
            let b: f64 = 6_f64.sqrt() / ((column as f64) + (fan_out as f64)).sqrt();
	        let matrix: Vec<Vec<f64>> = random(column, row, a, b);
		    
            return matrix;
        }

		pub fn he_normal_dis(column: usize, row: usize) -> Vec<Vec<f64>> {
            let a: f64 = 0.0;
            let b: f64 = (2.0 / (column as f64)).sqrt();
	        let matrix: Vec<Vec<f64>> = random(column, row, a, b);
		    
            return matrix;
        }

		pub fn he_uniform_dis(column: usize, row: usize) -> Vec<Vec<f64>> {
            let a: f64 = -(6.0 / (column as f64)).sqrt();
            let b: f64 = (6.0 / (column as f64)).sqrt();
	        let matrix: Vec<Vec<f64>> = random(column, row, a, b);

		    return matrix;
        }
    }
}

fn main() {
	let row: usize = 3; // nb neurones layer n
	let column: usize = 2; // nb neurones layer n-1, fan_in

	let test: Vec<Vec<f64>> = runst::weight_init::normal_dis(column, row);

	for i in 0..= (row - 1) {
        	println!("{:?}", test[i]);
    	}

	for i in 0..= (row - 1) {
		for j in 0..= (column - 1)  {
			println!("{}", test[i][j]);
		}
	}	
    
}
