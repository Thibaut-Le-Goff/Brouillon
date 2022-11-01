mod runst;
//from : https://www.youtube.com/watch?v=GKZoOHXGcLo&t=614s

fn main() {
    ///////////////////// Network initialisation //////////////////////////
    // The structure of the network
    println!("Initialisation du réseaux de neurones :");
    println!("Le réseau :");
    const LAYER: [usize; 3] = [1, 2, 1];
    println!("Le nombre de neurones de la première couche à la dernière :");
    println!("{:?}\n", LAYER);

    // The weights:
    println!("Les poids :");
    let mut matrix_weight_l0: Vec<f64> = runst::weight_init::normal_dis(LAYER[0], LAYER[1]);
    println!("La matrix des poids entre la couche 0(input) et 1 :");
    println!("{:?}\n", matrix_weight_l0);
   
    let mut matrix_weight_l1: Vec<f64> = runst::weight_init::normal_dis(LAYER[1], LAYER[2]);
    println!("La matrix des poids entre la couche 1 et 2 :");
    println!("{:?}\n", matrix_weight_l1);

    // Creation of the weights_tensor:
    let mut weights_tensor: [Vec<f64>; 2] = [matrix_weight_l0, matrix_weight_l1];

    // The bias:
    let mut bias_l1: Vec<f64> = vec![1.0; LAYER[1]];
    println!("Les biais de la couche 1 sont :");
    println!("{:?}\n", bias_l1);

    let mut bias_l2: Vec<f64> = vec![2.0; LAYER[2]];
    println!("Les biais de la couche 2 sont :");
    println!("{:?}\n", bias_l2);

    // Creation of the matrix of bias :
    let mut bias_matrix: [Vec<f64>; 2] = [bias_l1, bias_l2];
    
    println!("Les poids et les biais avant la propagations :");
    println!("Les poids : {:?}\n", weights_tensor);
    println!("Les biais : {:?}\n", bias_matrix);


    ////////////////////////////// Data set ///////////////////////
    const DOSAGE: [f64; 3] = [0.0, 0.5, 1.0]; // ce qui est donné au réseau
    const OBSERVED_EFFECT: [f64; 3] = [0.0, 1.0, 0.0]; // ce qui est attendu qu'il donne

    let mut network_output: Vec<Vec<f64>> = Vec::new();

    ////////////////////// PROPAGATION ////////////////////////////////////
    for i in 0..= DOSAGE.len() - 1 {
        println!("Propagation des données d'entrée :");

        // Input layer:
        println!("La couches des entrées, la numéros 0 a pour valeurs :");
        let vec_input: Vec<f64> = vec![DOSAGE[i]; LAYER[0]];

        println!("{:?}\n", &vec_input);
        println!("Dans les neurones de la couche 0(input) à 1 :");
        let vec_l0_sum: Vec<f64> = runst::multiply(&weights_tensor[0], &vec_input);
        println!("Après La multiplication :");
        println!("{:?}\n", &vec_l0_sum);
        let vec_l0_sum_bias: Vec<f64> = runst::bias_addition(&vec_l0_sum, &bias_matrix[0]);
        println!("Après l'ajout des biais :");
        println!("{:?}\n", &vec_l0_sum_bias);
        let vec_l0_activ_fun: Vec<f64> = runst::activ_fun::relu(&vec_l0_sum_bias);
        println!("Après le passage dans la function d'activation :");
        println!("{:?}\n", &vec_l0_activ_fun);

        println!("Dans les neurones de la couche 1 à 2 :");
        let vec_l1_sum: Vec<f64> = runst::multiply(&weights_tensor[1], &vec_l0_activ_fun);
        println!("Après La multiplication :");
        println!("{:?}\n", &vec_l1_sum);
        let vec_l1_sum_bias: Vec<f64> = runst::bias_addition(&vec_l1_sum, &bias_matrix[1]);
        println!("Après l'ajout des biais :");
        println!("{:?}\n", &vec_l1_sum_bias);
        let vec_l1_activ_fun: Vec<f64> = runst::activ_fun::none(&vec_l1_sum_bias);
        println!("Après le passage dans la function d'activation :");
        println!("{:?}\n", &vec_l1_activ_fun);


        //enregistrement des données:
        network_output.push(vec_input);

        //network_output.push(vec_l0_sum);
        network_output.push(vec_l0_sum_bias);
        network_output.push(vec_l0_activ_fun);

        //network_output.push(vec_l1_sum);
        network_output.push(vec_l1_sum_bias);
        network_output.push(vec_l1_activ_fun);
    }

    println!("\n\nCe que le réseaux me donne :");
    println!("{:?}", network_output);

    println!("\n\nLes poids à l'endroit :");
    for i in 0..= weights_tensor.len() - 1 {
        println!("Les poids de la couche {} :", i);
        println!("{:?}", weights_tensor[i]);
    }

    println!("\n\nLes poids à l'enver :");
    let weights_reverse: usize = weights_tensor.len() - 1;
    for i in 0..= weights_tensor.len() - 1 {
        println!("Les poids de la couche {} :", weights_reverse - i);
        println!("{:?}", weights_tensor[weights_reverse - i]);
    }

    println!("\n\nLes biais à l'endroit :");
    for i in 0..= bias_matrix.len() - 1 {
        println!("Les poids de la couche {} :", i);
        println!("{:?}", bias_matrix[i]);
    }

    println!("\n\nLes biais à l'enver :");
    let bias_reverse: usize = bias_matrix.len() - 1;
    for i in 0..= bias_matrix.len() - 1 {
        println!("Les biais de la couche {} :", bias_reverse - i);
        println!("{:?}", bias_matrix[bias_reverse - i]);
    }


    let iteration_reverse: usize = network_output.len() - 1;
    let iteration_network_outputs: usize = (LAYER.len() * 2) - 1;
    // pour un nombre qui est :
    //   LAYER.len() = le nombre de couches dans le réseau
    //   LAYER.len() *  2 = multiplier par le nb de données enregistrées sum_bias et activ_fun
    //   (LAYER.len() * 2) - 1 = moins la donnée n'existants pas
    //                           à la couche input

     
        /////////////////////// BACKPROPAGATION //////////////////////////

    let try_number: usize = 1000;

    let mut weights_l0_find: Vec<bool> = vec![false; weights_tensor[0].len()];
    let mut weights_l1_find: Vec<bool> = vec![false; weights_tensor[1].len()];
    let mut weights_find: [Vec<bool>; 2] = [weights_l0_find, weights_l1_find];
    //let mut weights_find: Vec<Vec<f64>> = Vec::new();

    let mut bias_l0_find: Vec<bool> = vec![false; bias_matrix[0].len()];
    let mut b3_l1_find: Vec<bool> = vec![false; bias_matrix[1].len()];
    let mut bias_find: [Vec<bool>; 2] = [bias_l0_find, b3_l1_find];
        
    let precision_success: f64 = 0.001;

    let mut step_size: f64;

    let learning_rate_weights: f64 = 0.01;
    let learning_rate_bias: f64 = 0.1;

    let mut sum_derivative_square_residual: f64;
    let mut derivative_square_residual: f64;

    let mut derivative_activ_fun: f64;



    for i in 0..= try_number - 1 {
        // pour chaque essayes
        println!("\n\nLes poids à l'enver :");
        let weights_reverse: usize = weights_tensor.len() - 1;
        for j in 0..= weights_tensor.len() - 1 {
            // pour chaque couche de poids
            for k in 0..= weights_tensor[weights_reverse - j].len() - 1 {
                // pour chaque poids
                println!("Le poid numéro {} de la couche {} :", k, weights_reverse - j);
                println!("{:?}", weights_tensor[weights_reverse - j][k]);

                for y in 0..= OBSERVED_EFFECT.len() - 1 { // fait 3 itération
                    // I will need many vectors for the network :
                    // - one for the sum_bias
                    // - one for the outputs of each neurons (including the input neuron)
                    predicted_effect = (bias_matrix[bias_reverse - 1][0] * weights_tensor[weights_reverse - 0][0]) + (bias_matrix[bias_reverse - 1][1] * weights_tensor[weights_reverse - 0][1]) + bias_matrix[bias_reverse - 0][0];
                    
                    if z == 0 {
                        derivative_square_residual = (-2.0 * &vec_l1[j]) * (OBSERVED_EFFECT[y] - predicted_effect);
                        sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
                    };

                    if z == 1 {
                        if weights_tensor[z][j] > 0.0 {
                            derivative_activ_fun = 1.0;
                        } else {
                            derivative_activ_fun = 0.0;
                        };
                        derivative_square_residual = DOSAGE[j] * derivative_activ_fun * &weights_tensor[1][j] * -2.0 * (OBSERVED_EFFECT[y] - &network_output[y]);
                        sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
                    };
                }
            }
        }

        println!("\n\nLes biais à l'enver :");
        let bias_reverse: usize = bias_matrix.len() - 1;
        for j in 0..= bias_matrix.len() - 1 {
            // pour chaque couche de poids
            for k in 0..= bias_matrix[bias_reverse - j].len() - 1 {
                // pour chaque poids
                println!("Le biai numéro {} de la couche {} :", k, bias_reverse - j);
                println!("{:?}", bias_matrix[bias_reverse - j][k]);
            }
        }
    }
}