/**
    --- Part Two ---
    It turns out that this circuit is very timing-sensitive; you actually need to minimize the signal delay.

    To do this, calculate the number of steps each wire takes to reach each intersection; choose the intersection where the sum of both wires' steps is lowest. If a wire visits a position on the grid multiple times, use the steps value from the first time it visits that position when calculating the total value of a specific intersection.

    The number of steps a wire takes is the total number of grid squares the wire has entered to get to that location, including the intersection being considered. Again consider the example from above:

    ...........
    .+-----+...
    .|.....|...
    .|..+--X-+.
    .|..|..|.|.
    .|.-X--+.|.
    .|..|....|.
    .|.......|.
    .o-------+.
    ...........
    In the above example, the intersection closest to the central port is reached after 8+5+5+2 = 20 steps by the first wire and 7+6+4+3 = 20 steps by the second wire for a total of 20+20 = 40 steps.

    However, the top-right intersection is better: the first wire takes only 8+5+2 = 15 and the second wire takes only 7+6+2 = 15, a total of 15+15 = 30 steps.

    Here are the best steps for the extra examples from above:

    R75,D30,R83,U83,L12,D49,R71,U7,L72
    U62,R66,U55,R34,D71,R55,D58,R83 = 610 steps
    R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
    U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = 410 steps
    What is the fewest combined steps the wires must take to reach an intersection?
*/

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Segment {
    p1: Point,
    p2: Point,
}

impl Segment {
    fn vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }
}

#[derive(Debug)]
enum Movement {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

fn to_movement(string: &str) -> Option<Movement> {
    let spaces: i32 = string[1..].parse::<i32>().expect("Invalid Move");

    match string.chars().nth(0).unwrap() {
        'L' => Some(Movement::Left(spaces)),
        'U' => Some(Movement::Up(spaces)),
        'R' => Some(Movement::Right(spaces)),
        'D' => Some(Movement::Down(spaces)),
        _ => None,
    }
}

fn extract_movements(movements: &str) -> Vec<Movement> {
    movements
        .split(',')
        .map(|s| to_movement(s).expect("Invalid Move"))
        .collect()
}

fn segments_from_movements(moves: &[Movement], start: Point) -> Vec<Segment> {
    let mut current: Point = start;
    let mut ret = Vec::new();

    for movement in moves {
        let next = match movement {
            Movement::Up(spaces) => Point {
                x: current.x,
                y: current.y + *spaces,
            },
            Movement::Down(spaces) => Point {
                x: current.x,
                y: current.y - *spaces,
            },
            Movement::Left(spaces) => Point {
                x: current.x - *spaces,
                y: current.y,
            },
            Movement::Right(spaces) => Point {
                x: current.x + *spaces,
                y: current.y,
            },
        };

        ret.push(Segment {
            p1: current,
            p2: next,
        });
        current = next;
    }

    ret
}

fn intersects(value: i32, p1: i32, p2: i32) -> bool {
    (value <= p1 && p2 <= value) || (value >= p1 && p2 >= value)
}

fn find_intersections(segments1: &[Segment], segments2: &[Segment]) -> Vec<Point> {
    let mut ret = Vec::new();

    for outer in segments1 {
        for inner in segments2 {
            if outer.vertical() == inner.vertical() {
                continue;
            }

            let (x, y) = if outer.vertical() {
                (outer.p1.x, inner.p1.y)
            } else {
                (inner.p1.x, outer.p1.y)
            };

            if outer.vertical() {
                if intersects(x, inner.p1.x, inner.p2.x) && intersects(y, outer.p1.y, outer.p2.y) {
                    ret.push(Point { x, y });
                }
            } else if intersects(x, outer.p1.x, outer.p2.x) && intersects(y, inner.p1.y, inner.p2.y)
            {
                ret.push(Point { x, y });
            }
        }
    }

    ret
}

fn distance_to_point(segments: &[Segment], point: Point) -> Option<i32> {
    let mut sum: i32 = 0;

    for segment in segments {
        if segment.vertical() {
            if point.x == segment.p1.x && intersects(point.y, segment.p1.y, segment.p2.y) {
                return Some(sum + (point.y - segment.p1.y).abs());
            }

            sum += (segment.p1.y - segment.p2.y).abs();
        } else {
            if point.y == segment.p1.y && intersects(point.x, segment.p1.x, segment.p2.x) {
                return Some(sum + (point.x - segment.p1.x).abs());
            }

            sum += (segment.p1.x - segment.p2.x).abs();
        }
    }

    None
}

fn main() -> std::io::Result<()> {
    let (wire1_moves, wire2_moves) = {
        let mut lines = include_str!("../input").lines();
        (
            extract_movements(lines.next().expect("Missing line")),
            extract_movements(lines.next().expect("Missing line")),
        )
    };

    let origin = Point { x: 0, y: 0 };

    let wire1_segments = segments_from_movements(&wire1_moves, origin);
    let wire2_segments = segments_from_movements(&wire2_moves, origin);

    let intersections = find_intersections(&wire1_segments, &wire2_segments);

    let distance = intersections
        .iter()
        .map(|point| {
            distance_to_point(&wire1_segments, *point).unwrap()
                + distance_to_point(&wire2_segments, *point).unwrap()
        })
        .min()
        .expect("Unexpected point");

    println!("Distance: {:?}", distance);

    Ok(())
}
