use super::constants::{A_FILE, EIGHTH_RANK, FIRST_RANK, H_FILE};

#[derive(Debug, Clone, PartialEq)]
pub struct Raycast {
    starting_position: u64,
    detection_layer: u64,
    blocked_layer: u64,
}

impl Raycast {
    pub fn new(starting_position: u64, detection_layer: u64, blocked_layer: u64) -> Self {
        return Self {
            starting_position,
            detection_layer,
            blocked_layer,
        };
    }

    pub fn get_first_hit(&self, direction: &Direction) -> u64 {
        let mut current = self.starting_position;
        let mut detection = 0;

        // Do not detect or be blocked by self
        let blocked = self.blocked_layer & !self.starting_position;
        let detect = self.detection_layer & !self.starting_position;

        while current != 0 {
            // Stop casting anything that has hit a blocker
            current &= !blocked;
            // Add detections from the detection layer
            detection |= current & detect;
            // Stop casting after any hit detections
            current &= !detection;
            // Advance the current position
            current = direction.advance(current);
        }

        detection
    }

    pub fn get_full_ray(&self, direction: &Direction) -> u64 {
        let mut current = self.starting_position;
        let mut ray = 0;

        // Do not detect or be blocked by self
        let blocked = self.blocked_layer & !self.starting_position;
        let detect = self.detection_layer & !self.starting_position;

        while current != 0 {
            // Advance the ray first
            current = direction.advance(current);
            // Detect if hit a blocker, flip to 0 if so
            current &= !blocked;
            // Add the current position to the ray
            ray |= current;
            // If hit the detect layer, flip the bit to 0 - after having added the square to ray (detects captures)
            current &= !detect;
        }

        ray
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    pub fn rook_directions() -> Vec<Direction> {
        vec![Direction::N, Direction::E, Direction::S, Direction::W]
    }
    pub fn bishop_directions() -> Vec<Direction> {
        vec![Direction::NE, Direction::SE, Direction::SW, Direction::NW]
    }
    pub fn advance(&self, current: u64) -> u64 {
        match self {
            Direction::N => (current & !EIGHTH_RANK) << 8,
            Direction::NE => (current & !H_FILE & !EIGHTH_RANK) << 9,
            Direction::E => (current & !H_FILE) << 1,
            Direction::SE => (current & !H_FILE & !FIRST_RANK) >> 7,
            Direction::S => (current & !FIRST_RANK) >> 8,
            Direction::SW => (current & !A_FILE & !FIRST_RANK) >> 9,
            Direction::W => (current & !A_FILE) >> 1,
            Direction::NW => (current & !A_FILE & !EIGHTH_RANK) << 7,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cgame::moves::algebraic_to_u64;

    use super::*;

    #[test]
    fn single_detection() {
        let starting_position = algebraic_to_u64("f3").unwrap();
        let detection_layer = algebraic_to_u64("b7").unwrap();
        let blocked_layer = 0;
        let raycast = Raycast::new(starting_position, detection_layer, blocked_layer);

        let hits = raycast.get_first_hit(&Direction::NW);
        assert_eq!(hits, detection_layer);
    }

    #[test]
    fn single_detection_blocked() {
        let starting_position = algebraic_to_u64("f3").unwrap();
        let detection_layer = algebraic_to_u64("b7").unwrap();
        let blocked_layer = algebraic_to_u64("c6").unwrap();
        let raycast = Raycast::new(starting_position, detection_layer, blocked_layer);

        let hits = raycast.get_first_hit(&Direction::NW);
        assert_eq!(hits, 0);
    }

    #[test]
    fn single_detection_ignores_self() {
        let starting_position = algebraic_to_u64("f3").unwrap();
        let detection_layer = 0x2000000200000; // b7, f3
        let blocked_layer = algebraic_to_u64("f3").unwrap();
        let raycast = Raycast::new(starting_position, detection_layer, blocked_layer);

        let hits = raycast.get_first_hit(&Direction::NW);
        assert_eq!(hits, algebraic_to_u64("b7").unwrap());
    }

    #[test]
    fn single_detection_board_edge() {
        let starting_position = algebraic_to_u64("f3").unwrap();
        let detection_layer = algebraic_to_u64("a8").unwrap();
        let blocked_layer = 0;
        let raycast = Raycast::new(starting_position, detection_layer, blocked_layer);

        let hits = raycast.get_first_hit(&Direction::NW);
        assert_eq!(hits, detection_layer);
    }

    #[test]
    fn multi_detection() {
        let starting_position = 0x1800; // d2, e2
        let detection_layer = 0x810000000000000; // d8, e7
        let blocked_layer = 0;
        let raycast = Raycast::new(starting_position, detection_layer, blocked_layer);

        let hits = raycast.get_first_hit(&Direction::N);
        assert_eq!(hits, detection_layer);
    }

    #[test]
    fn multi_detection_one_blocked() {
        let starting_position = 0x1800; // d2, e2
        let detection_layer = 0x810000000000000; // d8, e7
        let blocked_layer = 0x80000000000; // d6
        let raycast = Raycast::new(starting_position, detection_layer, blocked_layer);

        let hits = raycast.get_first_hit(&Direction::N);
        assert_eq!(hits, 0x10000000000000); // e7
    }

    #[test]
    fn full_ray_detects_capture() {
        let starting_position = 0x1000; // e2
        let detection_layer = 0x200000000; // b5, NW from e2
        let blocked_layer = 0;
        let expected_ray = 0x204080000; // d3, c4, b5 - captures on b5
        let raycast = Raycast::new(starting_position, detection_layer, blocked_layer);

        let hits = raycast.get_full_ray(&Direction::NW);
        assert_eq!(hits, expected_ray);
    }

    #[test]
    fn full_ray_multi() {
        let starting_position = 0x808080000000; // h4, h5, h6
        let detection_layer = 0x200000000000; // f6
        let blocked_layer = 0x800000000; // d5
        let expected_ray = 0x60707f000000;
        let raycast = Raycast::new(starting_position, detection_layer, blocked_layer);

        let hits = raycast.get_full_ray(&Direction::W);
        assert_eq!(hits, expected_ray);
    }
}
