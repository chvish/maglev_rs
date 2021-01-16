# Maglev Hashing Algorithm: An implementation in Rust

An implementation of the Maglev consistent hashing algorithm as described in
the Google [paper](https://static.googleusercontent.com/media/research.google.com/en//pubs/archive/44824.pdf)


## What is consistent hashing?
Hashing in general could be loosely defined as assigning a set of keys to a set
of buckets. In general, hashing algorithms aim to ensure that all buckets are
assigned an equal number of keys, and that the the keys are spread evenly
amongst the buckets.

The guarantees of a _traditional_ hashing algorithm are not enough when trying
to use hashing to assign keys amongst a set of distributed servers as
buckets. The issues being that resizing buckets can heavily rearrange the
assignment of keys. In a distributed system, since the buckets are servers,
which are not-unlikely to go down, using traditional hashing in a fault
tolerant system would mean a lot of network calls to re-arrange buckets and
bring the distributed systems to a stable state.

Consistent hashing algorithms are designed to ensure minimal distruption in
the distribution of keys on resizing buckets. To quote [wikipedia](https://en.wikipedia.org/wiki/Consistent_hashing): 
> ..when a hash table is resized, only n/m keys need to be remapped on average where nis the number of keys and m is the number of slots

## How does Maglev hashing work?
For each bucket(server) we generate a preference list for keys. The preference
list is essentially just a random permutation of array (0..M -1) for M keys. 

This preference list is then used to assign a bucket to each key. The assigment
loops through each preference list, picking up a key that hasn't been assigned
yet, and assigns it to the owner of the current preference list.
