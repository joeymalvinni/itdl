use std::io::{Stdout, stdout};

use crossterm::{terminal, cursor::MoveTo, execute, style::{Print, ResetColor, SetBackgroundColor, Color, SetForegroundColor}};

pub enum Input {
    Down,
    Up, 
    Cycle,
    Edit,
    Insert,
    Append
}

#[derive(PartialEq)]
pub enum Tabs {
    All,
    Todo,
    Done,
}

impl Tabs {
    fn from_index(index: usize) -> Tabs {
        match index {
            0 => Tabs::All,
            1 => Tabs::Todo,
            2 => Tabs::Done,
            _ => unreachable!(), // Handle any other values gracefully
        }
    }
}

pub struct UI {
    pub current_tab: Tabs,
    position: usize, // current selected element
    stdout: Stdout,
}

impl UI {
    pub fn new() -> UI {
        UI {
            current_tab: Tabs::Todo,
            position: 0,
            stdout: stdout(),
        }
    }

    pub fn cycle_tabs(&mut self) {
        self.current_tab = match self.current_tab {
            Tabs::All => Tabs::Todo,
            Tabs::Todo => Tabs::Done,
            Tabs::Done => Tabs::All,
        }
    }

    pub fn change_tab(&mut self, tab: Tabs) {
        self.current_tab = tab;
    }

    pub fn set_position(&mut self, pos: usize) {
        self.position = pos;
    }

    pub fn render_tabs(&mut self) {
        execute!(self.stdout, MoveTo(0, 1), terminal::Clear(terminal::ClearType::FromCursorUp)).unwrap();

        let tabs = ["All", "Todo", " Done"];

        for (i, tab) in tabs.iter().enumerate() {
            if self.current_tab == Tabs::from_index(i) {
                execute!(self.stdout, SetBackgroundColor(Color::White)).unwrap();
                execute!(self.stdout, SetForegroundColor(Color::Black)).unwrap();
            }

            execute!(self.stdout, MoveTo(i as u16 * 6 + 1, 0), Print(tab)).unwrap();

            if self.current_tab == Tabs::from_index(i) {
                execute!(self.stdout, ResetColor).unwrap();
            }
        }
    }

    pub fn render_list(&mut self, list: &Vec<String>) {
        execute!(self.stdout, MoveTo(0, 1), terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
        for (i, item) in list.iter().enumerate() {
            let line_number = i as u16;
    
            if self.position == i {
                execute!(self.stdout, SetBackgroundColor(Color::White), SetForegroundColor(Color::Black)).unwrap();
            }
    
            execute!(self.stdout, MoveTo(1, line_number + 2), Print(item)).unwrap();
    
            if self.position == i {
                execute!(self.stdout, ResetColor).unwrap();
            }
        }
    }
    
    pub fn handle_input(&mut self, input: Input) {
        match input {
            Input::Up => {
                if self.position > 0 {
                    self.position -= 1;
                }
            },
            Input::Down => {
                self.position += 1;
            },
            _ => {},
        }
    }
}
