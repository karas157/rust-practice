use std::collections::BTreeSet;

struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
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

fn area_occupied(rects: &Vec<Rectangle>) -> i32 {
    let mut x_coords = BTreeSet::new();

    for r in rects {
        x_coords.insert(r.a.x);
        x_coords.insert(r.b.x);
    }

    let x_sorted: Vec<i32> = x_coords.into_iter().collect();
    let mut total_area = 0;

    for i in 0..x_sorted.len() - 1 {
        let x1 = x_sorted[i];
        let x2 = x_sorted[i + 1];
        let width = x2 - x1;

        let mut y_intervals = Vec::new();

        for r in rects {
            if r.a.x <= x1 && r.b.x >= x2 {
                y_intervals.push((r.a.y, r.b.y));
            }
        }

        y_intervals.sort();
        let mut merged_intervals: Vec<(i32, i32)> = Vec::new();

        for (y1, y2) in y_intervals {
            if let Some((_last_y1, last_y2)) = merged_intervals.last_mut() {
                if *last_y2 >= y1 {
                    *last_y2 = (*last_y2).max(y2);
                } else {
                    merged_intervals.push((y1, y2));
                }
            } else {
                merged_intervals.push((y1, y2));
            }
        }

        let mut height = 0;
        for (y1, y2) in merged_intervals {
            height += y1 - y2;
        }
        total_area += width * height;
    }

    total_area
}

fn main() {
    let data = test_data();
    let occupied_area = area_occupied(&data);
    println!("Зайнята площа: {}", occupied_area);
}
