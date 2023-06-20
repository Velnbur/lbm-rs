use super::Geometry;
use crate::grid;
use crate::num;

pub struct Rectangle {
    x_center: [num; 2],
    lengths: [num; 2],
}

impl Rectangle {
    pub fn new(x_center: [num; 2], lengths: [num; 2]) -> Self {
        Self { x_center, lengths }
    }
}

impl Geometry for Rectangle {
    #[inline(always)]
    fn contains(&self, point: grid::X) -> bool {
        let x = point.0 as num;
        let y = point.1 as num;
        let x_center = self.x_center[0];
        let y_center = self.x_center[1];
        let x_length = self.lengths[0];
        let y_length = self.lengths[1];

        x >= x_center - x_length / 2.
            && x <= x_center + x_length / 2.
            && y >= y_center - y_length / 2.
            && y <= y_center + y_length / 2.
    }
}
