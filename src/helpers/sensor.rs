#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// line:
/// f(y) = a·x + b
/// a = slope
/// b = y intercept
pub struct Line {
    start: Point,
    end: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pair {
    pub sensor: Point,
    pub beacon: Point,
}
#[derive(Debug, Clone, Copy)]
pub struct Sensor {
    pub radius: usize,
    pub centre: Point,
}

impl Sensor {
    pub fn new(sensor: Point, beacon: Point) -> Self {
        let radius = sensor.x.abs_diff(beacon.x) + sensor.y.abs_diff(beacon.y);
        Self {
            radius: radius,
            centre: sensor,
        }
    }

    fn manhattan_distance(&self, p: &Point) -> usize {
        self.centre.x.abs_diff(p.x) + self.centre.x.abs_diff(p.y)
    }

    pub fn has_within_radius(&self, p: &Point) -> bool {
        self.manhattan_distance(p) <= self.radius
    }

    fn is_within_row(&self, row: isize) -> bool {
        // let top = if self.radius as isize > self.centre.x {
        //     0
        // } else {
        //     self.centre.x - self.radius as isize
        // };
        let top = self.centre.y - self.radius as isize;
        let bottom = self.centre.y + self.radius as isize;
        row > top && row < bottom
    }

    pub fn get_all_in_row(&self, row: isize) -> Vec<isize> {
        let mut within_range = vec![];
        if self.is_within_row(row) {
            let remaining_length = self.radius as isize - self.centre.y.abs_diff(row) as isize;
            for x in self.centre.x - remaining_length..=self.centre.x + remaining_length {
                within_range.push(x);
            }
        }
        within_range
    }

    pub fn get_all_within_radius(&self, large_lim: isize) -> Vec<Point> {
        let mut pos_in_range = vec![];
        // let large_lim: isize = 4000000;
        let left = self.centre.x - self.radius as isize;
        let left = left.max(0).min(large_lim);
        let right = self.centre.x + self.radius as isize;
        let right = right.max(0).min(large_lim);
        let top = self.centre.y - self.radius as isize;
        let top = top.max(0).min(large_lim);
        let bottom = self.centre.y + self.radius as isize;
        let bottom = bottom.max(0).min(large_lim);

        for y in top..=bottom {
            for x in left..=right {
                let manhattan = self.manhattan_distance(&Point{x, y});
                if manhattan <= self.radius {
                    pos_in_range.push(Point{x: x, y: y});
                }
            }
        }
        pos_in_range
    }

    pub fn get_all_around_radius(&self, large_lim: isize) -> Vec<Point> {
       let mut around_radius = vec![];
        // let large_lim: isize = 4000000;
        // let large_lim: isize = 20;
        let left = self.centre.x - self.radius as isize - 1;
        let left = left.max(0).min(large_lim);
        let right = self.centre.x + self.radius as isize + 1;
        let right = right.max(0).min(large_lim);
        let top = self.centre.y - self.radius as isize - 1;
        let top = top.max(0).min(large_lim);
        let bottom = self.centre.y + self.radius as isize + 1;
        let bottom = bottom.max(0).min(large_lim);

        for y in top..=bottom {
            for x in left..=right {
                let manhattan = self.manhattan_distance(&Point{x, y});
                if manhattan == self.radius + 1 {
                    around_radius.push(Point{x: x, y: y});
                }
            }
        }
        around_radius
    }

    pub fn top_left(&self) -> Line {
        let extra_length = self.radius as isize + 1;
        let start = Point {
            x: self.centre.x - extra_length,
            y: self.centre.y,
        };
        let end = Point {
            x: self.centre.x,
            y: self.centre.y - extra_length,
        };
        Line {
            start: start,
            end: end,
        }
    }

    pub fn top_right(&self) -> Line {
        let extra_length = self.radius as isize + 1;
        let start = Point {
            x: self.centre.x,
            y: self.centre.y - extra_length,
        };
        let end = Point {
            x: self.centre.x + extra_length,
            y: self.centre.y,
        };
        Line {
            start: start,
            end: end,
        }
    }

