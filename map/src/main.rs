use std::env;
use std::collections::HashMap;

// map sort
// can sort intigers in O(n)
fn main() {
    let input: Vec<i32> = 
        env::args()
        .skip(1)
        .map(|x| x.parse::<i32>()
        .expect("invalid int"))
        .collect();

    let mut map: HashMap<i32,Vec<i32>> = HashMap::new();
    
    let (mut min, mut max) =  (i32::MAX,i32::MIN);
    
    // add all items to map
    for i in input {        
        let v = map.entry(i).or_insert(Vec::new());
        v.push(i);
        
        if i > max {
            max = i;
        }
        
        if i < min {
            min = i;
        }
        if i == i32::MAX || i == i32::MIN {
            panic!("NICE TRY")
        }
    };
    
    // if list is empty refuse to sort
    if (min,max) != (i32::MAX,i32::MIN) {
        for i in min..=max {
            match map.get(&i) {
                Some(x) => for e in x {
                    println!("{}",e);
                },
                None => ()
            }
        }
    }
}
