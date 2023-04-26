fn log_floor(x: u32) -> u32 {
    return u32::BITS - x.leading_zeros() - 1;
}

struct RMQ {
    input: Vec<u32>,
    n: u32,
    k: u32,
    block_min: Vec<u32>,
    sparse_table: Vec<Vec<u32>>,
    bucket_rmq: Vec<Vec<Vec<usize>>>,
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
            bucket_rmq: Vec::new(),
        };
        new.calc_block_min();
        new.build_sparse();
        new.fill_bucket_rmq();
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

    fn fill_bucket_rmq(&mut self) {
        let mask_amount = (1 << (self.k - 1)) as usize;
        for mask in 0..mask_amount {
            let tmp = self.rmq_bitmask(mask as u32); // maybe change to usize
            self.bucket_rmq.push(tmp);
        }
    }

    fn rmq_bitmask(&mut self, mask: u32) -> Vec<Vec<usize>> {  
        let k: usize = self.k as usize;
        let mut rmq_matrix: Vec<Vec<usize>> = vec![vec![0;k]; k];
        let list = bitmask_to_array(self.k as usize, mask);
        for i in 0..k {
            for j in i..k {
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
    println!("min(0,1) = {}", rmq.get(0,1));
    println!("min(2,6) = {}", rmq.get(2,6));
    println!("{:?}", rmq.bucket_rmq);
    println!("{:?}", bitmask_to_array(3,1));
}