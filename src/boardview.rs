use crate::board;
use cursive::{
    event::{Event, EventResult},
    theme::{ColorStyle, Style},
    utils::span::{SpannedStr, SpannedString},
    Printer, Vec2,
};
use std::cmp::max;
use std::rc::Rc;

mod tileset;

/// This struct is responsible for the interaction between the user and the board.
pub struct BoardView {
    board: Rc<board::Board>,
    current_view: (usize, usize, usize, usize),
    current_pos: (usize, usize),
    tileset: tileset::Tileset,
    h_space: usize,
    view_padding: usize,
    y_offset: usize,
}

impl BoardView {
    pub fn new(size: (usize, usize, usize, usize, usize, usize), mines: u32, cheats: u32) -> Self {
        let board = Rc::new(board::Board::new(size, mines, cheats));

        let current_view = (0, 0, 0, 0);
        let current_pos = (0, 0);

        let tileset = tileset::Tileset::new();

        BoardView {
            board,
            current_view,
            current_pos,
            tileset,
            h_space: 2,
            view_padding: 2,
            y_offset: 5,
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

    /// Changes self.current_view and self.current_pos according to direction, bounded by the board size.
    pub fn change_pos(&mut self, direction: (i8, i8, i8, i8, i8, i8)) {
        let (s6, s5, s4, s3, s2, s1) = self.board.board.dim();
        let (d6, d5, d4, d3, d2, d1) = direction;

        self.current_view.0 = BoardView::add_checked(self.current_view.0, d6, 0, s6 - 1);
        self.current_view.1 = BoardView::add_checked(self.current_view.1, d5, 0, s5 - 1);
        self.current_view.2 = BoardView::add_checked(self.current_view.2, d4, 0, s4 - 1);
        self.current_view.3 = BoardView::add_checked(self.current_view.3, d3, 0, s3 - 1);
        self.current_pos.0 = BoardView::add_checked(self.current_pos.0, d2, 0, s2 - 1);
        self.current_pos.1 = BoardView::add_checked(self.current_pos.1, d1, 0, s1 - 1);
    }

    fn draw_board(&self, printer: &Printer, offset: (usize, usize), view: (i8, i8, i8, i8)) {
        let (s6, s5, s4, s3, y, x) = self.board.board.dim();

        let x6 = BoardView::add_checked(self.current_view.0, view.0, 0, s6 - 1);
        let x5 = BoardView::add_checked(self.current_view.1, view.1, 0, s5 - 1);
        let x4 = BoardView::add_checked(self.current_view.2, view.2, 0, s4 - 1);
        let x3 = BoardView::add_checked(self.current_view.3, view.3, 0, s3 - 1);

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
    pub fn add_checked(a: usize, b: i8, min: usize, max: usize) -> usize {
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
}

impl cursive::view::View for BoardView {
    fn draw(&self, printer: &Printer) {
        // the size of a single view
        let (_, _, _, _, y, x) = self.board.board.dim();
        let view_width = x * self.h_space;
        let view_height = y;

        // print status (position, size, …)
        printer.print((0, 0), self.format_pos_string().as_str());
        printer.print((0, 1), self.format_size_string().as_str());
        printer.print(
            (0, 2),
            format!(
                "Mines    {}+{}/{}",
                self.board.mines_flagged, self.board.mines_marked, self.board.mines_total
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

        // x3
        let x = 0;
        printer.print((x, y - 1), "x₃: +1");
        self.draw_board(printer, (x, y), (0, 0, 0, 1));
        printer.print((x, y + view_height + space - 1), "x₃ - 1");
        self.draw_board(printer, (x, y + view_height + space), (0, 0, 0, -1));

        // x4
        let x = view_width + space;
        printer.print((x, y - 1), "x₄: +1");
        self.draw_board(printer, (x, y), (0, 0, 1, 0));
        printer.print((x, y + view_height + space - 1), "x₄: -1");
        self.draw_board(printer, (x, y + view_height + space), (0, 0, -1, 0));

        // x5
        let x = (view_width + space) * 2;
        printer.print((x, y - 1), "x₅: +1");
        self.draw_board(printer, (x, y), (0, 1, 0, 0));
        printer.print((x, y + view_height + space - 1), "x₅: -1");
        self.draw_board(printer, (x, y + view_height + space), (0, -1, 0, 0));

        // x6
        let x = (view_width + space) * 3;
        printer.print((x, y - 1), "x₆: +1");
        self.draw_board(printer, (x, y), (1, 0, 0, 0));
        printer.print((x, y + view_height + space - 1), "x₆: -1");
        self.draw_board(printer, (x, y + view_height + space), (-1, 0, 0, 0));
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        let (_, _, _, _, y, x) = self.board.board.dim();

        let width_views = 4 * x * self.h_space + (3 * self.view_padding);
        let width_size = self.format_size_string().len();

        let height = self.y_offset + (3 * y) + (2 * self.view_padding);

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

                if Rc::get_mut(&mut self.board)
                    .unwrap()
                    .uncover_cell((x6, x5, x4, x3, x2, x1))
                {
                    return EventResult::Ignored;
                }
            }

            // flag cell
            Event::Char('f') => {
                let (x2, x1) = self.current_pos;
                let (x6, x5, x4, x3) = self.current_view;

                if Rc::get_mut(&mut self.board)
                    .unwrap()
                    .flag_cell((x6, x5, x4, x3, x2, x1))
                {
                    return EventResult::Ignored;
                }
            }

            // mark cell
            Event::Char('r') => {
                let (x2, x1) = self.current_pos;
                let (x6, x5, x4, x3) = self.current_view;

                Rc::get_mut(&mut self.board)
                    .unwrap()
                    .mark_cell((x6, x5, x4, x3, x2, x1));
            }

            // cheat
            Event::Char('c') => {
                let (x2, x1) = self.current_pos;
                let (x6, x5, x4, x3) = self.current_view;

                Rc::get_mut(&mut self.board)
                    .unwrap()
                    .cheat_cell((x6, x5, x4, x3, x2, x1));
            }

            _ => return EventResult::Ignored,
        }

        EventResult::Consumed(None)
    }
}
