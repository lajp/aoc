use std::io;

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    d: i64,
}

impl Sensor {
    fn new(x: i64, y: i64, px: i64, py: i64) -> Self {
        let d = (x - px).abs() + (y - py).abs();
        Self { x, y, d }
    }

    fn excluded_by_y(&self, y: i64) -> Option<(i64, i64)> {
        let d = (self.y - y).abs();

        if d > self.d {
            return None;
        }

        let diff = self.d - d;

        Some((self.x - diff, self.x + diff))
    }
}

fn main() {
    let mut sensors = Vec::new();

    let lines = io::stdin().lines();
    for line in lines.flatten() {
        let s1 = line.split(": ").collect::<Vec<&str>>();
        let sensor = s1[0].split(", ").collect::<Vec<&str>>();
        let sx = sensor[0].split('=').nth(1).unwrap().parse::<i64>().unwrap();
        let sy = sensor[1].split('=').nth(1).unwrap().parse::<i64>().unwrap();

        let beacon = s1[1].split(", ").collect::<Vec<&str>>();
        let bx = beacon[0].split('=').nth(1).unwrap().parse::<i64>().unwrap();
        let by = beacon[1].split('=').nth(1).unwrap().parse::<i64>().unwrap();

        sensors.push(Sensor::new(sx, sy, bx, by));
    }

    let mx = 4000000;

    for y in 0..mx {
        let mut excluded = Vec::new();

        for s in &sensors {
            if let Some(e) = s.excluded_by_y(y) {
                excluded.push(e);
            }
        }

        excluded.sort_by_key(|r| r.0);

        let mut prev = -1;

        for (sx, ex) in excluded {
            if prev >= mx {
                break;
            }

            if sx > prev + 1 {
                println!("{}", (prev + 1) * 4000000 + y);
                return;
            }

            prev = if prev > ex { prev } else { ex };
        }
    }
}
