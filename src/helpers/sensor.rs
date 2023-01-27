pub type Position = (isize, isize);

#[derive(Debug, Clone, Copy)]
pub struct Sensor {
    pub radius: usize,
    pub centre: Position,
}

impl Sensor {
    pub fn new(sensor: Position, beacon: Position) -> Self {
        let radius = sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1);
        Self {
            radius: radius,
            centre: sensor,
        }
    }

    fn is_within_row(&self, row: isize) -> bool {
        let top = if self.radius as isize > self.centre.1 {0} else {self.centre.1 - self.radius as isize};
        let bottom = self.centre.1 + self.radius as isize;
        row > top && row < bottom
    }

    pub fn get_all_in_row(&self, row: isize) -> Vec<isize> {
        let mut within_range = vec![];
        if self.is_within_row(row) {
            let remaining_length = self.radius as isize - self.centre.1.abs_diff(row) as isize;
            for x in self.centre.0 - remaining_length..=self.centre.0 + remaining_length {
                within_range.push(x);
            }
        }
        within_range
    }

    pub fn get_all_within_radius(&self) -> Vec<Position> {
        let mut pos_in_range = vec![];
        let large_lim: isize = 4000000;
        let left = self.centre.0 - self.radius as isize;
        let left = left.max(0).min(large_lim);
        let right = self.centre.0 + self.radius as isize;
        let right = right.max(0).min(large_lim);
        let top = self.centre.1 - self.radius as isize;
        let top = top.max(0).min(large_lim);
        let bottom = self.centre.1 + self.radius as isize;
        let bottom = bottom.max(0).min(large_lim);

        for y in top..=bottom {
            for x in left..=right {
                let manhattan = self.centre.0.abs_diff(x) + self.centre.1.abs_diff(y);
                if manhattan <= self.radius {
                    pos_in_range.push((x,y));
                }
            }
        }

        pos_in_range
    }
}
