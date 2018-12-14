extern crate num_bigint; // 0.2.1
extern crate num_traits; // 0.2.6

use core::ops::Index;
use core::ops::IndexMut;
use core::mem;

use num_bigint::BigUint;
use num_traits::One;

const CACHE_SIZE: usize = 64;

struct RotatingCache {
    cache: [BigUint; CACHE_SIZE]
}

impl RotatingCache {
    fn new() -> RotatingCache {
        unsafe {
            RotatingCache {
                cache: mem::uninitialized()
            }
        }
    }
}

impl Index<usize> for RotatingCache {
    type Output = BigUint;
    fn index(&self, index: usize) -> &BigUint {
        &self.cache[index % CACHE_SIZE]
    }
}

impl IndexMut<usize> for RotatingCache {
    fn index_mut(&mut self, index: usize) -> &mut BigUint {
        &mut self.cache[index % CACHE_SIZE]
    }
}

fn fib(index: usize) -> BigUint {
    let mut cache = RotatingCache::new();
    cache[0] = One::one();
    cache[1] = One::one();
    for i in 2..=index {
        cache[i] = cache[i - 1].clone() + cache[i - 2].clone();
    }
    cache[index].clone()
}

fn main(){
    println!("{}", fib(1_000_000_000_000));
}
