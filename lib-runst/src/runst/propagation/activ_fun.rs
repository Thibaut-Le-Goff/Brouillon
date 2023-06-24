//Hidden layer activation functions:
pub fn relu(vector: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..vector.len() {
        if vector[i] > 0.0 {
            result[i] =  vector[i];
        };
    }
    result
}

pub fn leaky_relu(vector: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..vector.len() {
        if vector[i] > 0.0 {
            result[i] =  vector[i];
        } else {
            result[i] = 0.01 * vector[i];
        };
    }
    result
}

pub fn silu(vector: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..vector.len() {
        result[i] = vector[i] * (1.0 / (1.0 + (-vector[i]).exp()));
    }
    result
}

pub fn softplus(vector: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..vector.len() {
        result[i] = (1.0 + (vector[i].exp())).ln();
    }
    result
}

// Last layer activation functions:
pub fn sigmoid(vector: &Vec<f32>) -> Vec<f32> {
    
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..vector.len() {
        result[i] = 1.0 / (1.0 + (-vector[i]).exp());
    }
    result
}

pub fn none(vector: &Vec<f32>) -> Vec<f32> {
    
    return vector.to_vec();
}

pub fn softmax(vector: &Vec<f32>) -> Vec<f32> {
    /* 
    softmax calculate for each neuron, in the output layer,
    the probability that the information it indicates is the 
    right.

    ex :
    In a neural network which has to know how to differentiate
    the picture of a cat from the one of a dog, we can see 
    the results like: The picture have 70% to be one of a 
    dog and 30% of a cat.

    To calculate this function, for each neuron, we have to
    calculate the exponential of the neuron and divide it by
    the sum of the exponent of the neurons.
    */

    let mut sum: f32 = 0.0;
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..vector.len() {
        sum = sum + vector[i].exp();
    }

    for i in 0..vector.len() {
        result[i] = vector[i].exp() / sum;
    }
    result
}

/* 
//////////////// Select the activation functions wanted ///////////
use crate::runst::Network;
use crate::runst::propagation::activ_fun::*;

pub fn choose(hidden_activ_fun: &String, hidden_activ_fun: &String) -> (FunType, FunType) {

    type FunType = Box<dyn Fn(&Vec<f32>)->Vec<f32>>;

    // linking the functions(FunType) to their name(&str):
    let mut activ_fun_list: Vec<(FunType, &str)> = Vec::new();

    activ_fun_list.push((Box::new(none), "none"));
    activ_fun_list.push((Box::new(relu), "relu"));
    activ_fun_list.push((Box::new(leaky_relu), "leaky_relu"));
    activ_fun_list.push((Box::new(silu), "silu"));
    activ_fun_list.push((Box::new(softplus), "softplus"));
    activ_fun_list.push((Box::new(sigmoid), "sigmoid"));
    activ_fun_list.push((Box::new(softmax), "softmax"));

    let mut hidden_activ_fun_i: usize = activ_fun_list.len();
    let mut out_activ_fun_i: usize = activ_fun_list.len();

    for i in 0..activ_fun_list.len() {
        if activ_fun_list[i].1 == hidden_activ_fun {
            hidden_activ_fun_i = i;
        }
        // not else if because the same fun can be use in the
        // hidden and out layers
        if activ_fun_list[i].1 == out_activ_fun {
            out_activ_fun_i = i;
        }
    }

    (hidden_activ_fun_i, out_activ_fun_i)
}
*/