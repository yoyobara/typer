mod multiline;

use circular_queue::CircularQueue;
use ncurses::*;
use multiline::MultiLine;

const NORMAL_TEXT: i16 = 1;
const GREEN_TEXT: i16 = 2;
const RED_TEXT: i16 = 3;
const HIGHLIGHTED_NORMAL_TEXT: i16 = 4;

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
    init_pair(HIGHLIGHTED_NORMAL_TEXT, COLOR_BLACK, COLOR_WHITE);
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

    wattron(text_window, COLOR_PAIR(COLOR_GREEN));
    wborder(text_window, 0, 0, 0, 0, 0, 0, 0, 0);
    wattroff(text_window, COLOR_PAIR(COLOR_GREEN));

    wrefresh(input_window);
    wrefresh(text_window);

    (input_window, text_window)
}

fn load_multiline(line_length: i32) -> MultiLine {
    let text = "hello world my name is yotam and I love cookies I work at nistec and I dont know what am I doing with my life.".to_string();

    let m = MultiLine::new(text, line_length);
    m
}

fn draw(m: &MultiLine, multiline_index: usize, win: WINDOW) {

    // draw characters
    for i in 0..m.len() {
        let sc = m.get(i);

        wattron(win, COLOR_PAIR(if i == multiline_index {HIGHLIGHTED_NORMAL_TEXT} else {sc.color}));

        mvwaddch(win, 1+sc.position.0, 1+sc.position.1, sc.character);
    }

    wrefresh(win);
}

fn main() {
    init();
    let (_input_window, text_window) = initialize_windows();
    let line_length: i32 = COLS() - 2;

    let mut multiline = load_multiline(line_length);
    let mut multiline_index = 0;

    let mut input_queue: CircularQueue<u32> = CircularQueue::with_capacity(line_length as usize);

    let mut finished = false;

    draw(&multiline, multiline_index, text_window);
    loop {

        match getch() {

            // f1 or esc
            KEY_F1 | 27 => {
                break;
            }

            KEY_BACKSPACE => {
                if multiline_index != 0 {
                    multiline_index -= 1;
                    multiline.get_mut(multiline_index).color = NORMAL_TEXT;
                }
            }
            
            // rest of characters
            ch => {
                let screen_char = multiline.get_mut(multiline_index);
                screen_char.color = if ch as u32 == screen_char.character {GREEN_TEXT} else {RED_TEXT};

                multiline_index += 1;

                if multiline_index == multiline.len() {
                    finished = true;   
                }
            }
        }
        draw(&multiline, multiline_index, text_window);
        if finished {break;};
    }

    delwin(_input_window);
    delwin(text_window);
    endwin();
    
    if finished {
        println!("SUCCESS!!");
    }
}
