#[cfg(test)]
mod board_tests {

    use super::super::Board;

    #[test]
    fn test_is_covered() {
        assert!(Board::is_covered(0x0fff));
        assert!(!Board::is_covered(0x4fff));
        assert!(!Board::is_covered(0x8fff));
        assert!(!Board::is_covered(0xcfff));
    }

    #[test]
    fn test_is_flagged() {
        assert!(!Board::is_flagged(0x0fff));
        assert!(Board::is_flagged(0x4fff));
        assert!(!Board::is_flagged(0x8fff));
        assert!(!Board::is_flagged(0xcfff));
    }

    #[test]
    fn test_is_marked() {
        assert!(!Board::is_marked(0x0fff));
        assert!(!Board::is_marked(0x4fff));
        assert!(Board::is_marked(0x8fff));
        assert!(!Board::is_marked(0xcfff));
    }

    #[test]
    fn test_is_uncovered() {
        assert!(!Board::is_uncovered(0x0fff));
        assert!(!Board::is_uncovered(0x4fff));
        assert!(!Board::is_uncovered(0x8fff));
        assert!(Board::is_uncovered(0xcfff));
    }

    #[test]
    fn test_is_empty() {
        assert!(Board::is_empty(0b0000_0000_0000_0000));
        assert!(!Board::is_empty(0b0010_0000_0000_0000));
        assert!(Board::is_empty(0b1101_1111_1111_1111));
        assert!(!Board::is_empty(0b1111_1111_1111_1111));
    }

    #[test]
    fn test_mines() {
        assert_eq!(Board::mines(0b0001_1111_1111_1111), 0b0001_1111_1111_1111);
        assert_eq!(Board::mines(0b1111_1111_1111_1111), 0b0001_1111_1111_1111);
        assert_eq!(Board::mines(0b0000_0000_0000_0000), 0b0000_0000_0000_0000);
        assert_eq!(Board::mines(0b1110_0000_0000_0000), 0b0000_0000_0000_0000);
    }
}
