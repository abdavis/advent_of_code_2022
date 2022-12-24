const INPUT: &str = include_str!("inputs/day15.txt");
const EXAMPLE: &str = include_str!("example_inputs/day15.txt");

pub fn run() -> String {
    let points: Points = INPUT.into();
    let tree = BiTree::new(&points, 2_000_000, None);
    let range = Range {
        min: 0,
        max: 4_000_000,
    };
    let quad_tree = QuadTree::new(&points, range, range);
    format!(
        "{}\n{}",
        tree.count_empty(),
        quad_tree.find_tuning_frequency().unwrap()
    )
}

struct QuadTree {
    x_range: Range,
    y_range: Range,
    content: Content<Self, 4>,
}
impl QuadTree {
    fn find_tuning_frequency(&self) -> Option<i64> {
        match &self.content {
            Content::MaybeBeacon => Some(self.x_range.min * 4_000_000 + self.y_range.min),
            Content::Mixed(arr) => {
                for tree in arr.iter() {
                    if let Some(tree) = tree {
                        if let Some(val) = tree.find_tuning_frequency() {
                            return Some(val);
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }
    fn new(points: &Points, x_range: Range, y_range: Range) -> Self {
        let make_quadrants = || match (x_range.split(), y_range.split()) {
            (Some((x_l, x_r)), Some((y_l, y_r))) => Self {
                x_range,
                y_range,
                content: Content::Mixed(Box::new([
                    Some(Self::new(points, x_l, y_l)),
                    Some(Self::new(points, x_r, y_l)),
                    Some(Self::new(points, x_l, y_r)),
                    Some(Self::new(points, x_r, y_r)),
                ])),
            },
            (Some((x_l, x_r)), None) => Self {
                x_range,
                y_range,
                content: Content::Mixed(Box::new([
                    Some(Self::new(points, x_l, y_range)),
                    Some(Self::new(points, x_r, y_range)),
                    None,
                    None,
                ])),
            },
            (None, Some((y_l, y_r))) => Self {
                x_range,
                y_range,
                content: Content::Mixed(Box::new([
                    Some(Self::new(points, x_range, y_l)),
                    None,
                    Some(Self::new(points, x_range, y_r)),
                    None,
                ])),
            },
            (None, None) => panic!("unable to split into quadrants"),
        };
        for beacon in &points.beacons {
            if x_range.contains(beacon.x) && y_range.contains(beacon.y) {
                if x_range.single() && y_range.single() {
                    return Self {
                        x_range,
                        y_range,
                        content: Content::Beacon,
                    };
                }
                return make_quadrants();
            }
        }
        let mut any_empty = false;
        for sensor in &points.sensors {
            match (
                x_range.min.abs_diff(sensor.x) + y_range.min.abs_diff(sensor.y) <= sensor.range,
                x_range.max.abs_diff(sensor.x) + y_range.min.abs_diff(sensor.y) <= sensor.range,
                x_range.min.abs_diff(sensor.x) + y_range.max.abs_diff(sensor.y) <= sensor.range,
                x_range.max.abs_diff(sensor.x) + y_range.max.abs_diff(sensor.y) <= sensor.range,
            ) {
                (true, true, true, true) => {
                    return Self {
                        x_range,
                        y_range,
                        content: Content::Empty,
                    }
                }
                (false, false, false, false) => (),
                _ => any_empty = true,
            }
        }
        match any_empty {
            false => Self {
                x_range,
                y_range,
                content: Content::MaybeBeacon,
            },
            true => make_quadrants(),
        }
    }
}
#[derive(Debug)]
struct BiTree {
    range: Range,
    content: Content<Self, 2>,
}
impl BiTree {
    fn count_empty(&self) -> u64 {
        match &self.content {
            Content::Empty => self.range.max.abs_diff(self.range.min) + 1,
            Content::Mixed(val) => {
                if let (Some(l), Some(r)) = (&val[0], &val[1]) {
                    l.count_empty() + r.count_empty()
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
    fn new(points: &Points, y_val: i64, range: Option<Range>) -> Self {
        let range = range.unwrap_or_default();
        for beacon in &points.beacons {
            if beacon.y == y_val && beacon.x <= range.max && beacon.x >= range.min {
                if range.single() {
                    return Self {
                        range,
                        content: Content::Beacon,
                    };
                }
                let (l, r) = range.split().unwrap();
                return Self {
                    range,
                    content: Content::Mixed(Box::new([
                        Some(Self::new(points, y_val, Some(l))),
                        Some(Self::new(points, y_val, Some(r))),
                    ])),
                };
            }
        }
        let mut any_empty = false;
        for sensor in &points.sensors {
            if sensor.y.abs_diff(y_val) > sensor.range {
                continue;
            }
            let hor_range = Range {
                min: sensor.x - (sensor.range - sensor.y.abs_diff(y_val)) as i64,
                max: sensor.x + (sensor.range - sensor.y.abs_diff(y_val)) as i64,
            };
            if hor_range.surrounds(&range) {
                return Self {
                    range,
                    content: Content::Empty,
                };
            }
            if hor_range.overlaps(&range) {
                any_empty = true;
            }
        }
        match any_empty {
            false => Self {
                range,
                content: Content::MaybeBeacon,
            },
            true => {
                let (r, l) = range.split().unwrap();
                Self {
                    range,
                    content: Content::Mixed(Box::new([
                        Some(Self::new(points, y_val, Some(l))),
                        Some(Self::new(points, y_val, Some(r))),
                    ])),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    min: i64,
    max: i64,
}
impl Range {
    fn split(&self) -> Option<(Self, Self)> {
        if self.min >= self.max {
            return None;
        }
        let half_offset = (self.min.abs_diff(self.max).saturating_add(1) / 2) as i64;
        Some((
            Self {
                min: self.min,
                max: self.max - half_offset,
            },
            Self {
                min: self.max - half_offset + 1,
                max: self.max,
            },
        ))
    }
    fn surrounds(&self, other: &Self) -> bool {
        self.min <= other.min && self.max >= other.max
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.min <= other.max && self.max >= other.min
    }
    fn contains(&self, val: i64) -> bool {
        val >= self.min && val <= self.max
    }
    fn single(&self) -> bool {
        self.min == self.max
    }
}
impl Default for Range {
    fn default() -> Self {
        Self {
            min: i64::MIN,
            max: i64::MAX,
        }
    }
}

#[derive(Debug)]
enum Content<T, const N: usize> {
    Empty,
    Beacon,
    MaybeBeacon,
    Mixed(Box<[Option<T>; N]>),
}

#[derive(Debug)]
struct Points {
    sensors: Vec<Sensor>,
    beacons: Vec<Beacon>,
}
impl From<&str> for Points {
    fn from(value: &str) -> Self {
        let mut sensors = Vec::new();
        let mut beacons = Vec::new();

        for line in value.lines() {
            let mut nums = line
                .split([' ', '=', ',', ':'])
                .filter_map(|w| w.parse::<i64>().ok());
            let sensor_x = nums.next().unwrap();
            let sensor_y = nums.next().unwrap();
            let beacon_x = nums.next().unwrap();
            let beacon_y = nums.next().unwrap();
            sensors.push(Sensor {
                x: sensor_x,
                y: sensor_y,
                range: sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y),
            });
            beacons.push(Beacon {
                x: beacon_x,
                y: beacon_y,
            });
        }

        Self { sensors, beacons }
    }
}
#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    range: u64,
}
#[derive(Debug)]
struct Beacon {
    x: i64,
    y: i64,
}
