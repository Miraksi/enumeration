fn log_floor(x: u32) -> u32 {
    return u32::BITS - x.leading_zeros() - 1;
}

struct RMQ {
    input: Vec<u32>,
    n: u32,
    k: u32,
    block_min: Vec<u32>,
    sparse_table: Vec<Vec<u32>>,
}
// TODO change other constuctors
impl RMQ {
    fn new(input: Vec<u32>) -> Self {
        let n = input.len() as u32;
        let k = log_floor(n)/ 2;        
        let mut new = Self {
            input: input,
            n: n,
            k: k,
            block_min: Vec::new(),
            sparse_table: vec![Vec::new();log_floor(n) as usize],
        };
        new.calc_block_min();
        new.build_sparse();
        return new;
    }
    fn calc_block_min(&mut self) {
        for i in 0..(self.n + self.k -1) / self.k {
            let min = self.calc_min(i*self.k);
            self.block_min.push(min);
        }
    }

    fn calc_min(&mut self, i: u32) -> u32{
        let mut current_min = u32::MAX;
        for j in i..i + self.k {
            match self.input.get(j as usize) {
                Some(x) => current_min = min(current_min, *x),
                None => {
                    return current_min;
                }
            };
        }
        return current_min;
    }

    fn build_sparse(&mut self) {
        for x in self.block_min.iter() {
            self.sparse_table[0].push(*x);
        }
        let n = self.block_min.len();
        for loglen in 1..(log_floor(n as u32) as usize) {
            println!("length of row {}",n - (1 << loglen));
            for i in 0..=n - (1 << loglen) {
                let a = self.sparse_table[loglen-1][i];
                let b = self.sparse_table[loglen-1][i + (1 << (loglen - 1))];
                self.sparse_table[loglen].push(min(a,b));
            }
        }
    }

    fn get(&self, l: u32, r: u32) -> u32 {
        let loglen = log_floor(r-l+1) as usize;
        let idx: usize = ((r as i64) - (1 << loglen as i64) + 1) as usize;
        return min(self.sparse_table[loglen][l as usize], self.sparse_table[loglen][idx]);
    }
}


fn min(a: u32, b: u32) -> u32 {
    match a < b {
        true => a,
        _ => b,
    } 
}





fn main() { 
    let rmq = RMQ::new(vec![0,1,4,12,432,12,34,45,23,45,76,34,23,5,67,34,23,54,67,43,23,56,65,2,34,56,23]);
    println!("For k={} Blocks we get the minima={:?}",rmq.k, rmq.block_min);
    println!("sparse_table = {:?}", rmq.sparse_table);
    println!("min(0,1) = {}", rmq.get(0,1));
    println!("min(2,6) = {}", rmq.get(2,6));
}