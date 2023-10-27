use ncurses:: {
    initscr, addstr, refresh, getch, endwin, start_color, init_pair, COLOR_WHITE, COLOR_BLACK, COLOR_GREEN, COLOR_RED, attron, attroff, COLOR_PAIR
};

const NORMAL_TEXT: i16 = 1;
const GREEN_TEXT: i16 = 2;
const RED_TEXT: i16 = 3;


/*
 * kinda initializes the curses screen, colors, etc
 */
fn init() {
    initscr();
    start_color();
    init_pair(NORMAL_TEXT, COLOR_WHITE, COLOR_BLACK);
    init_pair(GREEN_TEXT, COLOR_GREEN, COLOR_BLACK);
    init_pair(RED_TEXT, COLOR_RED, COLOR_BLACK);
}
fn main() {
    init();

    attron(COLOR_PAIR(RED_TEXT));
    addstr("hello world!");
    attroff(COLOR_PAIR(RED_TEXT));
    
    refresh();
    getch();

    endwin();
}
