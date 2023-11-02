mod multiline;

use ncurses::*;
use multiline::MultiLine;

const NORMAL_TEXT: i16 = 1;
const GREEN_TEXT: i16 = 2;
const RED_TEXT: i16 = 3;

/*
 * kinda initializes the curses screen, colors, etc
 */
fn init() {
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    noecho();

    start_color();
    init_pair(NORMAL_TEXT, COLOR_WHITE, COLOR_BLACK);
    init_pair(GREEN_TEXT, COLOR_GREEN, COLOR_BLACK);
    init_pair(RED_TEXT, COLOR_RED, COLOR_BLACK);

    refresh();
}

/*
 * initializes and draw the input window and the text window.
 */
fn initialize_windows() -> (WINDOW, WINDOW) {
    let input_window = newwin(3, COLS(), 0, 0);
    let text_window = newwin(LINES() - 3, COLS(), 3, 0);

    box_(input_window, 0, 0);
    box_(text_window, 0, 0);

    wrefresh(input_window);
    wrefresh(text_window);

    (input_window, text_window)
}

fn load_text() -> MultiLine {
    let line_length = COLS() - 2; // border duh
    let text = "hello world my name is yotam and I love cookies I work at nistec and I dont know what am I doing with my life.".to_string();

    let m = MultiLine::new(text, line_length);
    m
}

fn main() {
    init();
    let (input_window, text_window) = initialize_windows();

    getch();
    let m = load_text();

    for i in 0..m.len() {
        let sc = m.get(i);
        let (y, x) = sc.position;

        wattron(text_window, COLOR_PAIR(sc.color));
        mvwaddch(text_window, y + 1, x + 1, sc.character as u32);
    }
    wrefresh(text_window);

    getch();

    delwin(input_window);
    delwin(text_window);
    endwin();
}
