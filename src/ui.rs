use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
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
}
