use std::collections::HashMap;

type Kysyttava = HashMap<String, String>;


fn main() {
    println!("Hello, world!");
    let mut a = Kysyttava::new(); 
    a.insert("perusmuoto".to_string(),"食べる".to_string());
}
/*
struct Kysyttava {
    pub kysyttava: HashMap<String, String>
}

impl Kysyttava {
    pub fn new() -> Kysyttava {
        Kysyttava {
            kysyttava: HashMap::new()
        }
    }
}
*/
