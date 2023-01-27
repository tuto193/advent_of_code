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
}
