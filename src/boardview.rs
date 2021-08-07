use crate::{board, Options};
use cursive::{
    event::{Event, EventResult},
    theme::{ColorStyle, Style},
    utils::span::{SpannedStr, SpannedString},
    Printer, Vec2,
};
use std::{
    cmp::max,
    fs::{create_dir, OpenOptions},
    io::Write,
    path::Path,
};

mod tileset;

/// This struct is responsible for the interaction between the user and the board.
pub struct BoardView {
    board: board::Board,
    current_view: (usize, usize, usize, usize),
    current_pos: (usize, usize),
    tileset: tileset::Tileset,
    /// Spacing factor for the individual cells
    h_space: usize,
    /// Space between the additional views
    view_padding: usize,
    /// Number of lines used to display the status
    y_offset: usize,
    /// Length of the labels for the additional views
    label_len: usize,
    /// Path of the history file
    options: Options,
}

impl BoardView {
    pub fn new(
        size: (usize, usize, usize, usize, usize, usize),
        mines: u32,
        cheats: u32,
        options: Options,
    ) -> Self {
        let board = board::Board::new(size, mines, cheats);

        let current_view = (0, 0, 0, 0);
        let current_pos = (0, 0);

        // get tileset options from config
        let use_color = if let Some(ref config) = options.config {
            config.use_color
        } else {
            true
        };
        let use_unicode = if let Some(ref config) = options.config {
            config.use_unicode
        } else {
            true
        };

        let tileset = tileset::Tileset::new(use_color, use_unicode);

        // get h_space from config
        let h_space = if let Some(ref config) = options.config {
            config.cells_h_space
        } else {
            2
        };

        Self {
            board,
            current_view,
            current_pos,
            tileset,
            h_space,
            view_padding: 2,
            y_offset: 5,
            label_len: 6,
            options,
        }
    }

    /// Returns the current position as a formatted String.
    pub fn format_pos_string(&self) -> String {
        format!(
            "Position ({}, {}, {}, {}, {}, {})",
            self.current_pos.1,
            self.current_pos.0,
            self.current_view.3,
            self.current_view.2,
            self.current_view.1,
            self.current_view.0
        )
    }

    /// Returns the board size as a formatted String.
    pub fn format_size_string(&self) -> String {
        format!(
            "Size     ({}, {}, {}, {}, {}, {})",
            self.board.board.dim().5,
            self.board.board.dim().4,
            self.board.board.dim().3,
            self.board.board.dim().2,
            self.board.board.dim().1,
            self.board.board.dim().0,
        )
    }

    /// Changes `self.current_view` and `self.current_pos` according to direction, bounded by the board size.
    pub fn change_pos(&mut self, direction: (i8, i8, i8, i8, i8, i8)) {
        let (s6, s5, s4, s3, s2, s1) = self.board.board.dim();
        let (d6, d5, d4, d3, d2, d1) = direction;

        self.current_view.0 = Self::add_checked(self.current_view.0, d6, 0, s6 - 1);
        self.current_view.1 = Self::add_checked(self.current_view.1, d5, 0, s5 - 1);
        self.current_view.2 = Self::add_checked(self.current_view.2, d4, 0, s4 - 1);
        self.current_view.3 = Self::add_checked(self.current_view.3, d3, 0, s3 - 1);
        self.current_pos.0 = Self::add_checked(self.current_pos.0, d2, 0, s2 - 1);
        self.current_pos.1 = Self::add_checked(self.current_pos.1, d1, 0, s1 - 1);
    }

