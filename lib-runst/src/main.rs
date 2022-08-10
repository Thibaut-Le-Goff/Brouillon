mod runst;

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