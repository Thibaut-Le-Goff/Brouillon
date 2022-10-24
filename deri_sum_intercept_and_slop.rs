fn main() {
    // issue de : https://www.youtube.com/watch?v=sDv4f4s2SB8&t=815s 

    const OBSERVED_HEIGHT: [f64; 3] = [1.4, 1.9, 3.2];
    // les tailles de trois personne, la taille est 
    // la donné qu'on veut prévoir, à partir de ces trois
    // échantillions
    const WEIGHT: [f64; 3] = [0.5, 2.3, 2.9];
    // leur poids

    let try_number: usize = 1000;
    // nombre d'essayes

    let mut slope_intercept_trouve: [bool; 2] = [false, false];
    // indication si les valeurs attendue ont 
    // était trouvées
    
    let precision_success: f64 = 0.001;
    // le programme s'arêtera lorsque que la somme
    // des dérivées du carré de la différence entre les 
    // données observées et prévues est entre cette 
    // valeur et sont négatif

    let mut step_size: f64;
    // taille des pas dans le rapprochement de 
    // sum_derivative_square_residual

    let slope_intercept_learning_rate: [f64; 2] = [0.01, 0.1];
    // pour 0 :
    // coéficient de la taille des pas dans la calcule du step
    // size pour le coéficient directeur de la courbe des prédictions

    // pour 1 :
    // coéficient de la taille des pas dans la calcule du step
    // size pour l'intercept de la courbe des prédictions

    let mut sum_derivative_square_residual: f64;
    let mut derivative_square_residual: f64;
    // la somme des dérivés du carré de la différence 
    // entre la valeur observé et celle attendue
    // pour le calcule du coéficient directeur de la
    // courbes des prédictions a N-1 et N


    // <brouilon>
    // let batch_number: usize = 2;
    // pour mini batche :
    // for j in 0..= batch_number - 1 {  a la place de for j in WEIGHT.len() -1
    //  crée un nombre aléatoire x entre 0 et OBSERVED_HEIGHT.len()
    //  utilise x dans WEIGHT[x] et OBSERVED_HEIGHT[x]
    // </brouilon>


    let mut slope_intercept: [f64; 2] = [0.0, 0.0];
    // pour 0:
    // valeur de départ du coéficient directeur de la courbe
    // des prédictions

    // pour 1:
    // valeur de départ de l'intercept de la courbe
    // des prédictions

    let mut predicted_height: f64;
    // la ou sera stocké la taille prédite
    // par rapport à slope et intercept

    for _i in 0..= try_number - 1 {
    // pour le nombre d'essayes indiqués

        for y in 0..= slope_intercept_trouve.len() - 1 {
        // pour chaque info (slope, intercept)...
            if slope_intercept_trouve[y] == false {
                // met le "compteur" de la somme a zero
                sum_derivative_square_residual = 0.0;

                // calcule d ssr par rapport au coéficient directeur de la courbe des prédictions        
                for j in 0..= WEIGHT.len() - 1 {
                    predicted_height = slope_intercept[1] + (slope_intercept[0] * WEIGHT[j]);
                    
                    if y == 0 {
                        derivative_square_residual = (-2.0 * WEIGHT[j]) * (OBSERVED_HEIGHT[j] - predicted_height);
                        sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
                    }

                    if y == 1 {
                        derivative_square_residual = -2.0 * (OBSERVED_HEIGHT[j] - predicted_height);
                        sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
                    }
                }

                // calcule step size, le pas
                step_size = sum_derivative_square_residual * slope_intercept_learning_rate[y];

                // determination de la prochaine valeur de la valeur
                slope_intercept[y] = slope_intercept[y] - step_size;

                if sum_derivative_square_residual <= precision_success && sum_derivative_square_residual >= -precision_success {
                    slope_intercept_trouve[y] = true;

                    if y == 0 {
                        println!("\n\nfini de trouver le bon coéficient directeur de la droite de prediction  ! ");
                        println!("Le coéficient directeur : {:?}", slope_intercept[y]);
                    }

                    if y == 1 {
                        println!("\n\nfini de trouver le bon intercept de la droite de prediction  ! ");
                        println!("L'intercept : {:?}", slope_intercept[y]);
                    }

                }
            }
        }
    }
}