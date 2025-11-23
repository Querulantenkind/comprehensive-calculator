use evalexpr::eval;

pub struct App {
    /// Current user input string
    pub input: String,
    /// History of calculations (Expression = Result)
    pub history: Vec<String>,
    /// Flag to check if the app should exit
    pub should_quit: bool,
    /// Scroll offset for history list (optional, for future use)
    #[allow(dead_code)]
    pub scroll: usize,
}

impl App {
    pub fn new() -> App {
        App {
            input: String::new(),
            history: Vec::new(),
            should_quit: false,
            scroll: 0,
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

        let result = match eval(&self.input) {
            Ok(v) => v.to_string(),
            Err(_) => "Error".to_string(),
        };

        self.history.push(format!("{} = {}", self.input, result));
        self.input.clear();
    }

    /// Handles tick events (if needed later)
    #[allow(dead_code)]
    pub fn on_tick(&mut self) {}

    /// Quit the application
    pub fn on_quit(&mut self) {
        self.should_quit = true;
    }
}

