use ratatui::{
    backend::Backend,
    Frame,
    widgets::{Block, Borders, List, ListItem},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color, Modifier},
    text::{Line, Span},
};

pub fn draw_ui<B: Backend>(
    f: &mut Frame,
    commits: &[(String, String, Vec<String>)],
    selected: usize,
    scroll_offset: usize
) {
    // Create layout area with margin
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(1)].as_ref())
        .split(f.area());

    let area = chunks[0];
    let visible_height = area.height.saturating_sub(2) as usize; // 2 for top/bottom border

    let end = (scroll_offset + visible_height).min(commits.len());
    let start = if commits.len() >= visible_height {
        commits.len().saturating_sub(visible_height).min(scroll_offset)
    } else {
        0
    };
    let visible_commits = &commits[start..end];


    let items: Vec<ListItem> = visible_commits
        .iter()
        .enumerate()
        .map(|(i, (hash, msg, branches))| {
            let absolute_index = start + i;

            let graph_line = if absolute_index == 0 { "*─" } else { "│ " };
            let graph_span = Span::styled(graph_line, Style::default().fg(Color::Cyan));
            let hash_span = Span::styled(format!("{} ", hash), Style::default().add_modifier(Modifier::BOLD));
            let msg_span = Span::raw(msg.clone());

            let branch_span = if !branches.is_empty() {
                let joined = branches.iter().map(|b| format!("[{}]", b)).collect::<Vec<_>>().join(" ");
                Span::styled(format!("  {}", joined), Style::default().fg(Color::LightMagenta))
            } else {
                Span::raw("")
            };

            let mut item = ListItem::new(Line::from(vec![
                graph_span,
                hash_span,
                msg_span,
                branch_span,
            ]));

            if absolute_index == selected {
                item = item.style(Style::default().bg(Color::Blue));
            }

            item
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Git Commit Graph").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    f.render_widget(list, area);
}
