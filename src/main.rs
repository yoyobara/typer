use ncurses:: {
    initscr, addstr, refresh, getch, endwin
};

fn main() {
    initscr();

    addstr("hello world!");

    refresh();
    getch();

    endwin();
}
