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

    fn point_in_range(&self, px: i64, py: i64) -> bool {
        let d = (self.x - px).abs() + (self.y - py).abs();
        d <= self.d
    }
}

fn main() {
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

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
        beacons.push((bx, by));
    }

    sensors.sort_by_key(|s| s.x);
    let sx = sensors[0].x - sensors[0].d;
    let ex = sensors.last().unwrap().x + sensors.last().unwrap().d + 1;
    let y = 2000000;
    let mut ans = 0;

    for x in sx..ex {
        for s in &sensors {
            if s.point_in_range(x, y) && !beacons.contains(&(x, y)) {
                ans += 1;
                break;
            }
        }
    }

    println!("{}", ans);
}
