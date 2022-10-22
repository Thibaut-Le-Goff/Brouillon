mod runst;
//from : https://www.youtube.com/watch?v=GKZoOHXGcLo&t=614s

fn main() {
    ////////////////////////////// Data set ///////////////////////
    const DOSAGE: [f64; 3] = [0.0, 0.5, 1.0]; // ce qui est donné au réseau
    const OBSERVED_EFFECT: [f64; 3] = [0.0, 1.0, 0.0]; // ce qui est attendu qu'il donne



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
    let mut bias_l1: Vec<f64> = vec![0.0; LAYER[1]];
    println!("Les biais de la couche 1 sont :");
    println!("{:?}\n", bias_l1);

    let mut bias_l2: Vec<f64> = vec![0.0; LAYER[2]];
    println!("Les biais de la couche 2 sont :");
    println!("{:?}\n", bias_l2);

    // Creation of the matrix of bias :
    let mut bias_matrix: [Vec<f64>; 2] = [bias_l1, bias_l2];

    // Creation of the tensor of the weights and bias :
    //let mut weights_bias_tensor: [[Vec<f64>; 2]; 2] = [weights_tensor, bias_matrix];

    // details of the structure of the network
    let mut vec_input: Vec<f64> = vec![0.0; LAYER[0]];

    let mut sum_l1: Vec<f64> = vec![0.0; LAYER[1]];
    let mut sum_l1_bias: Vec<f64> = vec![0.0; LAYER[1]];
    let mut vec_l1: Vec<f64> = vec![0.0; LAYER[1]];

    let mut sum_l2: Vec<f64> = vec![0.0; LAYER[2]];
    let mut sum_l2_bias: Vec<f64> = vec![0.0; LAYER[2]];
    let mut vec_l2: Vec<f64> = vec![0.0; LAYER[2]];

    // the variables where will be stored the outputs of the network (the desired effects) :
    // I will have to register :
    //    - the weights
    //    - the sum_lx_bias for the derivative of the activation function
    //    - the vec_lx 
    

    let mut network_output: Vec<f64> = vec![0.0; DOSAGE.len()]; // ce que le réseau donne



    println!("Les poids et les biais avant la propagations :");
    println!("Les poids : {:?}\n", weights_tensor);
    println!("Les biais : {:?}\n", bias_matrix);

    ////////////////////// PROPAGATION ////////////////////////////////////
    for i in 0..= DOSAGE.len() - 1 {
        println!("Propagation des données d'entrée :");

        // Input layer:
        println!("La couches des entrées, la numéros 0 a pour valeurs :");
        vec_input[0] = DOSAGE[i];
        println!("{:?}\n", vec_input);

        println!("Dans les neurones de la couche 0(input) à 1 :");
        sum_l1 = runst::multiply(&weights_tensor[0], &vec_input);
        println!("Après La multiplication :");
        println!("{:?}\n", sum_l1);
        sum_l1_bias = runst::bias_addition(&sum_l1, &bias_matrix[0]);
        println!("Après l'ajout des biais :");
        println!("{:?}\n", sum_l1_bias);
        vec_l1 = runst::activ_fun::relu(&sum_l1_bias);
        println!("Après le passage dans la function d'activation :");
        println!("{:?}\n", vec_l1);

        println!("Dans les neurones de la couche 1 à 2 :");
        sum_l2 = runst::multiply(&weights_tensor[1], &vec_l1);
        println!("Après La multiplication :");
        println!("{:?}\n", sum_l2);
        sum_l2_bias = runst::bias_addition(&sum_l2, &bias_matrix[1]);
        println!("Après l'ajout des biais :");
        println!("{:?}\n", sum_l2_bias);
        vec_l2 = runst::activ_fun::sigmoid(&sum_l2_bias);
        println!("Après le passage dans la function d'activation :");
        println!("{:?}\n", vec_l2);

        println!("\n\nCe que le réseaux me donne :");
        println!("{:?}\n", vec_l2);

        println!("Enregistrement de l'output :");
        //observed_effect[i] = sum_l1_bias[0];
        network_output[i] = vec_l2[0];
    }

    println!("Les poids et les biais après la propagations :");
    println!("Les poids : {:?}\n", weights_tensor);
    println!("Les biais : {:?}\n", bias_matrix);

 
        /////////////////////// BACKPROPAGATION //////////////////////////

        /* 
        const DOSAGE: [f64; 3] = [0.0, 0.5, 1.0]; // ce qui est donné au réseau
        let mut observed_effect: Vec<f64> = vec![0.0; 3]; // ce que le réseau donne

        const network_output: [f64; 3] = [0.0, 1.0, 0.0]; // ce qui est attendu qu'il donne
        */

    let try_number: usize = 1000;

    let mut weights_l0_find: Vec<bool> = vec![false; weights_tensor[0].len()];
    let mut weights_l1_find: Vec<bool> = vec![false; weights_tensor[1].len()];
    let mut weights_find: [Vec<bool>; 2] = [weights_l0_find, weights_l1_find];

    let mut bias_l0_find: Vec<bool> = vec![false; bias_matrix[0].len()];
    let mut b3_l1_find: Vec<bool> = vec![false; bias_matrix[1].len()];
    let mut bias_find: [Vec<bool>; 2] = [bias_l0_find, b3_l1_find];

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

    let learning_rate_weights: f64 = 0.1;
    let learning_rate_bias: f64 = 0.1;

    let mut sum_derivative_square_residual: f64;
    let mut derivative_square_residual: f64;

    let mut derivative_activ_fun: f64;
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

    for _i in 0..= try_number - 1 {
        // pour le nombre d'essayes indiqué

        // for each layers of weight
        for z in 0..= weights_tensor.len() - 1 {
            // for eache weights of the layer z
            for j in 0..= weights_tensor[z].len() - 1 {
                if weights_find[z][j] == false {
                        // met le "compteur" de la somme a zero
                    sum_derivative_square_residual = 0.0;
        
                        // calcule d ssr
                    for y in 0..= network_output.len() - 1 {
                        //network_output[j] = (&vec_l1[0] * &weights_tensor[1][0]) + (&vec_l1[1] * &weights_tensor[1][1]) + &bias_matrix[1][0];
                        
                        if z == 1 {
                            derivative_square_residual = (-2.0 * &vec_l1[j]) * (OBSERVED_EFFECT[y] - &network_output[y]);
                            sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
                        };

                        if z == 0 {
                            if weights_tensor[z][j] > 0.0 {
                                derivative_activ_fun = 1.0;
                            } else {
                                derivative_activ_fun = 0.0;
                            };

                            derivative_square_residual = DOSAGE[j] * derivative_activ_fun * &weights_tensor[1][j] * -2.0 * (OBSERVED_EFFECT[y] - &network_output[y]);
                            sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
                        };

                        //sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;

                    }
                    //println!("\nLa somme des dérivées pour le calcule du poid numéro {:?} : {:?}",j, sum_derivative_square_residual);
            
                        // calcule step size, le pas
                    step_size = sum_derivative_square_residual * learning_rate_weights;
                    //println!("Le step_size pour le calcule du poid numéro {:?} : {:?}", j, step_size);
            
                        // determination de la prochaine valeur 
                    weights_tensor[z][j] = weights_tensor[z][j] - step_size;

                    //println!("Le poid numéro {:?} de la couche 1 est {:?}", j, weights_tensor[1][j]);

                    if sum_derivative_square_residual <= precision_success && sum_derivative_square_residual >= -precision_success {
                        //if step_size <= step_size_stop && step_size >= -step_size_stop {
                        println!("\n\nfini de trouver le bon poid numéro {:?} de la couche {:?} !", j, z);
                        weights_find[z][j] = true;
                        println!("Le poid : {:?}", weights_tensor[z][j]);
                    }
                }
            }
        }

        // for eachs layers of bias
        for z in 0..= bias_matrix.len() - 1 {
            // for each bias
            for j in 0..= bias_matrix[z].len() - 1 {
                if bias_find[z][j] == false {
                    // met le "compteur" de la somme a zero
                    sum_derivative_square_residual = 0.0;
        
                    // calcule d ssr
                    for y in 0..= network_output.len() - 1 {
                        //network_output[j] = (&vec_l1[0] * &weights_tensor[1][0]) + (&vec_l1[1] * &weights_tensor[1][1]) + &bias_matrix[1][0];

                        if z == 1 {
                            derivative_square_residual = -2.0 * (OBSERVED_EFFECT[y] - &network_output[y]);
                            sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
                        };

                        if z == 0 {
                            if weights_tensor[z][j] > 0.0 {
                                derivative_activ_fun = 1.0;
                            } else {
                                derivative_activ_fun = 0.0;
                            };
                        
                            derivative_square_residual = derivative_activ_fun * &weights_tensor[1][j] * -2.0 * (OBSERVED_EFFECT[y] - &network_output[y]);
                            sum_derivative_square_residual = derivative_square_residual + sum_derivative_square_residual;
                        };

                    }
                    //println!("\nLa somme des dérivées pour le calcule du poid numéro {:?} : {:?}",j, sum_derivative_square_residual);
            
                        // calcule step size, le pas
                    step_size = sum_derivative_square_residual * learning_rate_bias;
                    //println!("Le step_size pour le calcule du poid numéro {:?} : {:?}", j, step_size);
            
                        // determination de la prochaine valeur
                    bias_matrix[z][j] = bias_matrix[z][j] - step_size;

                    //println!("Le poid numéro {:?} de la couche 1 est {:?}", j, weights_tensor[1][j]);

                    if sum_derivative_square_residual <= precision_success && sum_derivative_square_residual >= -precision_success {
                        //if step_size <= step_size_stop && step_size >= -step_size_stop {
                        println!("\n\nfini de trouver le bon biai numéro {:?} de la couche {:?} !", j, z);
                        bias_find[z][j] = true;
                        println!("Le biai : {:?}", bias_matrix[z][j]);
                    }
                }
            }
        }
    }

    println!("Les poids et les biais après la retropropagations :");
    println!("Les poids : {:?}\n", weights_tensor);
    println!("Les biais : {:?}\n", bias_matrix);
}