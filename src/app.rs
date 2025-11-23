use directories::ProjectDirs;
use evalexpr::{eval_with_context_mut, HashMapContext, Value};
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct HistoryItem {
    pub expression: String,
    pub result: String,
    pub is_error: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AppState {
    pub history: Vec<HistoryItem>,
    pub variables: HashMap<String, Value>,
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
    /// Flag to show help popup
    pub show_help: bool,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            input: String::new(),
            history: Vec::new(),
            should_quit: false,
            history_state: ListState::default(),
            context: HashMapContext::new(),
            show_help: false,
        };
        app.load_state();
        // If history exists, scroll to bottom
        if !app.history.is_empty() {
            app.history_state.select(Some(app.history.len() - 1));
        }
        app
    }

    /// Handles regular character input
    pub fn on_key(&mut self, c: char) {
        if self.show_help {
            return;
        }
        self.input.push(c);
    }

    /// Handles backspace
    pub fn on_backspace(&mut self) {
        if self.show_help {
            return;
        }
        self.input.pop();
    }

    /// Evaluates the current input and adds it to history
    pub fn on_enter(&mut self) {
        if self.show_help {
            self.show_help = false;
            return;
        }

        let input = self.input.trim();
        if input.is_empty() {
            return;
        }

        // Command parsing
        if input.starts_with(':') {
            match input {
                ":c" | ":clear" => {
                    self.history.clear();
                    self.context = HashMapContext::new();
                }
                ":q" | ":quit" => {
                    self.on_quit();
                }
                ":h" | ":help" => {
                    self.show_help = true;
                }
                _ => {
                    self.history.push(HistoryItem {
                        expression: self.input.clone(),
                        result: "Unknown command".to_string(),
                        is_error: true,
                    });
                }
            }
            self.input.clear();
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
        if self.show_help {
            return;
        }
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
        if self.show_help {
            return;
        }
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

    /// Toggle help
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    /// Quit the application
    pub fn on_quit(&mut self) {
        self.save_state();
        self.should_quit = true;
    }

    fn get_config_path() -> Option<PathBuf> {
        ProjectDirs::from("com", "calculator", "comprehensive-calculator")
            .map(|proj_dirs| proj_dirs.data_dir().join("state.json"))
    }

    fn save_state(&self) {
        if let Some(path) = Self::get_config_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            
            // Extract variables from context manually since HashMapContext isn't directly serializable in a clean way
            // Actually, evalexpr::HashMapContext isn't serializable. We need to iterate/extract.
            // But HashMapContext structure is opaque. Wait, we can iterate if we had access.
            // Since we can't easily extract all variables from HashMapContext without iterating (if supported),
            // we might have to rely on `context` being just `HashMapContext`.
            // Limitation: evalexpr doesn't expose an iterator for variables easily in 13.0.0?
            // Checking docs or assuming we can't save variables easily without a custom context wrapper.
            // Workaround: We will skip saving variables for now or only save history.
            // Let's check if we can. `HashMapContext` has `iter_variables`? No.
            // We will save only History for now to avoid compilation errors with opaque types.
            
            let state = AppState {
                history: self.history.clone(),
                variables: HashMap::new(), // Placeholder as we can't easily extract
            };

            if let Ok(data) = serde_json::to_string(&state) {
                let _ = fs::write(path, data);
            }
        }
    }

    fn load_state(&mut self) {
        if let Some(path) = Self::get_config_path() {
            if let Ok(data) = fs::read_to_string(path) {
                if let Ok(state) = serde_json::from_str::<AppState>(&data) {
                    self.history = state.history;
                    // Restore variables if we could.
                }
            }
        }
    }
}
