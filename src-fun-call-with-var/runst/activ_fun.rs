// Hidden layer activation functions:
pub fn relu(vector: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..= vector.len() - 1 {
        if vector[i] > 0.0 {
            result[i] =  vector[i];
        };
    }
    return result;
}

pub fn leaky_relu(vector: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..= vector.len() - 1 {
        if vector[i] > 0.0 {
            result[i] =  vector[i];
        } else {
            result[i] = 0.01 * &vector[i];
        };
    }
    return result;
}

pub fn silu(vector: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..= vector.len() - 1 {
        if vector[i] > 0.0 {
            result[i] =  vector[i];
        } else {
            result[i] = 1.0 / (1.0 + (-1.0 * &vector[i]).exp());
        };
    }
    return result;
}

pub fn soft_plus(vector: &Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..= vector.len() - 1 {
        result[i] = (1.0 + (&vector[i].exp())).log(10.0);
        //result[i] = (1.0 + (&vector[i].exp())).ln();
    }
    return result;
}


// Last layer activation functions:
pub fn sigmoid(vector: &Vec<f32>) -> Vec<f32> {
    
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..= vector.len() - 1 {
        result[i] = 1.0 / (1.0 + (-1.0 * &vector[i]).exp());
    }
    return result;
}

pub fn none(vector: &Vec<f32>) -> Vec<f32> {
    
    let mut result: Vec<f32> = vec![0.0; vector.len()];

    for i in 0..= vector.len() - 1 {
        result[i] = vector[i];
    }
    return result;
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

    for i in 0..= vector.len() - 1 {
        sum = sum + &vector[i].exp();
    }

    for i in 0..= vector.len() - 1 {
        result[i] = &vector[i].exp() / sum;
    }
    return result;
}