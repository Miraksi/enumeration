fn log_floor(x: u32) -> u32 {
    return u32::BITS - x.leading_zeros() - 1;
}

struct RMQ {
    input: Vec<u32>,
    n: u32, // TODO maybe change to usize
    k: usize,
    block_min: Vec<u32>,
    sparse_table: Vec<Vec<u32>>,
    block_rmq: Vec<Vec<Vec<usize>>>,
    block_mask: Vec<u32>,
}
// TODO change other constuctors
// TODO k should be usize
impl RMQ {
    fn new(input: Vec<u32>) -> Self {
        let n = input.len() as u32;
        let k = log_floor(n)/ 2;        
        let mut new = Self {
            input: input,
            n: n,
            k: k as usize,
            block_min: Vec::new(),
            sparse_table: vec![Vec::new();log_floor(n) as usize],
            block_rmq: Vec::new(),
            block_mask: Vec::new(),
        };
        new.calc_block_min();
        new.build_sparse();
        new.fill_block_rmq();
        new.precompute_masks();
        return new;
    }
    fn calc_block_min(&mut self) {
        for i in 0..(self.n as usize + self.k -1) / self.k {
            let min = self.calc_min(i*self.k);
            self.block_min.push(min);
        }
    }

    fn calc_min(&mut self, i: usize) -> u32{
        let mut current_min = u32::MAX;
        for j in i..i + self.k {
            match self.input.get(j) {
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
            for i in 0..=n - (1 << loglen) {
                let a = self.sparse_table[loglen-1][i];
                let b = self.sparse_table[loglen-1][i + (1 << (loglen - 1))];
                self.sparse_table[loglen].push(min(a,b));
            }
        }
    }

    fn get(&self, l: usize, r: usize) -> u32 {
        let block_l = l/self.k ;
        let block_r = r/self.k ;
        let l_suffix = self.get_in_block(block_l, l % self.k, self.k - 1);
        let r_prefix = self.get_in_block(block_r, 0, r % self.k);
        match block_l - block_r {
            0 => return self.get_in_block(block_l, l, r),
            1 => return min(l_suffix, r_prefix),
            _ => return min(min(l_suffix, self.get_on_blocks(l+1, r-1)), r_prefix),
        };
    }

    fn get_on_blocks(&self, l: usize, r: usize) -> u32 {
        let loglen = log_floor((r-l+1) as u32) as usize;
        let idx: usize = ((r as i64) - (1 << loglen as i64) + 1) as usize;
        return min(self.sparse_table[loglen][l as usize], self.sparse_table[loglen][idx]);
    }

    fn get_in_block(&self, block_idx: usize, l: usize, r: usize) -> u32 {  
        let mask = self.calc_bitmask(block_idx);
        let min_idx = self.block_rmq[mask as usize][l][r];
        return self.input[min_idx + block_idx * (self.k as usize)];
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

    }

    // we initialize the mask with k-1 ones
    // this is necessary so if blocks are of size < k the bitmask is still correct
    fn calc_bitmask(&self, block_idx: usize) -> u32{
        let mut mask: u32 = (1 << (self.k - 1)) - 1;  
        for i in self.k*block_idx + 1..self.k * (block_idx + 1) {
            let last = self.input[i-1];
            println!("exponent {}", self.k-(i % self.k));
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



fn main() { 
    let rmq = RMQ::new(vec![0,1,2,1,2,3,4,5,4,5,4,3,2,3,4,5,6,7,8,9,8,7,6,5,7,6,5,6,7,8,9,10,9,8,7,8,7,6,7,6,5,4,3,2,1,2,3,2,1,2,3,4,5,6,7,8,7,6,5,4,3,4,5,6,7,8,7,6,5,4,5,4,3,2,3,4]);
    println!("For k={} Blocks we get the minima={:?}",rmq.k, rmq.block_min);
    println!("sparse_table = {:?}", rmq.sparse_table);
    println!("min(2,2) = {}", rmq.get_on_blocks(2,2));
    println!("min(2,6) = {}", rmq.get_on_blocks(2,6));
    println!("{:?}", rmq.block_rmq);
    println!("calc_bitmask(2) {:?}", rmq.calc_bitmask(2));
}