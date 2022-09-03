//mod runst;

//fn random(nmin1: usize, n: usize, a: f64, b: f64) -> [[f64; LAYER[0]]; LAYER[1]] {
fn random(nmin1: usize, n: usize, a: f64, b: f64) -> [[f64; LAYER[0 + nmin1]]; LAYER[0 + n]] {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();

    //let mut matrix: [[f64; LAYER[0]]; LAYER[1]] = [[0.0; LAYER[0]]; LAYER[1]];
    let mut matrix: [[f64; LAYER[0 + nmin1]]; LAYER[0 + n]] = [[0.0; LAYER[0 + nmin1]]; LAYER[0 + n]];

    //for i in 0..= n - 1 {
    for i in 0..= LAYER[n] - 1 {
        //for j in 0..= nmin1 - 1 {
        for j in 0..= LAYER[nmin1] - 1 {
            let rand: f64 = rng.gen_range(a..=b);
            matrix[i][j] = rand;
        }
    }
    return matrix;
}

const LAYER: [usize; 2] = [2, 3];

fn main() {
    let a: f64 = 0.0;
    let b: f64 = 1.0;

    //let test: [[f64; LAYER[0]]; LAYER[1]] = random(LAYER[0], LAYER[1], a, b);
    let test: [[f64; LAYER[0]]; LAYER[1]] = random(0, 1, a, b);

    let test1: [f64; LAYER[0]] = [2.0; LAYER[0]];

    println!("La matrise :");
	for i in 0..= (LAYER[1] - 1) {
        println!("{:?}", test[i]);
    }

    println!("Multipliée par le vecteur :");
    println!("{:?}", test1);

    println!("Font :");
    //let res: [f64; row] = runst::multiply(column, row, &test, &test1);
    //let res: Vec<f64> = runst::multiply(test, test1);

    //println!("{:?}", res);


    /*test concluant :
    use ndarray::Array;

    let array_vec = Array::from_vec(res);
    println!("{}", array_vec);

    let array_mat = Array::from_vec(test);
    println!("{:?}", array_mat);
    let res: Vec<f64> = runst::multiply(array_mat, test1);
    println!("{:?}", res);

    */

/*
    // test
    use rayon::prelude::*;

    fn multiply_vec(matrix: Vec<f64>, vector: Vec<f64>) -> f64 {
    //fn multiply(matrix: &Vec<Vec<f64>>, vector: &Vec<f64>) -> Vec<f64> {
        matrix.par_iter()
            .map(|&i| i * vector[matrix[i]])
            .sum()
    }

    let test12: Vec<f64> = vec![2.0, 1.0];
    let test21: Vec<f64> = vec![7.0, 3.0];
    let test2: f64 = multiply_vec(test12, test21);
    println!("{:?}", test2);

 
    fn multiply_vec(matrix_vec: &Vec<f64>, vector: &Vec<f64>) -> f64 {
        matrix_vec.par_iter()
            .map(|&j| j * vector[(j as usize) - 1])
            .sum()
    }

    fn multiply(matrix: Vec<Vec<f64>>, vector: Vec<f64>) -> Vec<f64> {
        //matrix.par_iter().for_each(
        let mut result: Vec<f64> = vec![0.0; matrix.len()];
        for i in 0..= matrix.len() - 1 {
            result[i] = multiply_vec(vector, matrix[i]);
        }
        return result;
    }

    let test12: Vec<f64> = vec![1.0, 2.0];
    let test21: Vec<f64> = vec![1.0, 2.0];
    let test2: f64 = multiply_vec(&test12, &test21);
    //let test2: Vec<f64> = multiply(test, test1);
    println!("{:?}", test2);
    */

}