    fn draw_board(&self, printer: &Printer, offset: (usize, usize), view: (i8, i8, i8, i8)) {
        let (s6, s5, s4, s3, y, x) = self.board.board.dim();

        let x6 = Self::add_checked(self.current_view.0, view.0, 0, s6 - 1);
        let x5 = Self::add_checked(self.current_view.1, view.1, 0, s5 - 1);
        let x4 = Self::add_checked(self.current_view.2, view.2, 0, s4 - 1);
        let x3 = Self::add_checked(self.current_view.3, view.3, 0, s3 - 1);

        // valid view ?
        if (x6 == self.current_view.0 && view.0 != 0)
            || (x5 == self.current_view.1 && view.1 != 0)
            || (x4 == self.current_view.2 && view.2 != 0)
            || (x3 == self.current_view.3 && view.3 != 0)
        {
            return;
        }

        for x2 in 0..y {
            for x1 in 0..x {
                let value = self.board.board[[x6, x5, x4, x3, x2, x1]];
                let (string, mut style) = self.tileset.format_cell(value);

                // highlight if cursor is at current cell
                if (x2, x1) == self.current_pos {
                    style = ColorStyle::merge(style, ColorStyle::highlight());
                }

                let styled = SpannedString::<Style>::styled(&string, style);
                printer.print_styled(
                    (x1 * self.h_space + offset.0, x2 + offset.1),
                    SpannedStr::<Style>::from(&styled),
                );
            }
        }
    }

    // a + b if: min <= (a + b) <= max
    pub const fn add_checked(a: usize, b: i8, min: usize, max: usize) -> usize {
        let mut a = a as isize;
        a += b as isize;

        if a < min as isize {
            min
        } else if a > max as isize {
            max
        } else {
            a as usize
        }
    }

    /// Writes the result and options of the current game to the history file.
    fn store_result(&self, result: &str) {
        if let Some(history_path) = &self.options.history_path {
            // attempt to create parent directory if it doesn't exist
            if let Some(parent) = history_path.parent() {
                if !Path::exists(parent) {
                    let _ = create_dir(parent);
                }
            }

            // write to history file
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(history_path)
            {
                let size = self.board.board.dim();
                let _ = file.write_all(
                    format!("{{\"result\": \"{}\", \"mines\": {}, \"cheats\": {}, \"size\": [{}, {}, {}, {}, {}, {}]}}\n",
                        result,
                        self.board.mines_total,
                        self.board.cheats_total,
                        size.5,
                        size.4,
                        size.3,
                        size.2,
                        size.1,
                        size.0
                    ).as_bytes()
                );
            }
        }
    }
}

