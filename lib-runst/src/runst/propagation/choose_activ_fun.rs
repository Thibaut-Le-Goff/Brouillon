/*
/////////////// Select the activation functions wanted ///////////
use crate::runst::Network;
use crate::runst::propagation::activ_fun::*;

pub fn fun(hidden_activ_fun: &String, hidden_activ_fun: &String) -> (FunType, FunType) {

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