fn main() {
    const OBSERVED_HEIGHT: [f64; 3] = [1.4, 1.9, 3.2];
    const WEIGHT: [f64; 3] = [0.5, 2.3, 2.9];

    let learning_rate_slope: f64 = 0.01;
    let learning_rate_intercept: f64 = 0.1;

    let try_number: usize = 1000;
    let precision_success: f64 = 0.01;

    // The slope will have as many decimal as precision_success
    // let batch_number: usize = 2;
    // pour mini batche :
    // for j in 0..= batch_number - 1 {  a la place de for j in WEIGHT.len() -1
    //  crée un nombre aléatoire x entre 0 et OBSERVED_HEIGHT.len()
    //  utilise x dans WEIGHT[x] et OBSERVED_HEIGHT[x]

    let mut slope: f64 = 0.0;
    let mut intercept: f64 = 0.0;

    let mut sum_derivative_square_residual_slope: f64;
    let mut sum_derivative_square_residual_intercept: f64;

    let mut previous_sum_derivative_square_residual_slope: f64 = 0.0;
    let mut previous_sum_derivative_square_residual_intercept: f64 = 0.0;

    let mut predicted_height: f64;
    let mut derivative_square_residual_slope: f64;
    let mut derivative_square_residual_intercept: f64;

    let mut step_size: f64;

    let mut slop_finale_trouve: bool = false;
    let mut intercept_finale_trouve: bool = false;

    for _i in 0..= try_number {

        if slop_finale_trouve == false {
            // met le "compteur" de la somme a zero
            sum_derivative_square_residual_slope = 0.0;  

            // calcule d ssr par rapport au coéficient directeur 
            // de la courbe des prédictions        
            for j in 0..= WEIGHT.len() - 1 {
                predicted_height = intercept + (slope * WEIGHT[j]);
                derivative_square_residual_slope = (-2.0 * WEIGHT[j]) * (OBSERVED_HEIGHT[j] - predicted_height);

                sum_derivative_square_residual_slope = derivative_square_residual_slope + sum_derivative_square_residual_slope;
            }

            // calcule step size, du pas
            step_size = sum_derivative_square_residual_slope * learning_rate_slope;

            // determination de la prochaine valeur du coéficient directeur
            slope = slope - step_size;
            println!("Avec le coéficient directeur de la droite de prediction {:?} la dérivée de la somme des square residual est {:?}", slope, sum_derivative_square_residual_slope);
            
            if (sum_derivative_square_residual_slope / precision_success).floor() == (previous_sum_derivative_square_residual_slope / precision_success).floor() {
                println!("fini de trouver le bon coéficient directeur de la droite de prediction  ! ");
                slop_finale_trouve = true;
                println!("résultat : {:?}", slope);
            }
            previous_sum_derivative_square_residual_slope = sum_derivative_square_residual_slope;
        }

        if intercept_finale_trouve == false {
            sum_derivative_square_residual_intercept = 0.0;

            for j in 0..= WEIGHT.len() - 1 { // calcule d ssr par rapport a intercept
                predicted_height = intercept + (slope * WEIGHT[j]);
                derivative_square_residual_intercept = -2.0 * (OBSERVED_HEIGHT[j] - predicted_height);

                sum_derivative_square_residual_intercept = derivative_square_residual_intercept + sum_derivative_square_residual_intercept;
            }

            step_size = sum_derivative_square_residual_intercept * learning_rate_intercept;
            
            intercept = intercept - step_size;
            println!("Avec l'intercept {:?} la dérivée de la somme des square residual est {:?}", intercept, sum_derivative_square_residual_intercept);
            // si sum egale au précédent sum = arréter
            
            if (sum_derivative_square_residual_intercept / precision_success).floor() == (previous_sum_derivative_square_residual_intercept / precision_success).floor() {
                println!("fini de trouver le bon intercept de la droite de prediction  ! ");
                intercept_finale_trouve = true;
                println!("résultat : {:?}", intercept);
            }

            previous_sum_derivative_square_residual_intercept = sum_derivative_square_residual_intercept;
        }

        if intercept_finale_trouve == true && slop_finale_trouve == true {
            println!("fini de trouver le bon coéficient directeur de la droite de prediction  ! ");
            slop_finale_trouve = true;
            println!("résultat : {:?}", slope);

            println!("fini de trouver le bon intercept de la droite de prediction  ! ");
            intercept_finale_trouve = true;
            println!("résultat : {:?}", intercept);

            break //free
        }
    }
}