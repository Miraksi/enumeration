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