impl cursive::view::View for BoardView {
    fn draw(&self, printer: &Printer) {
        // the size of a single view
        let (x6, x5, x4, x3, y, x) = self.board.board.dim();
        let view_width = max(x * self.h_space, self.label_len);
        let view_height = y;

        // print status (position, size, …)
        let current_cell = self.board.board[[
            self.current_view.0,
            self.current_view.1,
            self.current_view.2,
            self.current_view.3,
            self.current_pos.0,
            self.current_pos.1,
        ]];
        let neighboring_mines = if board::Board::is_uncovered(current_cell) {
            format!("{}", board::Board::mines(current_cell))
        } else {
            "?".to_string()
        };

        printer.print((0, 0), self.format_pos_string().as_str());
        printer.print((0, 1), self.format_size_string().as_str());
        printer.print(
            (0, 2),
            format!(
                "Mines    {}+{}/{} ({})",
                self.board.mines_flagged,
                self.board.mines_marked,
                self.board.mines_total,
                neighboring_mines
            )
            .as_str(),
        );
        printer.print(
            (0, 3),
            format!("Cheats   {}", self.board.cheats_remaining).as_str(),
        );

        // print current view of the board
        self.draw_board(printer, (0, self.y_offset), (0, 0, 0, 0));

        // print additional views
        let space = self.view_padding;
        let y = view_height + self.y_offset + space;
        let mut x = 0;

        // x3
        if x3 > 1 {
            printer.print((x, y - 1), "x₃: +1");
            self.draw_board(printer, (x, y), (0, 0, 0, 1));
            printer.print((x, y + view_height + space - 1), "x₃: -1");
            self.draw_board(printer, (x, y + view_height + space), (0, 0, 0, -1));
            x += view_width + space;
        }

        // x4
        if x4 > 1 {
            printer.print((x, y - 1), "x₄: +1");
            self.draw_board(printer, (x, y), (0, 0, 1, 0));
            printer.print((x, y + view_height + space - 1), "x₄: -1");
            self.draw_board(printer, (x, y + view_height + space), (0, 0, -1, 0));
            x += view_width + space;
        }

        // x5
        if x5 > 1 {
            printer.print((x, y - 1), "x₅: +1");
            self.draw_board(printer, (x, y), (0, 1, 0, 0));
            printer.print((x, y + view_height + space - 1), "x₅: -1");
            self.draw_board(printer, (x, y + view_height + space), (0, -1, 0, 0));
            x += view_width + space;
        }

        // x6
        if x6 > 1 {
            printer.print((x, y - 1), "x₆: +1");
            self.draw_board(printer, (x, y), (1, 0, 0, 0));
            printer.print((x, y + view_height + space - 1), "x₆: -1");
            self.draw_board(printer, (x, y + view_height + space), (-1, 0, 0, 0));
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        let (x6, x5, x4, x3, y, x) = self.board.board.dim();

        // number of additional views
        let mut num_views_x = 0;
        if x6 > 1 {
            num_views_x += 1;
        }
        if x5 > 1 {
            num_views_x += 1;
        }
        if x4 > 1 {
            num_views_x += 1;
        }
        if x3 > 1 {
            num_views_x += 1;
        }

        // number of spaces between the views in x direction
        let num_padding_x = if num_views_x != 0 { num_views_x - 1 } else { 0 };

        // number of total views in the y direction
        let num_views_y = if num_views_x == 0 { 1 } else { 3 };

        // number of spaces between the views in y direction
        let num_padding_y = num_views_y - 1;

        let width_views = num_views_x * max(x * self.h_space, self.label_len)
            + (num_padding_x * self.view_padding);
        let width_size = self.format_size_string().len();

        let height = self.y_offset + (num_views_y * y) + (num_padding_y * self.view_padding);

        Vec2::new(max(width_views, width_size), height)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            // cursor movement
            Event::Char('w') => self.change_pos((0, 0, 0, 0, -1, 0)),
            Event::Char('s') => self.change_pos((0, 0, 0, 0, 1, 0)),
            Event::Char('a') => self.change_pos((0, 0, 0, 0, 0, -1)),
            Event::Char('d') => self.change_pos((0, 0, 0, 0, 0, 1)),
            Event::Char('q') => self.change_pos((0, 0, 0, -1, 0, 0)),
            Event::Char('e') => self.change_pos((0, 0, 0, 1, 0, 0)),
            Event::Char('i') => self.change_pos((0, -1, 0, 0, 0, 0)),
            Event::Char('k') => self.change_pos((0, 1, 0, 0, 0, 0)),
            Event::Char('j') => self.change_pos((0, 0, -1, 0, 0, 0)),
            Event::Char('l') => self.change_pos((0, 0, 1, 0, 0, 0)),
            Event::Char('u') => self.change_pos((-1, 0, 0, 0, 0, 0)),
            Event::Char('o') => self.change_pos((1, 0, 0, 0, 0, 0)),

            // uncover cell
            Event::Char(' ') => {
                let (x2, x1) = self.current_pos;
                let (x6, x5, x4, x3) = self.current_view;

                if self.board.uncover_cell((x6, x5, x4, x3, x2, x1)) {
                    self.store_result("lost");
                    return EventResult::Ignored;
                }
            }

            // flag cell
            Event::Char('f') => {
                let (x2, x1) = self.current_pos;
                let (x6, x5, x4, x3) = self.current_view;

                if self.board.flag_cell((x6, x5, x4, x3, x2, x1)) {
                    self.store_result("won");
                    return EventResult::Ignored;
                }
            }

            // mark cell
            Event::Char('r') => {
                let (x2, x1) = self.current_pos;
                let (x6, x5, x4, x3) = self.current_view;

                self.board.mark_cell((x6, x5, x4, x3, x2, x1));
            }

            // cheat
            Event::Char('c') => {
                let (x2, x1) = self.current_pos;
                let (x6, x5, x4, x3) = self.current_view;

                if self.board.cheat_cell((x6, x5, x4, x3, x2, x1)) {
                    self.store_result("won");
                    return EventResult::Ignored;
                }
            }

            _ => return EventResult::Ignored,
        }

        EventResult::Consumed(None)
    }
}
