use clap::{App, Arg};
use cursive::{
    event::{Event, Key},
    traits::*,
    view::Scrollable,
    views::{Dialog, DummyView, EditView, LinearLayout, Panel, ScrollView, TextView},
    Cursive,
};
use directories::ProjectDirs;
use std::{cmp::max, process::exit};

mod board;
mod boardview;
mod options;
use options::Options;

fn main() {
    // parse commandline arguments
    let args = App::new("mines6d")
        .arg(
            Arg::new("paths")
                .short('p')
                .long("paths")
                .about("show the config and history paths and exit"),
        )
        .get_matches();

    // print config and history paths
    if args.occurrences_of("paths") > 0 {
        let options = get_options();

        println!(
            "{}\n{}",
            options
                .history_path
                .unwrap_or_else(|| {
                    println!("history path is undefined");
                    exit(1);
                })
                .display(),
            options
                .config_path
                .unwrap_or_else(|| {
                    println!("config path is undefined");
                    exit(1);
                })
                .display(),
        );

        exit(0);
    }

    let mut siv = cursive::default();

    // add global callbacks
    siv.add_global_callback(Event::Key(Key::Esc), |s| s.quit());
    siv.add_global_callback(Event::Key(Key::F1), |s| show_help(s));

    show_main_menu(&mut siv);
    siv.run();
}

// get the global options from the config file
fn get_options() -> Options {
    let mut options: Options = Options::new();

    // get config and save paths
    if let Some(project_dirs) = ProjectDirs::from("org", "foo", "6dmines") {
        let mut history_path = project_dirs.data_dir().to_path_buf();
        history_path.push("history.json");

        let mut config_path = project_dirs.config_dir().to_path_buf();
        config_path.push("config");

        options.history_path = Some(history_path);
        options.config_path = Some(config_path);
    };

    // TODO! parse config file

    options
}

// shows the main menu
fn show_main_menu(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("Press F1 for help"))
                .child(TextView::new("Press Esc to exit"))
                .child(DummyView.fixed_height(1))
                .child(TextView::new("Size of x₁"))
                .child(EditView::new().content("10").with_name("edit_x1"))
                .child(DummyView.fixed_height(1))
                .child(TextView::new("Size of x₂"))
                .child(EditView::new().content("10").with_name("edit_x2"))
                .child(DummyView.fixed_height(1))
                .child(TextView::new("Size of x₃"))
                .child(EditView::new().content("3").with_name("edit_x3"))
                .child(DummyView.fixed_height(1))
                .child(TextView::new("Size of x₄"))
                .child(EditView::new().content("1").with_name("edit_x4"))
                .child(DummyView.fixed_height(1))
                .child(TextView::new("Size of x₅"))
                .child(EditView::new().content("1").with_name("edit_x5"))
                .child(DummyView.fixed_height(1))
                .child(TextView::new("Size of x₆"))
                .child(EditView::new().content("1").with_name("edit_x6"))
                .child(DummyView.fixed_height(1))
                .child(TextView::new("Mines"))
                .child(EditView::new().content("15").with_name("edit_mines"))
                .child(DummyView.fixed_height(1))
                .child(TextView::new("Cheats"))
                .child(EditView::new().content("0").with_name("edit_cheats"))
                .scrollable(),
        )
        .title("6D Minesweeper")
        .button("start", |s| {
            let x6 = max(get_editview_as(s, "edit_x6", 1), 1);
            let x5 = max(get_editview_as(s, "edit_x5", 1), 1);
            let x4 = max(get_editview_as(s, "edit_x4", 1), 1);
            let x3 = max(get_editview_as(s, "edit_x3", 3), 1);
            let x2 = max(get_editview_as(s, "edit_x2", 10), 1);
            let x1 = max(get_editview_as(s, "edit_x1", 10), 1);
            let mines = get_editview_as(s, "edit_mines", 15);
            let cheats = get_editview_as(s, "edit_cheats", 0);

            s.pop_layer();
            show_board(s, (x6, x5, x4, x3, x2, x1), mines, cheats);
        }),
    );
}

// shows the help dialog
fn show_help(s: &mut Cursive) {
    // remove callbacks to prevent the winning/losing dialogs being shown accidentally
    s.clear_global_callbacks('f');
    s.clear_global_callbacks('c');
    s.clear_global_callbacks(' ');

    s.add_layer(
        Dialog::around(TextView::new(include_str!("help.txt")).scrollable())
            .title("Help")
            .button("ok", |s| {
                s.pop_layer();
            }),
    );
}

// shows the "you lost" dialog
fn show_lost(s: &mut Cursive) {
    // remove callbacks to prevent the winning/losing dialogs being shown accidentally
    s.clear_global_callbacks('f');
    s.clear_global_callbacks('c');
    s.clear_global_callbacks(' ');

    s.add_layer(
        Dialog::text("Return to the main menu")
            .title("You lost")
            .button("ok", |s| {
                s.pop_layer();
                show_main_menu(s);
            }),
    );
}

// shows the "you won" dialog
fn show_won(s: &mut Cursive) {
    s.add_layer(
        Dialog::text("Return to the main menu")
            .title("You won")
            .button("ok", |s| {
                s.pop_layer();
                show_main_menu(s);
            }),
    );
}

// shows the board
fn show_board(
    s: &mut Cursive,
    size: (usize, usize, usize, usize, usize, usize),
    mines: u32,
    cheats: u32,
) {
    // add the BoardView
    let bv = boardview::BoardView::new(size, mines, cheats, get_options());
    s.add_layer(Panel::new(
        ScrollView::new(bv.with_name("boardview")).scroll_x(true),
    ));

    // add callbacks
    s.add_global_callback(Event::Char(' '), |s| {
        s.pop_layer();
        show_lost(s);
    });

    s.add_global_callback(Event::Char('f'), |s| {
        s.pop_layer();
        show_won(s);
    });

    s.add_global_callback(Event::Char('c'), |s| {
        s.pop_layer();
        show_won(s);
    });
}

// returns the current value of the EditView having the given name
fn get_editview_as<T: std::str::FromStr>(s: &mut Cursive, name: &str, default: T) -> T {
    s.call_on_name(name, |view: &mut EditView| view.get_content())
        .unwrap()
        .parse::<T>()
        .unwrap_or(default)
}
