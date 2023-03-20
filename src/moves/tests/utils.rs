use crate::board::Position;

pub fn assert_moves(expected: &Vec<Position>, actual: &Vec<Position>) {
    assert_eq!(expected.len(), actual.len());
    for expected_pos in expected {
        assert!(actual.iter().find(|pos| &expected_pos == pos).is_some());
    }
}
