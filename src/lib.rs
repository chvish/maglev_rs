use std::collections::hash_map::DefaultHasher;
use std::convert::TryInto;
use std::hash::{Hash, Hasher};

///! # Maglev
///! An implementation of the maglev consistent hashing algorithm

#[derive(Debug, PartialEq)]
pub struct Maglev {
    table: Vec<u64>,
    backends: Vec<String>,
}

fn calculate_hash<T: Hash>(v: &T, hasher: &mut DefaultHasher) -> u64 {
    v.hash(hasher);
    hasher.finish()
}

impl Maglev {
    /// Creates a new hash table of table_size with given backends. The backend size 
    /// should be a prime, else returns an error
    pub fn new(backends: &Vec<String>, table_size: u64) -> Result<Maglev, &str> {
        if primes::is_prime(table_size) {
            Ok(Maglev {
                table: Maglev::populate(backends, table_size as usize),
                backends: backends.to_vec(),
            })
        } else {
            Err("Table size should be a prime")
        }
    }

    /// Get backend for a given index
    pub fn get_backend(&self, idx: usize) -> Option<String> {
        self.table
            .get(idx)
            .map(|i| String::from(&self.backends[*i as usize]))
    }


    /// Add a backend to the list. Regenerates the hashing table
    pub fn put_backend(&mut self, backend: &String) -> Result<(), &str> {
        let matching: Vec<&String> = self.backends.iter().filter(|b| b == &backend).collect();
        match matching.is_empty() {
            false => Err("Backend already exists"),
            true => {
                self.backends.push(String::from(backend));
                Ok(self.reset())
            }
        }
    }

    /// Remove a backend to the list. Regenerates the hashing table
    pub fn remove_backend(&mut self, backend: &String) -> Result<(), &str> {
        let mut idx = None;
        for (i, b) in self.backends.iter().enumerate() {
            if b == backend {
                idx = Some(i);
                break;
            }
        }
        match idx {
            None => Err("Backend not found"),
            Some(idx) => {
                self.backends.swap_remove(idx);
                Ok(self.reset())
            }
        }
    }

    fn reset(&mut self) {
        self.table = Maglev::populate(&self.backends, self.table.len())
    }

    fn populate(backends: &Vec<String>, table_size: usize) -> Vec<u64> {
        let permutations = Maglev::get_permutations(backends, table_size as u64);
        let num_backends = backends.len();
        let mut next = vec![0; num_backends];
        let mut entry = vec![-1; table_size];
        let mut n = 0;
        'outer: loop {
            for i in 0..num_backends {
                let mut c = permutations[i][next[i]];
                while entry[c as usize] >= 0 {
                    next[i] = next[i] + 1;
                    c = permutations[i][next[i]]
                }
                entry[c as usize] = i.try_into().unwrap();
                next[i] = next[i] + 1;
                n = n + 1;
                if n == table_size {
                    break 'outer;
                }
            }
        }
        entry.into_iter().map(|x| x as u64).collect()
    }

    fn get_permutations(backends: &Vec<String>, table_size: u64) -> Vec<Vec<u64>> {
        let n = backends.len();
        let mut permutations: Vec<Vec<u64>> = Vec::new();
        let (offsets, skips) = Maglev::get_offsets_and_skips(backends, table_size);
        for i in 0..n {
            let offset = offsets[i];
            let skip = skips[i];
            let permutation: Vec<u64> = (0..table_size)
                .map(|x| (offset + x * skip) % table_size)
                .collect();
            permutations.push(permutation)
        }

        permutations
    }

    fn get_offsets_and_skips(backends: &Vec<String>, table_size: u64) -> (Vec<u64>, Vec<u64>) {
        let mut hasher = DefaultHasher::new();
        let mut skips = Vec::new();
        let mut offsets = Vec::new();
        for backend in backends {
            let hash = calculate_hash(backend, &mut hasher);
            let offset = (hash << 32) % table_size;
            let skip = (hash & 0xffffffff) % (table_size - 1) + 1;
            offsets.push(offset);
            skips.push(skip);
        }
        (offsets, skips)
    }
}
