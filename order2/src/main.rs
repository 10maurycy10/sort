use std::env;
use std::collections::HashMap;

// this has some ineficencys with wasted spots in arrays.
fn order(n: i32, list: Vec<i32>) -> Vec<i32> {
    // count the amount of eatch number
    let mut qnt_buffer = vec![0;(n + 1) as usize];
    for i in list.iter() {
        qnt_buffer[*i as usize] += 1;
    }
    // compute the buffer offsets
    let mut offset_buffer = vec![0;n as usize];
    for i in 1..n {
        offset_buffer[i as usize] = offset_buffer[i as usize - 1] + qnt_buffer[i as usize];
    }
    let order_buffer_size = offset_buffer[n as usize - 1];
    
    // we need to keep track of how meney of each value was aready writen
    let mut order_buffer_fillng = vec![0; n as usize];
    let mut order_buffer = vec![0; order_buffer_size];
    
    for i in list.iter() {
        let offset = offset_buffer[(i - 1) as usize];
        order_buffer[(offset + order_buffer_fillng[*i as usize]) as usize] = *i;
        order_buffer_fillng[*i as usize] += 1;
    }
    
    order_buffer
}

// order2 sort
// can order intigers in O(n)
// can handle copys of eatch int
fn main() {
    let input: Vec<i32> = 
        env::args()
        .skip(1)
        .map(|x| x.parse::<i32>()
        .expect("invalid int"))
        .collect();

    println!("{:?}",order(
        *input
            .iter()
            .next()
            .expect("you must provide N")
        ,
        input
            .iter()
            .skip(1)
            .map(|x| *x)
            .collect()
    ))
}
