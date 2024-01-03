use ncurses::{*, ll::{mv, refresh}};
const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

fn main() {
    initscr();

    let mut quit = false;
    let todos = vec!["No 1","No 2","No 3"];
    let mut todo_curr = 0;

    while !quit{
        unsafe{
            for (index, todo) in todos.iter().enumerate(){
                if todo_curr==index{
                    // We should be rendering a different style to indicate that the current item
                    // is selected

                }
                mv(index as i32, 1);
                addstr(*todo);
            }
            refresh();
            let key = getch();
            match key as u8 as char{
                'q' => quit = true,
                _=> {}
            }
        }
    }

    endwin();
}
