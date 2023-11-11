mod multiline;

use circular_queue::CircularQueue;
use ncurses::*;
use multiline::MultiLine;
use std::time::{ Instant, Duration };

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
    mvwaddstr(input_window, 0, 1, "INPUT");

    wattron(text_window, COLOR_PAIR(COLOR_GREEN));
    wborder(text_window, 0, 0, 0, 0, 0, 0, 0, 0);
    mvwaddstr(text_window, 0, 1, "TEXT");
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

fn draw(m: &MultiLine, multiline_index: usize, input_queue: &CircularQueue<u32>, text_window: WINDOW, input_window: WINDOW) {

    // draw characters
    for i in 0..m.len() {
        let sc = m.get(i);

        wattron(text_window, COLOR_PAIR(
            if i == multiline_index {HIGHLIGHTED_NORMAL_TEXT} else {sc.color})
        );

        mvwaddch(text_window, 1+sc.position.0, 1+sc.position.1, sc.character);
    }
    wrefresh(text_window);

    // clear input window
    for i in 0..(getmaxx(input_window) - 2) {
        mvwaddch(input_window, 1, i + 1, 32);
    }

    // draw input window
    for (i, ch) in input_queue.iter().rev().enumerate() {
        mvwaddch(input_window, 1, 1+i as i32, *ch);
    }
    wrefresh(input_window);
}

fn calculate_wpm(m: &MultiLine, dur: Duration) -> f64 {
    getch();
    let minutes_elapsed = dur.as_secs_f64() / 60.0;
    m.count_full_words() as f64 / minutes_elapsed
}

fn main() {
    init();
    let (input_window, text_window) = initialize_windows();

    let mut multiline = load_multiline(getmaxx(text_window) - 2);
    let mut multiline_index = 0;

    let mut input_queue: CircularQueue<u32> = CircularQueue::with_capacity((getmaxx(input_window) - 2) as usize);
    let mut finished = false;

    let mut wpm: Option<f64> = None;
    let mut start_time: Option<Instant> = None;

    draw(&multiline, multiline_index, &input_queue, text_window, input_window);
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

                // if space add to words count, clear queue.
                if ch == 32 {
                    input_queue.clear();
                } else {
                    input_queue.push(ch as u32);
                }

                if multiline_index == multiline.len() {
                    finished = true;   
                }

            }
        }
        
        // check if started rn
        if start_time.is_none() {
            start_time = Some(Instant::now());
        }

        // draw screen
        draw(&multiline, multiline_index, &input_queue, text_window, input_window);
    
        // if finish calculate wpm and break
        if finished {
            wpm = Some(calculate_wpm(&multiline, start_time.unwrap().elapsed()));
            break;
        };
    }

    delwin(input_window);
    delwin(text_window);
    endwin();
    
    if finished {
        println!("SUCCESS with {} words per minute!", wpm.unwrap());
    }
}
