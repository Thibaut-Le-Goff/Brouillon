fn main() {
    const OBSERVED_HEIGHT: [f64; 3] = [1.4, 1.9, 3.2];
    const WEIGHT: [f64; 3] = [0.5, 2.3, 2.9];
    let learning_rate: f64 = 0.01;
    let try_number: usize = 50;
    let precision_success: f64 = 0.01; 
    // The slope will have as many decimal as precision_success
    // let batch_number: usize = 2;
    // pour mini batche :
    // for j in 0..= batch_number - 1 {  a la place de for j in WEIGHT.len() -1
    //  crée un nombre aléatoire x entre 0 et OBSERVED_HEIGHT.len()
    //  utilise x dans WEIGHT[x] et OBSERVED_HEIGHT[x]

    let mut slope: f64 = 100.0;
    let mut sum_derivative_square_residual: f64;
    let mut previous_sum_derivative_square_residual: f64;
    let mut predicted_height: f64;
    let mut derivative_square_residual: f64;
    let mut step_size: f64;

    for _i in 0..= try_number {
        sum_derivative_square_residual = 0.0;
        previous_sum_derivative_square_residual = sum_derivative_square_residual;

        for j in 0..= WEIGHT.len() - 1 {
            predicted_height = 1.0 + (slope * WEIGHT[j]);
            derivative_square_residual = (-2.0 * WEIGHT[j]) * (OBSERVED_HEIGHT[j] - predicted_height);

            sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
        }
        println!("Avec le coéficient directeur de la droite de prediction {:?} la dérivée de la somme des square residual est {:?}", slope, sum_derivative_square_residual);

        step_size = sum_derivative_square_residual * learning_rate;
        // si sum egale au précédent sum = arréter
        if (sum_derivative_square_residual / precision_success).floor() == (previous_sum_derivative_square_residual / precision_success).floor() {
            break
        }else {
            slope = slope - step_size;
        }
    }
}
