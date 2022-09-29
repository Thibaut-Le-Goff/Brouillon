fn main() {
    const OBSERVED_HEIGHT: [f64; 3] = [1.5, 2.0, 3.5];
    const OBSERVED_WEIGHT: [f64; 3] = [0.5, 2.5, 3.0];
    
    for i in 0..= 10 {
        let intercept: f64 = i as f64;
        let mut square_residual: f64 = 0.0;

        for j in 0..= OBSERVED_WEIGHT.len() - 1 {

            let predicted_height: f64 = intercept + (0.6 * OBSERVED_WEIGHT[j]);
            let residual: f64 = OBSERVED_HEIGHT[j] - predicted_height;

            square_residual = (residual * residual) + square_residual;
        }
        println!("Avec l'intercept {:?} le square residual est {:?}", i, square_residual);
    }
}
