use inverse_conway::Conway;

#[test]
fn test_forward_block() {
    let mut start = Conway::new(4, 4);
    start.set(1, 1, true).unwrap();
    start.set(1, 2, true).unwrap();
    start.set(2, 1, true).unwrap();
    start.set(2, 2, true).unwrap();
    let next_1 = start.forward();
    let next_2 = next_1.forward();
    assert_eq!(start, next_1);
    assert_eq!(start, next_2);
}

#[test]
fn test_forward_beehive() {
    let mut start = Conway::new(5, 6);
    start.set(1, 2, true).unwrap();
    start.set(1, 3, true).unwrap();
    start.set(2, 1, true).unwrap();
    start.set(2, 4, true).unwrap();
    start.set(3, 2, true).unwrap();
    start.set(3, 3, true).unwrap();
    let next_1 = start.forward();
    let next_2 = next_1.forward();
    assert_eq!(start, next_1);
    assert_eq!(start, next_2);
}

#[test]
fn test_forward_loaf() {
    let mut start = Conway::new(6, 6);
    start.set(1, 2, true).unwrap();
    start.set(1, 3, true).unwrap();
    start.set(2, 1, true).unwrap();
    start.set(2, 4, true).unwrap();
    start.set(3, 2, true).unwrap();
    start.set(3, 4, true).unwrap();
    start.set(4, 3, true).unwrap();
    let next_1 = start.forward();
    let next_2 = next_1.forward();
    assert_eq!(start, next_1);
    assert_eq!(start, next_2);
}

#[test]
fn test_forward_boat() {
    let mut start = Conway::new(5, 5);
    start.set(1, 1, true).unwrap();
    start.set(1, 2, true).unwrap();
    start.set(2, 1, true).unwrap();
    start.set(2, 3, true).unwrap();
    start.set(3, 2, true).unwrap();
    let next_1 = start.forward();
    let next_2 = next_1.forward();
    assert_eq!(start, next_1);
    assert_eq!(start, next_2);
}

#[test]
fn test_forward_tub() {
    let mut start = Conway::new(5, 5);
    start.set(1, 2, true).unwrap();
    start.set(2, 1, true).unwrap();
    start.set(2, 3, true).unwrap();
    start.set(3, 2, true).unwrap();
    let next_1 = start.forward();
    let next_2 = next_1.forward();
    assert_eq!(start, next_1);
    assert_eq!(start, next_2);
}

#[test]
fn test_forward_blinker() {
    let mut start = Conway::new(5, 5);
    start.set(1, 2, true).unwrap();
    start.set(2, 2, true).unwrap();
    start.set(3, 2, true).unwrap();
    let mut next_1 = Conway::new(5, 5);
    next_1.set(2, 1, true).unwrap();
    next_1.set(2, 2, true).unwrap();
    next_1.set(2, 3, true).unwrap();
    assert_eq!(start.forward(), next_1);
    assert_eq!(next_1.forward(), start);
}

#[test]
fn test_forward_toad() {
    let mut start = Conway::new(6, 6);
    start.set(2, 2, true).unwrap();
    start.set(2, 3, true).unwrap();
    start.set(2, 4, true).unwrap();
    start.set(3, 1, true).unwrap();
    start.set(3, 2, true).unwrap();
    start.set(3, 3, true).unwrap();
    let mut next_1 = Conway::new(6, 6);
    next_1.set(1, 3, true).unwrap();
    next_1.set(2, 4, true).unwrap();
    next_1.set(3, 4, true).unwrap();
    next_1.set(2, 1, true).unwrap();
    next_1.set(3, 1, true).unwrap();
    next_1.set(4, 2, true).unwrap();
    assert_eq!(start.forward(), next_1);
    assert_eq!(next_1.forward(), start);
}

#[test]
fn test_forward_beacon() {
    let mut start = Conway::new(6, 6);
    start.set(1, 1, true).unwrap();
    start.set(1, 2, true).unwrap();
    start.set(2, 1, true).unwrap();
    start.set(2, 2, true).unwrap();
    start.set(3, 3, true).unwrap();
    start.set(3, 4, true).unwrap();
    start.set(4, 3, true).unwrap();
    start.set(4, 4, true).unwrap();
    let mut next_1 = Conway::new(6, 6);
    next_1.set(1, 1, true).unwrap();
    next_1.set(1, 2, true).unwrap();
    next_1.set(2, 1, true).unwrap();
    next_1.set(3, 4, true).unwrap();
    next_1.set(4, 3, true).unwrap();
    next_1.set(4, 4, true).unwrap();
    assert_eq!(start.forward(), next_1);
    assert_eq!(next_1.forward(), start);
}