    pub fn bottom_left(&self) -> Line {
        let extra_length = self.radius as isize + 1;
        let start = Point {
            x: self.centre.x - extra_length,
            y: self.centre.y,
        };
        let end = Point {
            x: self.centre.x,
            y: self.centre.y + extra_length,
        };
        Line {
            start: start,
            end: end,
        }
    }

    pub fn bottom_right(&self) -> Line {
        let extra_length = self.radius as isize + 1;
        let start = Point {
            x: self.centre.x,
            y: self.centre.y + extra_length,
        };
        let end = Point {
            x: self.centre.x + extra_length,
            y: self.centre.y,
        };
        Line {
            start: start,
            end: end,
        }
    }
}

impl Line {
    pub fn slope(&self) -> isize {
        if self.end.y - self.start.y > 0 {
            1
        } else {
            -1
        }
    }

    pub fn y_intercept(&self) -> isize {
        self.start.y - self.slope() * self.start.x
    }

    pub fn y_at(&self, x: isize) -> Option<isize> {
        if self.start.x > x || self.end.x < x {
            return None;
        }
        Some(self.slope() * x + self.y_intercept())
    }

    pub fn overlap(&self, other: &Line) -> Option<Line> {
        debug_assert_eq!(self.slope(), other.slope());
        debug_assert_eq!(self.y_intercept(), other.y_intercept());
        let x = self.start.x.max(other.start.x);
        let y = self.y_at(x)?;
        let start = Point { x, y };

        let x = self.end.x.min(other.end.x);
        let y = self.y_at(x)?;
        let end = Point { x, y };
        Some(Line { start, end })
    }

    pub fn interception(&self, other: &Line) -> Option<Point> {
        debug_assert_ne!(self.slope(), other.slope());
        let y_intercept_diff = other.y_intercept() - self.y_intercept();
        let slope_diff = self.slope() - other.slope();
        let x = y_intercept_diff / slope_diff;
        let y = self.y_at(x)?;
        if y != other.y_at(x)? {
            return None;
        }
        Some(Point { x, y })
    }
}


impl Point {
    fn manhattan_distance(&self, other: &Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Pair {
    pub fn cover_distance(&self) -> isize {
        self.sensor.manhattan_distance(&self.beacon)
    }

    pub fn top_left(&self) -> Line {
        Line {
            start: Point {
                x: self.sensor.x - self.cover_distance() - 1,
                y: self.sensor.y,
            },
            end: Point {
                x: self.sensor.x,
                y: self.sensor.y - self.cover_distance() - 1,
            },
        }
    }

    pub fn top_right(&self) -> Line {
        Line {
            start: Point {
                x: self.sensor.x,
                y: self.sensor.y - self.cover_distance() - 1,
            },
            end: Point {
                x: self.sensor.x + self.cover_distance() + 1,
                y: self.sensor.y,
            },
        }
    }

    pub fn bottom_left(&self) -> Line {
        Line {
            start: Point {
                x: self.sensor.x - self.cover_distance() - 1,
                y: self.sensor.y,
            },
            end: Point {
                x: self.sensor.x,
                y: self.sensor.y + self.cover_distance() + 1,
            },
        }
    }

    pub fn bottom_right(&self) -> Line {
        Line {
            start: Point {
                x: self.sensor.x,
                y: self.sensor.y + self.cover_distance() + 1,
            },
            end: Point {
                x: self.sensor.x + self.cover_distance() + 1,
                y: self.sensor.y,
            },
        }
    }

    pub fn covers(&self, point: &Point) -> bool {
        self.sensor.manhattan_distance(point) <= self.cover_distance()
    }

    // fn covered_xs(&self, row: isize) -> Option<Range<isize>> {
    //     let x_offset = self.cover_distance() - (self.sensor.y - row).abs();
    //     Some(self.sensor.x - x_offset..self.sensor.x + x_offset + 1).filter(|r| !r.is_empty())
    // }
}
