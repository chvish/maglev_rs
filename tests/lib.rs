#[cfg(test)]
use maglev_rs::Maglev;
#[test]
fn maglev_hash_requires_prime_table_size() {
    assert_eq!(
        Err("Table size should be a prime"),
        Maglev::new(&vec!["b1".to_string(), "b2".to_string()], 50)
    )
}

#[test]
fn maglev_hash_works() {
    let backends = vec!["b0", "b1", "b2"]
        .into_iter()
        .map(|s| String::from(s))
        .collect();
    let m = Maglev::new(&backends, 11).unwrap();
    assert_eq!(Some("b1".to_string()), m.get_backend(5))
}

#[test]
fn maglev_backend_update_works() {
    let backends = vec!["b0", "b1", "b2"]
        .into_iter()
        .map(|s| String::from(s))
        .collect();
    let mut m = Maglev::new(&backends, 11).unwrap();
    assert_eq!(Ok(()), m.put_backend(&String::from("b3")));
    // get backend should continue to work
    assert_eq!(Some("b2".to_string()), m.get_backend(5));
    assert_eq!(Ok(()), m.remove_backend(&String::from("b2")));
    assert_eq!(Some("b3".to_string()), m.get_backend(5))
}
