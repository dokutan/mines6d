use super::board;
use cursive::theme::{BaseColor, Color, ColorStyle};

/// Used to convert the value of a cell to a styled string.
pub struct Tileset {
    use_color: bool,
    use_unicode: bool,
}

impl Tileset {
    /// Creates a new `Tileset` containing default values.
    pub const fn new(use_color: bool, use_unicode: bool) -> Self {
        Self {
            use_color,
            use_unicode,
        }
    }

    /// Checks `self.use_color` and `self.use_unicode` and returns a formatted `String` and `Colorstyle`.
    pub fn format_cell(&self, value: u16) -> (String, ColorStyle) {
        let string = if self.use_unicode {
            Self::format_cell_unicode(value)
        } else {
            Self::format_cell_ascii(value)
        };

        let style = if self.use_color {
            Self::format_cell_colorstyle(value)
        } else {
            ColorStyle::inherit_parent()
        };

        (string, style)
    }

    /// Formats value using ASCII characters.
    pub fn format_cell_ascii(value: u16) -> String {
        if board::Board::is_covered(value) {
            "#".to_string()
        } else if board::Board::is_flagged(value) {
            "X".to_string()
        } else if board::Board::is_marked(value) {
            "?".to_string()
        } else if board::Board::is_empty(value) {
            let value = board::Board::mines(value);
            match value {
                0 => ".".to_string(),
                1..=15 => format!("{:X}", value),
                _ => "+".to_string(),
            }
        } else {
            "*".to_string()
        }
    }

    /// Formats value using Unicode characters.
    pub fn format_cell_unicode(value: u16) -> String {
        if board::Board::is_covered(value) {
            "▮".to_string()
        } else if board::Board::is_flagged(value) {
            "⚑".to_string()
        } else if board::Board::is_marked(value) {
            "?".to_string()
        } else if board::Board::is_empty(value) {
            let value = board::Board::mines(value);
            match value {
                0 => "·".to_string(),
                1..=15 => format!("{:X}", value),
                _ => "+".to_string(),
            }
        } else {
            "*".to_string()
        }
    }

    /// Returns the  `ColorStyle` that matches value.
    pub fn format_cell_colorstyle(value: u16) -> ColorStyle {
        let bg = Color::Dark(BaseColor::White);

        if board::Board::is_covered(value) {
            ColorStyle::new(Color::Dark(BaseColor::Black), bg)
        } else if board::Board::is_flagged(value) {
            ColorStyle::new(Color::Dark(BaseColor::Blue), bg)
        } else if board::Board::is_marked(value) {
            ColorStyle::new(Color::Dark(BaseColor::Magenta), bg)
        } else if board::Board::is_empty(value) {
            match board::Board::mines(value) {
                0 => ColorStyle::new(Color::Dark(BaseColor::Cyan), bg),
                1 | 2 | 3 => ColorStyle::new(Color::RgbLowRes(0, 3, 0), bg),
                4 | 5 | 6 => ColorStyle::new(Color::RgbLowRes(1, 2, 0), bg),
                7 | 8 | 9 => ColorStyle::new(Color::RgbLowRes(2, 1, 0), bg),
                _ => ColorStyle::new(Color::RgbLowRes(3, 0, 0), bg),
            }
        } else {
            ColorStyle::inherit_parent()
        }
    }
}
