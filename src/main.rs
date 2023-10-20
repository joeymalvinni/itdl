use crossterm::cursor::{SetCursorStyle, MoveTo};
use crossterm::style::Print;
use crossterm::{cursor, execute};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use std::{time::Duration, io::stdout};

mod ui;
mod todo;
use todo::Todo;
use ui::{UI, Input, Tabs};

fn main() {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    execute!(stdout, EnterAlternateScreen, cursor::MoveTo(0, 0)).unwrap();
    execute!(stdout, cursor::Hide).unwrap();

    let mut todo = Todo::from("TODO.txt");
    let mut ui = UI::new();
    let mut current_list = todo.collect_todo_md();
    let mut accepting_input = false;

    ui.render_tabs();
    ui.render_list(&current_list);

    // main input loop
    loop {
        if poll(Duration::from_millis(200)).unwrap() {
            if let Ok(event) = read() {
                match event {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        ..
                    }) => {
                        break; // exit on q
                    },
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('j'),
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        ..
                    }) => {
                        // handle move down
                        ui.handle_input(Input::Down);
                        ui.render_list(&current_list);
                    },
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('k'),
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        ..
                    }) => {
                        // handle move up
                        ui.handle_input(Input::Up);
                        ui.render_list(&current_list);
                    },
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('a'),
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        ..
                    }) => {
                        // handle append
                        execute!(stdout, MoveTo(1, current_list.len() as u16 + 2), Print("- [ ]")).unwrap();
                        execute!(stdout, MoveTo(7, current_list.len() as u16 + 2), cursor::Show, SetCursorStyle::BlinkingBar).unwrap();
                        accepting_input = true;
                    },
                    Event::Key(KeyEvent {
                        code: KeyCode::Tab,
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        ..
                    }) => {
                        // handle cycle tabs
                        current_list = match ui.current_tab {
                            Tabs::All => todo.collect_todo_md(),
                            Tabs::Todo => todo.collect_done_md(),
                            Tabs::Done => todo.collect_all_md(),
                        };

                        ui.set_position(0);
                        ui.cycle_tabs();
                        ui.render_list(&current_list);
                        ui.render_tabs();
                    }
                    _ => {},
                }
            }
        }
    }

    execute!(stdout, cursor::Show).unwrap();
    execute!(stdout, LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}
