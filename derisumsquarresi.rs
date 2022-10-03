fn main() {
    const OBSERVED_HEIGHT: [f64; 3] = [1.4, 1.9, 3.2];
    const WEIGHT: [f64; 3] = [0.5, 2.3, 2.9];

    let learning_rate: f64 = 0.1;
    let precision_success: f64 = 0.01; 
    // The intercept  will have as many decimal as precision_success
    let try_number: usize = 50;

    let mut intercept: f64 = 100.0;

    let mut sum_derivative_square_residual: f64;
    let mut previous_sum_derivative_square_residual: f64;
    let mut predicted_height: f64;
    let mut derivative_square_residual: f64;
    let mut step_size: f64;

    for _i in 0..= try_number {
        sum_derivative_square_residual = 0.0;
        previous_sum_derivative_square_residual = sum_derivative_square_residual;

        for j in 0..= WEIGHT.len() - 1 {
            predicted_height = intercept + (0.64 * WEIGHT[j]);
            derivative_square_residual = -2.0 * (OBSERVED_HEIGHT[j] - predicted_height);

            sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
        }
        println!("Avec l'intercept {:?} la dérivée de la somme des square residual est {:?}", intercept, sum_derivative_square_residual);

        step_size = sum_derivative_square_residual * learning_rate;
        // si sum egale au précédent sum = arréter
        if (sum_derivative_square_residual / precision_success).floor() == (previous_sum_derivative_square_residual / precision_success).floor() {
            break
        }else {
            intercept = intercept - step_size;
        }
    }
}
