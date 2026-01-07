use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

pub struct Stretch<'a> {
    speed: u32,
    length: u32,
    cars: u32,
    cap: u32,
    connections: Vec<&'a Self>,
}

impl Stretch<'_> {
    fn weight(&self) -> f32 {
        let lov = self.length as f32 / self.speed as f32;
        let noc = (self.cars as f32 / self.cap as f32);
        let noc4 = noc * noc * noc * noc;
        (0.15 * noc4 + 1.0) * lov
    }
}

impl PartialEq for Stretch<'_> {
    fn eq(&self, other: &Self) -> bool {
        // Im gonna say that two stretches are effectively equal
        // if they are within some tolerance of eachother, this
        // may not actually be needed but whatever

        (self.weight() - other.weight()).abs() <= 0.001
    }
}

impl Eq for Stretch<'_> {}

impl PartialOrd for Stretch<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Stretch<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            std::cmp::Ordering::Equal
        } else if self.weight() < other.weight() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}
