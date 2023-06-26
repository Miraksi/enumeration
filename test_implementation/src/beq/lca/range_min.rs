fn log_floor(x: u32) -> u32 {
    return u32::BITS - x.leading_zeros() - 1;
}
// maybe add lifetimes
pub struct RMQ {
    input: Vec<u32>,
    n: usize, 
    k: usize,
    block_min: Vec<u32>,
    block_min_idx: Vec<usize>,
    sparse_idx: Vec<Vec<usize>>,
    block_rmq: Vec<Vec<Vec<usize>>>,
    block_mask: Vec<u32>,
}
// TODO change other constuctors
impl RMQ {
    pub fn new(input: Vec<u32>) -> Self {
        let n = input.len() as u32;
        let k = log_floor(n)/ 2;        
        let mut new = Self {
            input: input,
            n: n as usize,
            k: k as usize,
            block_min: Vec::new(),
            block_min_idx: Vec::new(),
            sparse_idx: vec![Vec::new();log_floor(n) as usize], // is a sparse table, which only stores the indeces
            block_rmq: Vec::new(),
            block_mask: Vec::new(),
        };
        new.calc_block_min();
        new.build_sparse();
        new.fill_block_rmq();
        new.precompute_masks();
        // println!("RMQ-DATA-STRUCTURE---------\ninput: {:?}\nsparse-idx: {:?}\n-------------------", new.input, new.sparse_idx);
        return new;
    }
    fn calc_block_min(&mut self) {
        for i in 0..(self.n + self.k -1) / self.k {
            let (min, min_idx) = self.calc_min(i*self.k);
            self.block_min.push(min);
            self.block_min_idx.push(min_idx)
        }
    }

    fn calc_min(&mut self, i: usize) -> (u32, usize) {
        let mut current_min = u32::MAX;
        let mut min_idx: usize = i;
        for j in i..i + self.k {
            match self.input.get(j) {
                Some(x) => {
                    current_min = min(current_min, *x);
                    min_idx = self.min_idx(min_idx, j);
                },
                None => break,
            };
        }
        return (current_min, min_idx);
    }

    fn build_sparse(&mut self) {
        for x in self.block_min_idx.iter() {
            self.sparse_idx[0].push(*x);
        }
        let m = self.block_min_idx.len();
        for loglen in 1..=(log_floor(m as u32) as usize) {
            for i in 0..= m - (1 << loglen) {
                let a = self.sparse_idx[loglen-1][i];
                let b = self.sparse_idx[loglen-1][i + (1 << (loglen - 1))];
                let tmp = self.min_idx(a,b);
                self.sparse_idx[loglen].push(tmp);
            }
        }
    }

    pub fn get(&self, mut l: usize, mut r: usize) -> usize { 
        // println!("RMQ({},{})", l, r);
        if l > r {
            let tmp = l;
            l = r;
            r = tmp;
        }

        let block_l = l/self.k ;
        let block_r = r/self.k ;
        let l_suffix = self.get_in_block(block_l, l % self.k, self.k - 1);
        let r_prefix = self.get_in_block(block_r, 0, r % self.k);
        match block_r - block_l {
            0 => return self.get_in_block(block_l, l % self.k, r % self.k),
            1 => return self.min_idx(l_suffix, r_prefix),
            _ => return self.min_idx(self.min_idx(l_suffix, self.get_on_blocks(block_l+1, block_r-1)), r_prefix),
        };
    }

    fn get_on_blocks(&self, l: usize, r: usize) -> usize {
        let loglen = log_floor((r-l+1) as u32) as usize;
        let idx: usize = ((r as i64) - (1 << loglen as i64) + 1) as usize;
        let a = self.sparse_idx[loglen][l as usize];
        let b = self.sparse_idx[loglen][idx];
        return self.min_idx(a,b);
    }

    fn get_in_block(&self, block_idx: usize, l: usize, r: usize) -> usize {  
        let mask = self.block_mask[block_idx];
        let min_idx = self.block_rmq[mask as usize][l][r];
        return min_idx + block_idx * self.k;
    }

    fn fill_block_rmq(&mut self) {
        let mask_amount = 1 << (self.k - 1);
        for mask in 0..mask_amount {
            let tmp = self.rmq_bitmask(mask as u32); // maybe change to usize
            self.block_rmq.push(tmp);
        }
    }

    fn rmq_bitmask(&mut self, mask: u32) -> Vec<Vec<usize>> {  
        let mut rmq_matrix: Vec<Vec<usize>> = vec![vec![0;self.k]; self.k];
        let list = bitmask_to_array(self.k, mask);
        for i in 0..self.k {
            for j in i..self.k {
                if i == j {
                    rmq_matrix[i][j] = i;
                }
                else {
                    let min = list[rmq_matrix[i][j-1]];     //Do we want range-minimum or range-maximum
                    if list[j] < min {
                        rmq_matrix[i][j] = j;
                    }
                    else {
                        rmq_matrix[i][j] = rmq_matrix[i][j-1];
                    }
                }
            }
        }
        return rmq_matrix;
    }

    fn precompute_masks(&mut self) {
        for i in 0..self.block_min.len() {
            self.block_mask.push(self.calc_bitmask(i));
        }
    }

    // we initialize the mask with k-1 ones
    // this is necessary so if blocks are of size < k the bitmask is still correct
    fn calc_bitmask(&self, block_idx: usize) -> u32{
        let mut mask: u32 = (1 << (self.k - 1)) - 1;  
        for i in self.k*block_idx + 1..self.k * (block_idx + 1) {
            let last = self.input[i-1];
            match self.input.get(i) {
                Some(&x) => {
                    if last > x {
                        mask -= 1 << (self.k-1-(i % self.k));
                    }
                },
                None => break,
            };
        }                                
        return mask;
    }
    fn min_idx(&self, i: usize, j: usize) -> usize {
        if self.input[i] < self.input[j] {
            return i;
        }
        return j;
    }
}

fn min(a: u32, b: u32) -> u32 {
    match a < b {
        true => a,
        _ => b,
    } 
}

fn bitmask_to_array(k: usize, mut mask: u32) -> Vec<i32> {
    let mut list: Vec<i32> = vec![0];
    for i in 0..k-1{
        match mask % 2 {
            1 => list.push(list[i] - 1),
            _ => list.push(list[i] + 1),
        };
        mask /= 2;
    }
    list.reverse();
    return list;
}