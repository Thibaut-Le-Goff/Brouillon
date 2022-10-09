fn main() {
    // issue de : https://www.youtube.com/watch?v=sDv4f4s2SB8&t=815s 

    const OBSERVED_HEIGHT: [f64; 3] = [1.4, 1.9, 3.2];
    // les tailles de trois personne, la taille est 
    // la donné qu'on veut prévoir, à partir de ces trois
    // échantillions
    const WEIGHT: [f64; 3] = [0.5, 2.3, 2.9];
    // leur poids

    let try_number: usize = 50;
    // nombre d'essayes
    //let precision_success: f64 = 0.001;
    // valeur dont sera divisé les sum_square_residual 
    // aux itération N-1 et N pour les comparer, les deux
    // résulatats sont arrondies à la valeur inférieur 
    // pour une comparaison qui puisse dire que les 
    // deux résultats sont les mêmes
    // ex : 0.00977 et 0.00955
    // = 9 et 9 => la bonne valeur a était trouvé

    let mut step_size: f64; // minimum doit être 0.001
    // taille des pas dans le rapprochement de 
    // sum_derivative_square_residual
    let learning_rate_slope: f64 = 0.01;
    // coéficient de la taille des pas dans la calcule du step
    // size pour le coéficient directeur de la courbe des prédictions
    let learning_rate_intercept: f64 = 0.1;
    // coéficient de la taille des pas dans la calcule du step
    // size pour l'intercept de la courbe des prédictions

    //let mut previous_sum_derivative_square_residual_slope: f64;
    let mut sum_derivative_square_residual_slope: f64 = 0.0;
    let mut derivative_square_residual_slope: f64;
    // la somme des dérivés du carré de la différence 
    // entre la valeur observé et celle attendue
    // pour le calcule du coéficient directeur de la
    // courbes des prédictions a N-1 et N

    //let mut previous_sum_derivative_square_residual_intercept: f64;
    let mut sum_derivative_square_residual_intercept: f64 = 0.0;
    let mut derivative_square_residual_intercept: f64;
    // la somme des dérivés du carré de la différence 
    // entre la valeur observé et celle attendue
    // pour le calcule de l'intercept de la
    // courbes des prédictions N-1 et N


    // <brouilon>
    // let batch_number: usize = 2;
    // pour mini batche :
    // for j in 0..= batch_number - 1 {  a la place de for j in WEIGHT.len() -1
    //  crée un nombre aléatoire x entre 0 et OBSERVED_HEIGHT.len()
    //  utilise x dans WEIGHT[x] et OBSERVED_HEIGHT[x]
    // </brouilon>


    let mut slope: f64 = 0.0;
    // valeur de départ du coéficient directeur de la courbe 
    // des prédictions
    let mut intercept: f64 = 0.0;
    // valeur de départ de l'intercept de la courbe 
    // des prédictions
    let mut predicted_height: f64;
    // la ou sera stocké la taille prédite
    // par rapport à slope et intercept

    let step_size_stop: f64 = 0.001;
    let mut slop_finale_trouve: bool = false;
    let mut intercept_finale_trouve: bool = false;
    // indication si les valeurs attendue ont 
    // était trouvées

    for _i in 0..= try_number - 1 {
    // pour le nombre d'essayes indiqué

        if slop_finale_trouve == false {
            // met le "compteur" de la somme a zero
            sum_derivative_square_residual_slope = 0.0;  

            // calcule d ssr par rapport au coéficient directeur de la courbe des prédictions        
            for j in 0..= WEIGHT.len() - 1 {
                predicted_height = intercept + (slope * WEIGHT[j]);
                derivative_square_residual_slope = (-2.0 * WEIGHT[j]) * (OBSERVED_HEIGHT[j] - predicted_height);

                sum_derivative_square_residual_slope = derivative_square_residual_slope + sum_derivative_square_residual_slope;
            }

            // calcule step size, le pas
            step_size = sum_derivative_square_residual_slope * learning_rate_slope;
            println!("step size slope : {:?}", step_size);

            // determination de la prochaine valeur du coéficient directeur
            slope = slope - step_size;
            println!("Avec le coéficient directeur de la droite de prediction {:?} la dérivée de la somme des square residual est {:?}", slope, sum_derivative_square_residual_slope);

            //previous_sum_derivative_square_residual_slope = sum_derivative_square_residual_slope;

            if step_size <= step_size_stop && step_size >= -step_size_stop {
                println!("fini de trouver le bon coéficient directeur de la droite de prediction  ! ");
                slop_finale_trouve = true;
                println!("résultat : {:?}", slope);
            }
        }

        if intercept_finale_trouve == false {
            // met le "compteur" de la somme a zero
            sum_derivative_square_residual_intercept = 0.0;

            // calcule d ssr par rapport a intercept de la courbe des prédictions
            for j in 0..= WEIGHT.len() - 1 { 
                predicted_height = intercept + (slope * WEIGHT[j]);
                derivative_square_residual_intercept = -2.0 * (OBSERVED_HEIGHT[j] - predicted_height);

                sum_derivative_square_residual_intercept = derivative_square_residual_intercept + sum_derivative_square_residual_intercept;
            }

            // calcule step size, le pas
            step_size = sum_derivative_square_residual_intercept * learning_rate_intercept;
            println!("step size intercept : {:?}", step_size);

            // determination du prochain intercept
            intercept = intercept - step_size;
            println!("Avec l'intercept {:?} la dérivée de la somme des square residual est {:?}", intercept, sum_derivative_square_residual_intercept);
        
            //previous_sum_derivative_square_residual_slope = sum_derivative_square_residual_slope;

            if step_size <= step_size_stop && step_size >= -step_size_stop {
                println!("fini de trouver le bon intercept de la droite de prediction  ! ");
                intercept_finale_trouve = true;
                println!("résultat : {:?}", intercept);
            }
        }
    }
}
