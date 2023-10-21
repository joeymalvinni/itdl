use crossterm::cursor::{MoveTo, SetCursorStyle};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::style::Print;
use crossterm::{cursor, execute, terminal};

use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io::Stdout;
use std::{io::stdout, time::Duration};

mod todo;
mod ui;
use todo::Todo;
use ui::{Input, Tabs, UI};

const FILE: &str = "TODO.txt";

fn clear_and_render(stdout: &mut Stdout, len: usize, item: &String) {
    execute!(
        stdout,
        MoveTo(0, len as u16 + 2),
        terminal::Clear(terminal::ClearType::FromCursorDown)
    )
    .unwrap();
    execute!(stdout, MoveTo(1, len as u16 + 2), Print("- [ ]")).unwrap();
    execute!(stdout, MoveTo(7, len as u16 + 2), Print(item.to_string())).unwrap();
}

fn remove_last_word(input: &str) -> String {
    let mut words = input.rsplit(' ').collect::<Vec<&str>>();

    if words.len() < 2 {
        return String::from("");
    }

    words.remove(0);

    words.into_iter().rev().collect::<Vec<&str>>().join(" ")
}

fn main() {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    execute!(stdout, EnterAlternateScreen, cursor::MoveTo(0, 0)).unwrap();
    execute!(stdout, cursor::Hide).unwrap();

    let mut todo = Todo::from(FILE);
    let mut ui = UI::new();
    let mut current_list = todo.collect_todo_md();
    let mut accepting_input = false;

    ui.render_tabs();
    ui.render_list(&current_list);

    let mut new_todo_item = String::from("");

    // main input loop
    loop {
        if poll(Duration::from_millis(200)).unwrap() {
            if let Ok(event) = read() {
                // handle inserting a new item
                if accepting_input {
                    match event {
                        Event::Key(KeyEvent {
                            code: KeyCode::Esc,
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            ..
                        }) => {
                            accepting_input = false;
                            execute!(stdout, cursor::Hide).unwrap();
                            execute!(
                                stdout,
                                MoveTo(0, current_list.len() as u16 + 2),
                                terminal::Clear(terminal::ClearType::FromCursorDown)
                            )
                            .unwrap();
                            new_todo_item = String::from("");
                        }
                        Event::Key(KeyEvent {
                            code: KeyCode::Enter,
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            ..
                        }) => {
                            match ui.current_tab {
                                Tabs::Done => {
                                    todo.add_to_done(new_todo_item.clone());
                                    current_list = todo.collect_done_md();
                                }
                                Tabs::Todo => {
                                    todo.add_to_todo(new_todo_item.clone());
                                    current_list = todo.collect_todo_md();
                                }
                                Tabs::All => {
                                    todo.add_to_todo(new_todo_item.clone());
                                    current_list = todo.collect_all_md();
                                }
                            }
                            new_todo_item.clear();
                            accepting_input = false;
                            execute!(stdout, cursor::Hide).unwrap();
                            execute!(
                                stdout,
                                MoveTo(0, current_list.len() as u16 + 2),
                                terminal::Clear(terminal::ClearType::FromCursorDown)
                            )
                            .unwrap();

                            ui.render_list(&current_list);
                        }
                        // handle delete
                        Event::Key(KeyEvent {
                            code: KeyCode::Backspace,
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            ..
                        }) => {
                            if !new_todo_item.is_empty() {
                                new_todo_item.pop();
                                clear_and_render(&mut stdout, current_list.len(), &new_todo_item);
                            }
                        }
                        // handle control delete, delete the last word
                        Event::Key(KeyEvent {
                            code: KeyCode::Backspace,
                            modifiers: KeyModifiers::CONTROL,
                            kind: KeyEventKind::Press,
                            ..
                        }) => {
                            if !new_todo_item.is_empty() {
                                new_todo_item = remove_last_word(&new_todo_item);
                                if new_todo_item.len() > 1 {
                                    new_todo_item.push(' ');
                                }
                                clear_and_render(&mut stdout, current_list.len(), &new_todo_item);
                            }
                        }
                        Event::Key(KeyEvent {
                            code: KeyCode::Char(c),
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            ..
                        }) => {
                            new_todo_item.push(c);
                            clear_and_render(&mut stdout, current_list.len(), &new_todo_item);
                        }
                        Event::Key(KeyEvent {
                            code: KeyCode::Char(c),
                            modifiers: KeyModifiers::SHIFT,
                            kind: KeyEventKind::Press,
                            ..
                        }) => {
                            new_todo_item.push(c.to_ascii_uppercase());
                            clear_and_render(&mut stdout, current_list.len(), &new_todo_item);
                        }
                        _ => {}
                    }
                } else {
                    // match keybinds
                    match event {
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('q'),
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            ..
                        }) => {
                            break; // exit on q
                        }
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('j'),
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            ..
                        }) => {
                            // handle move down
                            ui.handle_input(Input::Down, current_list.len());
                            ui.render_list(&current_list);
                        }
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('k'),
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            ..
                        }) => {
                            // handle move up
                            ui.handle_input(Input::Up, current_list.len());
                            ui.render_list(&current_list);
                        }
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('s'),
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            ..
                        }) => {
                            todo.save(FILE);
                        }
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('a'),
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            ..
                        }) => {
                            // handle append
                            execute!(
                                stdout,
                                MoveTo(1, current_list.len() as u16 + 2),
                                Print("- [ ]")
                            )
                            .unwrap();
                            execute!(
                                stdout,
                                MoveTo(7, current_list.len() as u16 + 2),
                                cursor::Show,
                                SetCursorStyle::BlinkingBar
                            )
                            .unwrap();
                            accepting_input = true;
                        }
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('x'),
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            ..
                        }) => match ui.current_tab {
                            Tabs::Done => {
                                todo.mark_as_todo(ui.position);
                                current_list = todo.collect_done_md();
                                ui.set_position(0);
                                ui.render_list(&current_list);
                            }
                            Tabs::Todo => {
                                todo.mark_as_done(ui.position);
                                current_list = todo.collect_todo_md();
                                ui.set_position(0);
                                ui.render_list(&current_list);
                            }
                            Tabs::All => {
                                let (local_pos, tab) = todo.all_to_single(ui.position);

                                match tab {
                                    Tabs::Todo => {
                                        todo.mark_as_done(local_pos);
                                        ui.set_position(0);
                                        current_list = todo.collect_all_md();
                                    }
                                    Tabs::Done => {
                                        todo.mark_as_todo(local_pos);
                                        ui.set_position(0);
                                        current_list = todo.collect_all_md();
                                    }
                                    _ => {}
                                }
                                ui.render_list(&current_list);
                            }
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
                        _ => {}
                    }
                }
            }
        }
    }

    execute!(stdout, cursor::Show).unwrap();
    execute!(stdout, LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}
