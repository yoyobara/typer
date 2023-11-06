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
    keypad(stdscr(), true);

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

    wborder(input_window, 0, 0, 0, 0, 0, 0, 0, 0);
    wborder(text_window, 0, 0, 0, 0, 0, 0, 0, 0);

    wrefresh(input_window);
    wrefresh(text_window);

    (input_window, text_window)
}

fn load_multiline() -> MultiLine {
    let line_length = COLS() - 2; // border duh
    let text = "hello world my name is yotam and I love cookies I work at nistec and I dont know what am I doing with my life.".to_string();

    let m = MultiLine::new(text, line_length);
    m
}

fn main() {
    init();
    let (_input_window, text_window) = initialize_windows();

    let multiline = load_multiline();
    let multiline_index = 0;

    loop {

        match getch() {

            // f1 or esc
            KEY_F1 | 27 => {
                break;
            }

            // whitespace
            32 => {
            }
            
            // rest of characters
            _ => {

            }
        }

    }

    delwin(_input_window);
    delwin(text_window);
    endwin();
}
