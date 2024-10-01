use inverse_conway::Conway;

pub fn parse(s: &str) -> Option<Conway> {
    let lines = s.lines().map(|x| x.trim()).filter(|x| !x.is_empty()).collect::<Vec<_>>();
    let (rows, cols) = (lines.len(), lines[0].chars().count());
    if lines.iter().any(|x| x.chars().count() != cols) {
        return None;
    }

    let mut res = Conway::new(rows, cols);
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            res.set(row, col, match ch {
                '■' => true,
                '□' => false,
                _ => return None,
            }).unwrap();
        }
    }
    Some(res)
}

#[test]
fn test_new() {
    assert_eq!(Conway::new(0, 4), Conway::new(0, 0));
    assert_eq!(Conway::new(4, 0), Conway::new(0, 0));
}

#[test]
fn test_block() {
    let mut start = Conway::new(4, 4);
    start.set(1, 1, true).unwrap();
    start.set(1, 2, true).unwrap();
    start.set(2, 1, true).unwrap();
    start.set(2, 2, true).unwrap();
    let next_1 = start.forward(1);
    let next_2 = next_1.forward(1);
    assert_eq!(start, next_1);
    assert_eq!(start, next_2);
}

#[test]
fn test_beehive() {
    let mut start = Conway::new(5, 6);
    start.set(1, 2, true).unwrap();
    start.set(1, 3, true).unwrap();
    start.set(2, 1, true).unwrap();
    start.set(2, 4, true).unwrap();
    start.set(3, 2, true).unwrap();
    start.set(3, 3, true).unwrap();
    let next_1 = start.forward(1);
    let next_2 = next_1.forward(1);
    assert_eq!(start, next_1);
    assert_eq!(start, next_2);
}

#[test]
fn test_loaf() {
    let mut start = Conway::new(6, 6);
    start.set(1, 2, true).unwrap();
    start.set(1, 3, true).unwrap();
    start.set(2, 1, true).unwrap();
    start.set(2, 4, true).unwrap();
    start.set(3, 2, true).unwrap();
    start.set(3, 4, true).unwrap();
    start.set(4, 3, true).unwrap();
    let next_1 = start.forward(1);
    let next_2 = next_1.forward(1);
    assert_eq!(start, next_1);
    assert_eq!(start, next_2);
}

#[test]
fn test_boat() {
    let mut start = Conway::new(5, 5);
    start.set(1, 1, true).unwrap();
    start.set(1, 2, true).unwrap();
    start.set(2, 1, true).unwrap();
    start.set(2, 3, true).unwrap();
    start.set(3, 2, true).unwrap();
    let next_1 = start.forward(1);
    let next_2 = next_1.forward(1);
    assert_eq!(start, next_1);
    assert_eq!(start, next_2);
}

#[test]
fn test_tub() {
    let mut start = Conway::new(5, 5);
    start.set(1, 2, true).unwrap();
    start.set(2, 1, true).unwrap();
    start.set(2, 3, true).unwrap();
    start.set(3, 2, true).unwrap();
    let next_1 = start.forward(1);
    let next_2 = next_1.forward(1);
    assert_eq!(start, next_1);
    assert_eq!(start, next_2);
}

#[test]
fn test_blinker() {
    let mut start = Conway::new(5, 5);
    start.set(1, 2, true).unwrap();
    start.set(2, 2, true).unwrap();
    start.set(3, 2, true).unwrap();
    let mut next_1 = Conway::new(5, 5);
    next_1.set(2, 1, true).unwrap();
    next_1.set(2, 2, true).unwrap();
    next_1.set(2, 3, true).unwrap();
    assert_eq!(start.forward(1), next_1);
    assert_eq!(next_1.forward(1), start);
    assert_eq!(start.forward(2), next_1.forward(1));
    assert_eq!(start.forward(4), next_1.forward(1));

    assert_eq!(next_1.backward(0).unwrap(), next_1);
    let prev_1 = next_1.backward(1).unwrap();
    assert_eq!(prev_1.forward(1), next_1);
    let prev_2 = next_1.backward(2).unwrap();
    assert_eq!(prev_2.forward(2), next_1);
}

#[test]
fn test_toad() {
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
    assert_eq!(start.forward(1), next_1);
    assert_eq!(next_1.forward(1), start);
    assert_eq!(start.forward(2), next_1.forward(1));
    assert_eq!(start.forward(4), next_1.forward(1));

    assert_eq!(next_1.backward(0).unwrap(), next_1);
    let prev_1 = next_1.backward(1).unwrap();
    assert_eq!(prev_1.forward(1), next_1);
    let prev_2 = next_1.backward(2).unwrap();
    assert_eq!(prev_2.forward(2), next_1);
}

#[test]
fn test_beacon() {
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
    assert_eq!(start.forward(1), next_1);
    assert_eq!(next_1.forward(1), start);
    assert_eq!(start.forward(2), next_1.forward(1));
    assert_eq!(start.forward(4), next_1.forward(1));

    assert_eq!(next_1.backward(0).unwrap(), next_1);
    let prev_1 = next_1.backward(1).unwrap();
    assert_eq!(prev_1.forward(1), next_1);
    let prev_2 = next_1.backward(2).unwrap();
    assert_eq!(prev_2.forward(2), next_1);
}

#[test]
fn test_big() {
    let start = parse(r#"
□□□■■□□■■□□■■□□□
□□■□■□□■■□□■□■□□
□■□□□□□□□□□□□□■□
■□□□□□□□□□□□□□□■
■■□□■■■□□□□□□□■■
□□□□□□□□□■□□□□□□
□□□□□□□□□■□□□□□□
□□□□□□□□□■□□□□□□
■■□□■■■□□□□□□□■■
■□□□□□□□□□□□□□□■
□■□□□□□□□□□□□□■□
□□■□■□□■■□□■□■□□
□□□■■□□■■□□■■□□□
"#).unwrap();
    let next_1 = parse(r#"
□□□■■□□■■□□■■□□□
□□■□■□□■■□□■□■□□
□■□□□□□□□□□□□□■□
■□□□□■□□□□□□□□□■
■■□□□■□□□□□□□□■■
□□□□□■□□□□□□□□□□
□□□□□□□□■■■□□□□□
□□□□□■□□□□□□□□□□
■■□□□■□□□□□□□□■■
■□□□□■□□□□□□□□□■
□■□□□□□□□□□□□□■□
□□■□■□□■■□□■□■□□
□□□■■□□■■□□■■□□□
"#).unwrap();
    assert_eq!(start.forward(1), next_1);
    assert_eq!(next_1.forward(1), start);

    assert_eq!(start.backward(0).unwrap(), start);
    let prev_1 = start.backward(1).unwrap();
    assert_eq!(prev_1.forward(1), start);
    let prev_2 = start.backward(2).unwrap();
    assert_eq!(prev_2.forward(2), start);
}
