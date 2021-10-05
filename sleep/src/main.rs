use std::env;

fn main() {
    let input: Vec<String> = env::args().collect();
    // pharse input to numbers
    let input_n: Vec<i32> = input.iter().skip(1).map(|x| x.parse().expect("all arguments must be numbers")).collect();
    // compute maximum of input
    let max = {
        let mut max = 0_i32;
        for i in input_n.iter() {
            if max < *i {
                max = *i;
            }
        }
        max
    };
    
    for i in 0..=max {
        for e in input_n.iter() {
            if i == *e {
                println!("{}",e)
            }
        }
    }
}
