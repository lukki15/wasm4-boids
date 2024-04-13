use crate::point::Point;
use crate::wasm4::SCREEN_SIZE;

#[test]
fn add() {
    let mut a = Point{x:1.0, y:2.0};
    let b = Point{x:3.0, y:4.0};

    a.add(b);

    assert_eq!(a.x, 4.0);
    assert_eq!(a.y, 6.0);
}

#[test]
fn sub() {
    let mut a = Point{x:1.0, y:2.0};
    let b = Point{x:4.0, y:3.0};

    a.sub(b);

    assert_eq!(a.x, -3.0);
    assert_eq!(a.y, -1.0);
}

#[test]
fn div() {
    let mut a = Point{x:1.0, y:2.0};
    let n = 0.5;

    a.div(n);

    assert_eq!(a.x, 2.0);
    assert_eq!(a.y, 4.0);
}

#[test]
fn set_magniute() {
    let mut a = Point{x:3.0, y:4.0};

    a.set_magniute(10.0);

    assert_eq!(a.x, 6.0);
    assert_eq!(a.y, 8.0);
}

#[test]
fn limit() {
    let mut a = Point{x:6.0, y:8.0};

    a.limit(11.0);
    assert_eq!(a.x, 6.0);
    assert_eq!(a.y, 8.0);

    a.limit(10.0);
    assert_eq!(a.x, 6.0);
    assert_eq!(a.y, 8.0);

    a.limit(5.0);
    assert_eq!(a.x, 3.0);
    assert_eq!(a.y, 4.0);
}

#[test]
fn distance() {
    let a = Point{x:1.0, y:2.0};
    let b = Point{x:4.0, y:6.0};

    let d = Point::distance(&a, &b);

    assert_eq!(d, 5.0);
}

#[test]
fn distance_edges() {
    let screen_size: f32 = SCREEN_SIZE.into();
    let top_left = Point{x:1.0, y:1.0};
    let top_right = Point{x:screen_size -1.0, y:1.0};
    let bottom_left = Point{x:1.0, y:screen_size -1.0};
    let bottom_right = Point{x:screen_size -1.0, y:screen_size -1.0};

    // clock-wise
    assert_eq!(Point::distance(&top_left, &top_right), 2.0);
    assert_eq!(Point::distance(&top_right, &bottom_right), 2.0);
    assert_eq!(Point::distance(&bottom_right, &bottom_left), 2.0);
    assert_eq!(Point::distance(&bottom_left, &top_left), 2.0);

    // counter clock-wise
    assert_eq!(Point::distance(&top_left, &bottom_left), 2.0);
    assert_eq!(Point::distance(&bottom_left, &bottom_right), 2.0);
    assert_eq!(Point::distance(&bottom_right, &top_right), 2.0);
    assert_eq!(Point::distance(&top_right, &top_left), 2.0);

    // diagonal
    assert_eq!(Point::distance(&top_left, &bottom_right), f32::sqrt(8.0));
    assert_eq!(Point::distance(&bottom_right, &top_left), f32::sqrt(8.0));
    assert_eq!(Point::distance(&top_right, &bottom_left), f32::sqrt(8.0));
    assert_eq!(Point::distance(&bottom_left, &top_right), f32::sqrt(8.0));
}