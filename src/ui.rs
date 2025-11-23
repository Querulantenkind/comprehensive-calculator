use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crate::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.area());

    // History
    let history_items: Vec<ListItem> = app
        .history
        .iter()
        .map(|item| {
            let style = if item.is_error {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::White)
            };
            
            let content = format!("{} = {}", item.expression, item.result);
            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let history_list = List::new(history_items)
        .block(Block::default().borders(Borders::ALL).title("History"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan))
        .highlight_symbol("> ");

    f.render_stateful_widget(history_list, chunks[0], &mut app.history_state);

    // Input
    let input = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(input, chunks[1]);

    if app.show_help {
        let block = Block::default().title("Help").borders(Borders::ALL);
        let area = centered_rect(60, 50, f.area());
        f.render_widget(Clear, area); // Clear background
        
        let text = Text::from(vec![
            Line::from(vec![Span::styled("Comprehensive Calculator", Style::default().add_modifier(Modifier::BOLD))]),
            Line::from(""),
            Line::from(vec![Span::styled("Controls:", Style::default().fg(Color::Cyan))]),
            Line::from("  Enter      Evaluate expression / Run command"),
            Line::from("  Up/Down    Scroll history"),
            Line::from("  Ctrl+y     Copy selected result to clipboard"),
            Line::from("  ? / F1     Toggle help"),
            Line::from("  Esc        Quit"),
            Line::from(""),
            Line::from(vec![Span::styled("Commands:", Style::default().fg(Color::Cyan))]),
            Line::from("  :clear, :c Clear history"),
            Line::from("  :help, :h  Show this help"),
            Line::from("  :quit, :q  Exit"),
        ]);
        
        let p = Paragraph::new(text)
            .block(block)
            .wrap(Wrap { trim: true });
            
        f.render_widget(p, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
