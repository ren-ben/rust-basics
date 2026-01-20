use rust_bootcamp_part1::*;

#[test]
fn furthest_generic() {
    let pts = [
        Point { x: 1.0, y: 1.0 },
        Point { x: -5.0, y: 0.0 },
        Point { x: 2.0, y: 2.0 },
    ];
    let res = furthest_from_origin(&pts).unwrap();
    assert_eq!(*res, Point { x: -5.0, y: 0.0 });

    let tuples = [(0.0, 1.0), (3.0, 4.0), (2.0, 0.0)];
    let res2 = furthest_from_origin(&tuples).unwrap();
    assert_eq!(*res2, (3.0, 4.0));
}

#[test]
fn min_by_key_test() {
    let nums = [10, 50, 2, 8, 3];
    let min = min_by_key(&nums, |x| *x);
    assert_eq!(min, Some(&2));

    let strings = ["apple", "banana", "pear", "kiwi"];
    let shortest = min_by_key(&strings, |s| s.len());
    assert_eq!(shortest, Some(&"pear"));

    let empty: &[i32] = &[];
    assert_eq!(min_by_key(empty, |x| *x), None);
}
