use std::env;
use std::collections::HashMap;

fn main() {
    let input: Vec<String> = env::args().skip(1).collect();
    
    let alfabet = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','x'];
    
    let mut map: HashMap<usize,Vec<String>> = HashMap::new();
    
    for i in input {
        // find index of entry in alfabet
        let index = alfabet.iter().enumerate().find(|&r| r.1 == &i.chars().next().expect("empty string")).expect("invalid string").0;
        
        // add that entry to hashmap
        let v = map.entry(index).or_insert(Vec::new());
        v.push(i);
    };

    // print all entrys in order
    for i in alfabet.iter().enumerate() {
        match map.get(&i.0) {
            Some(x) => {
                for e in x {
                    println!("{:?}", e)
                }
            }
            None => {}
        }
    }
}
