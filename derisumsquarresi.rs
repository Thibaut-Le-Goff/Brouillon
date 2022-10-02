fn main() {
    const OBSERVED_HEIGHT: [f64; 3] = [1.4, 1.9, 3.2];
    const WEIGHT: [f64; 3] = [0.5, 2.3, 2.9];
    let mut intercept: f64 = 100.0;
    let learning_rate: f64 = 0.1;
    let try_number: usize = 50;

    for _i in 0..= try_number {
        let mut sum_derivative_square_residual: f64 = 0.0;

        for j in 0..= WEIGHT.len() - 1 {

            let predicted_height: f64 = intercept + (0.64 * WEIGHT[j]);
            let derivative_square_residual: f64 = -2.0 * (OBSERVED_HEIGHT[j] - predicted_height);

            sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
        }
        println!("Avec l'intercept {:?} la dérivée de la somme des square residual est {:?}", intercept, sum_derivative_square_residual);
        let step_size: f64 = sum_derivative_square_residual * learning_rate;
        // si sum egale au précédent sum = arréter
        if sum_derivative_square_residual == step_size {
            break
        }else {
            intercept = intercept - step_size;
        }
    }
}
