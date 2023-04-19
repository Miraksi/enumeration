use std::collections::{HashSet, HashMap};

struct Automaton {
    q_current: usize,
    q_final: HashSet<usize>,
    delta: Vec<HashMap<char, usize>>,
}

impl Automaton {
    fn new(q_final: HashSet<usize>, delta: Vec<HashMap<char, usize>>) -> Self {
        Self{q_current: 0, q_final: q_final, delta: delta}
    }

    fn transition(&mut self, a: char) -> Result<bool, String>{
        if !self.delta[self.q_current].contains_key(&a) {
            return Err(format!("There is no transition with a = {} from q = {}",a , self.q_current));
        }
        self.q_current = self.delta[self.q_current][&a];
        return Ok(true);
    }

    fn contains(&mut self, w: &str) -> bool {
        self.q_current = 0;
        for a in w.chars() {
            match self.transition(a) {
                Ok(_) => {},
                Err(e) => {
                    println!("{e}");
                    return false;
                },
            };
        }
        if !self.q_final.contains(&self.q_current) {
            return false;
        }
        return true;
    }
}
fn main() {
    let string = "Hello World";
    let mut v: Vec<HashMap<char, usize>> = Vec::new();
    let mut tmp = HashMap::new();
    tmp.insert('H', 1);
    v.push(tmp);
    tmp = HashMap::new();
    tmp.insert('e', 2);
    v.push(tmp);
    tmp = HashMap::new();
    tmp.insert('l', 3);
    v.push(tmp);
    tmp = HashMap::new();
    tmp.insert('l', 4);
    v.push(tmp);
    tmp = HashMap::new();
    tmp.insert('o', 5);
    v.push(tmp);
    v.push(HashMap::new());
    
    let mut q_final: HashSet<usize> = HashSet::new();
    q_final.insert(5);
    let mut a = Automaton::new(q_final, v);
    println!("{}",a.contains("Hello"));
}