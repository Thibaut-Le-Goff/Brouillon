///////////////// for the propagation ///////////////////////////
use crate::runst::calculations;
use crate::runst::Network;

pub fn propagation(net: &Network, inputs: &Vec<Vec<f32>>, weights: &Vec<Vec<f32>>, bias: &Vec<Vec<f32>>) -> Vec<f32> {
//pub fn propagation(net: &Network, inputs: &Vec<Vec<f32>>, weights: &Vec<Vec<f32>>, bias: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    
    //type FunType = Box<dyn Fn(&Vec<f32>)->Vec<f32>>;

    //let (hidden_activ_fun_i, out_activ_fun_i): (FunType, FunType) = activ_fun::choose(&net.hidden_activ_fun, &net.hidden_activ_fun);
    

    
    //////////////// Select the activation functions wanted ///////////
    type FunType = Box<dyn Fn(&Vec<f32>)->Vec<f32>>;

    // linking the functions(FunType) to their name(&str):
    let mut activ_fun_list: Vec<(FunType, &str)> = Vec::new();

    activ_fun_list.push((Box::new(activ_fun::none), "none"));
    activ_fun_list.push((Box::new(activ_fun::relu), "relu"));
    activ_fun_list.push((Box::new(activ_fun::leaky_relu), "leaky_relu"));
    activ_fun_list.push((Box::new(activ_fun::silu), "silu"));
    activ_fun_list.push((Box::new(activ_fun::softplus), "softplus"));
    activ_fun_list.push((Box::new(activ_fun::sigmoid), "sigmoid"));
    activ_fun_list.push((Box::new(activ_fun::softmax), "softmax"));

    let mut hidden_activ_fun_i: usize = activ_fun_list.len();
    let mut out_activ_fun_i: usize = activ_fun_list.len();

    for i in 0..activ_fun_list.len() {
        if activ_fun_list[i].1 == net.hidden_activ_fun {
            hidden_activ_fun_i = i;
        }
        // not else if because the same fun can be use in the
        // hidden and out layers
        if activ_fun_list[i].1 == net.out_activ_fun {
            out_activ_fun_i = i;
        }
    }
    

    /*
    Note to myself:
        send an error message if:
            hidden_activ_fun_i and/or out_activ_fun_i are still equal to 
            activ_fun_list.len() in this area
        
        Because the activation function wanted is 
        not available
    */

    /////////////// The propagation really start here //////////////
    
    let mut network_predictions: Vec<f32> = Vec::new();
    //let mut network_predictions: Vec<Vec<f32>> = Vec::new();
    
    for i in 0..inputs.len() {
        // for each pair of datas in the data set
        println!("Propagation numéro {} des données d'entrée :", i + 1);

        println!("La couches des entrées, la numéros 0 a pour valeurs :");
        let mut neuron_out_to_input: Vec<f32> = inputs[i].clone();
        println!("{:?}\n", &neuron_out_to_input);

        for y in 0..weights.len() {
            // for each layers of weights :

            println!("\nDans les neurones de la couche {} :", y + 2);
            let neuron_sum: Vec<f32> = calculations::multiplication::mat_vec(&weights[y], &neuron_out_to_input);
            println!("Après La multiplication :");
            println!("{:?}\n", &neuron_sum);
            let neuron_sum_bias: Vec<f32> = calculations::addition::vec_vec(&neuron_sum, &bias[y]);
            println!("Après l'ajout des biais :");
            println!("{:?}\n", &neuron_sum_bias);

            if y == weights.len() - 1 {
                // if this is the last layer (the last iteration)
                // last iteration is equal to net.network_struct.len() - 2
                // because :
                // - 1 : the iteration started at 0
                // - 1 : avoid the first layer exactly like above

                let neurons_outputs = activ_fun_list[out_activ_fun_i].0(&neuron_sum_bias);
                
                println!("Après le passage dans la function d'activation :");
                println!("{:?}\n", &neurons_outputs);

                //network_predictions.push(neurons_outputs);

                 
                for j in 0..neurons_outputs.len() {
                    network_predictions.push(neurons_outputs[j]);
                }
                

            } else {
                neuron_out_to_input = activ_fun_list[hidden_activ_fun_i].0(&neuron_sum_bias);

                // We need two time the same output:
                // - one for the next iteration
                // - one for the output
                let neurons_outputs: Vec<f32> = neuron_out_to_input.clone();

                println!("Après le passage dans la function d'activation :");
                println!("{:?}\n", &neuron_out_to_input);

                //network_predictions.push(neurons_outputs);
                 
                for j in 0..neurons_outputs.len() {
                    network_predictions.push(neurons_outputs[j]);
                }
                          
            }
        }
    }

    network_predictions
}

pub mod activ_fun;
//pub mod choose_activ_fun;