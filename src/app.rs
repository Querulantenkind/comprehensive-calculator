use evalexpr::{eval_with_context_mut, HashMapContext};
use ratatui::widgets::ListState;

pub struct HistoryItem {
    pub expression: String,
    pub result: String,
    pub is_error: bool,
}

pub struct App {
    /// Current user input string
    pub input: String,
    /// History of calculations (Expression = Result)
    pub history: Vec<HistoryItem>,
    /// Flag to check if the app should exit
    pub should_quit: bool,
    /// State for history list scrolling
    pub history_state: ListState,
    /// Evaluation context for variables
    pub context: HashMapContext,
}

impl App {
    pub fn new() -> App {
        App {
            input: String::new(),
            history: Vec::new(),
            should_quit: false,
            history_state: ListState::default(),
            context: HashMapContext::new(),
        }
    }

    /// Handles regular character input
    pub fn on_key(&mut self, c: char) {
        self.input.push(c);
    }

    /// Handles backspace
    pub fn on_backspace(&mut self) {
        self.input.pop();
    }

    /// Evaluates the current input and adds it to history
    pub fn on_enter(&mut self) {
        if self.input.trim().is_empty() {
            return;
        }

        let (result_str, is_error) = match eval_with_context_mut(&self.input, &mut self.context) {
            Ok(v) => (v.to_string(), false),
            Err(e) => (format!("Error: {}", e), true),
        };

        self.history.push(HistoryItem {
            expression: self.input.clone(),
            result: result_str,
            is_error,
        });

        // Auto-scroll to bottom
        self.history_state.select(Some(self.history.len() - 1));

        self.input.clear();
    }

    /// Selects the previous item in the history list
    pub fn select_previous(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let i = match self.history_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.history.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.history.len() - 1,
        };
        self.history_state.select(Some(i));
    }

    /// Selects the next item in the history list
    pub fn select_next(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let i = match self.history_state.selected() {
            Some(i) => {
                if i >= self.history.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.history_state.select(Some(i));
    }

    /// Handles tick events (if needed later)
    #[allow(dead_code)]
    pub fn on_tick(&mut self) {}

    /// Quit the application
    pub fn on_quit(&mut self) {
        self.should_quit = true;
    }
}
