use ratatui::{
    backend::Backend,
    Frame,
    widgets::{Block, Borders, List, ListItem},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
};

pub fn draw_ui<B: Backend>(f: &mut Frame, commits: &[(String, String)]) {
    let items: Vec<ListItem> = commits
        .iter()
        .map(|(hash, msg)| ListItem::new(format!("{hash}: {msg}")))
        .collect();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(1)].as_ref())
        .split(f.area());

    let list = List::new(items)
        .block(Block::default().title("Git Commits").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    f.render_widget(list, chunks[0]);
}
