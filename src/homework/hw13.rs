
struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    // Use a HashSet to store unique covered points
    use std::collections::HashSet;
    let mut covered = HashSet::new();

    for rect in xs {
        let x_min = rect.a.x.min(rect.b.x);
        let x_max = rect.a.x.max(rect.b.x);
        let y_min = rect.a.y.min(rect.b.y);
        let y_max = rect.a.y.max(rect.b.y);

        // Loop through every point inside the rectangle (non-inclusive of bottom-right edge)
        for x in x_min..x_max {
            for y in y_min..y_max {
                covered.insert((x, y));
            }
        }
    }

    covered.len() as i32
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}
#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}


/* //this is madness but it also works.
    /*
    1. find 2 or 4 points of intersection.
    2. calculate the area inside the intersections
    3. subtract the sum of intersection areas from sum of areas of rectangles.
    */
fn rect_area(rect: &Rectangle) -> i32 {
    let x1 = rect.a.x.min(rect.b.x);
    let x2 = rect.a.x.max(rect.b.x);
    let y1 = rect.a.y.min(rect.b.y);
    let y2 = rect.a.y.max(rect.b.y);
    (x2 - x1).max(0) * (y2 - y1).max(0)
}

fn intersection(r1: &Rectangle, r2: &Rectangle) -> Option<Rectangle> {
    let x1 = r1.a.x.min(r1.b.x).max(r2.a.x.min(r2.b.x));
    let x2 = r1.a.x.max(r1.b.x).min(r2.a.x.max(r2.b.x));
    let y1 = r1.a.y.min(r1.b.y).max(r2.a.y.min(r2.b.y));
    let y2 = r1.a.y.max(r1.b.y).min(r2.a.y.max(r2.b.y));

    if x1 < x2 && y1 < y2 {
        Some(Rectangle {
            a: Point { x: x1, y: y1 },
            b: Point { x: x2, y: y2 },
        })
    } else {
        None
    }
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let a1 = rect_area(&xs[0]);
    let a2 = rect_area(&xs[1]);
    let a3 = rect_area(&xs[2]);

    let i12 = intersection(&xs[0], &xs[1]).map_or(0, |r| rect_area(&r));
    let i13 = intersection(&xs[0], &xs[2]).map_or(0, |r| rect_area(&r));
    let i23 = intersection(&xs[1], &xs[2]).map_or(0, |r| rect_area(&r));

    let triple = intersection(&xs[0], &xs[1])
        .and_then(|i| intersection(&i, &xs[2]))
        .map_or(0, |r| rect_area(&r));

    a1 + a2 + a3 - i12 - i13 - i23 + triple
}
 */
