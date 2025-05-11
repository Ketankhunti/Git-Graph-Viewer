use ratatui::{
    backend::Backend,
    Frame,
    widgets::{Block, Borders, List, ListItem},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color,Modifier},
    text::{Line},
};    

use ratatui::text::Span;

pub fn draw_ui<B: Backend>(f: &mut Frame, commits: &[(String, String,Vec<String>)]) {
    // Format commit messages with simple graph visuals
    let items: Vec<ListItem> = commits
    .iter()
    .enumerate()
    .map(|(i, (hash, msg, branches))| {
        let graph_line = if i == 0 { "*─" } else { "│ " };

        // Build graph symbol
        let graph_span = Span::styled(graph_line, Style::default().fg(Color::Cyan));

        // Short hash styled bold
        let hash_span = Span::styled(format!("{} ", hash), Style::default().add_modifier(Modifier::BOLD));

        // Commit message
        let msg_span = Span::raw(msg.clone());

        // Branches (if any)
        let branch_span = if !branches.is_empty() {
            let joined = branches
                .iter()
                .map(|b| format!("[{}]", b))
                .collect::<Vec<_>>()
                .join(" ");
            Span::styled(
                format!("  {}", joined),
                Style::default().fg(Color::LightMagenta),
            )
        } else {
            Span::raw("")
        };

        let line = Line::from(vec![graph_span, hash_span, msg_span, branch_span]);

        ListItem::new(line)
    })
    .collect();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(1)].as_ref())
        .split(f.area());

    let list = List::new(items)
        .block(Block::default().title("Git Commit Graph").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    f.render_widget(list, chunks[0]);
}
