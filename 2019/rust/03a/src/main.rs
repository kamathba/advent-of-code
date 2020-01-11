/**
    --- Day 3: Crossed Wires ---
    The gravity assist was successful, and you're well on your way to the Venus refuelling station. During the rush back on Earth, the fuel management system wasn't completely installed, so that's next on the priority list.

    Opening the front panel reveals a jumble of wires. Specifically, two wires are connected to a central port and extend outward on a grid. You trace the path each wire takes as it leaves the central port, one wire per line of text (your puzzle input).

    The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit, you need to find the intersection point closest to the central port. Because the wires are on a grid, use the Manhattan distance for this measurement. While the wires do technically cross right at the central port where they both start, this point does not count, nor does a wire count as crossing with itself.

    For example, if the first wire's path is R8,U5,L5,D3, then starting from the central port (o), it goes right 8, up 5, left 5, and finally down 3:

    ...........
    ...........
    ...........
    ....+----+.
    ....|....|.
    ....|....|.
    ....|....|.
    .........|.
    .o-------+.
    ...........
    Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down 4, and left 4:

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
    These wires cross at two locations (marked X), but the lower-left one is closer to the central port: its distance is 3 + 3 = 6.

    Here are a few more examples:

    R75,D30,R83,U83,L12,D49,R71,U7,L72
    U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
    R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
    U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135
    What is the Manhattan distance from the central port to the closest intersection?
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
    vertical: bool,
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
        let (next, is_vertical) = match movement {
            Movement::Up(spaces) => (
                Point {
                    x: current.x,
                    y: current.y + *spaces,
                },
                true,
            ),
            Movement::Down(spaces) => (
                Point {
                    x: current.x,
                    y: current.y - *spaces,
                },
                true,
            ),
            Movement::Left(spaces) => (
                Point {
                    x: current.x - *spaces,
                    y: current.y,
                },
                false,
            ),
            Movement::Right(spaces) => (
                Point {
                    x: current.x + *spaces,
                    y: current.y,
                },
                false,
            ),
        };

        ret.push(Segment {
            p1: current,
            p2: next,
            vertical: is_vertical,
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
            if outer.vertical == inner.vertical {
                continue;
            }

            let (x, y) = if outer.vertical {
                (outer.p1.x, inner.p1.y)
            } else {
                (inner.p1.x, outer.p1.y)
            };

            if outer.vertical {
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
        .map(|point| point.x.abs() + point.y.abs())
        .min()
        .expect("Unexpected point");

    println!("Distance: {:?}", distance);

    Ok(())
}
