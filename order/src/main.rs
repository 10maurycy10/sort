use std::env;
use std::collections::HashMap;

fn order(n: i32, list: Vec<i32>) -> Vec<i32> {
    let mut buffer = vec![0;n as usize];
    for i in list.iter() {
        buffer[*i as usize - 1] = *i;
    }
    let mut buffer_out = vec![];
    for i in buffer.iter() {
        if *i != 0 {
            buffer_out.push(*i);
        }
    }
    buffer_out
}

// order sort
// can order intigers in O(n)
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
