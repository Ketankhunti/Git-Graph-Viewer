use ratatui::{
    backend::Backend,
    Frame,
    widgets::{Block, Borders, List, ListItem},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color, Modifier},
    text::{Line, Span},
};

pub struct CommitInfo {
    pub hash: String,
    pub message: String,
    pub branches: Vec<String>,
    pub author: String,
    pub date: String,
}

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
    let total = commits.len();
    let visible_height = area.height.saturating_sub(2) as usize; // 2 for top/bottom border

    // Clamp scroll_offset so that the window doesn't go out of bounds
    let max_scroll = total.saturating_sub(visible_height);
    let scroll_offset = scroll_offset.min(max_scroll);

    // Always show the selected commit
    let scroll_offset = if selected < scroll_offset {
        selected
    } else if selected >= scroll_offset + visible_height {
        selected + 1 - visible_height
    } else {
        scroll_offset
    };

    let end = (scroll_offset + visible_height).min(total);
    let visible_commits = &commits[scroll_offset..end];

    let items: Vec<ListItem> = visible_commits
        .iter()
        .enumerate()
        .map(|(i, (hash, msg, branches))| {
            let absolute_index = scroll_offset + i;

            // Example graph line (for demo, you can improve this logic)
            let graph_line = if absolute_index == 0 {
                "*─"
            } else if absolute_index % 2 == 0 {
                "│ "
            } else {
                "╰─"
            };
            let graph_span = Span::styled(graph_line, Style::default().fg(Color::Cyan));

            // Branch labels
            let branch_labels = if !branches.is_empty() {
                let joined = branches.iter().map(|b| format!("[{}]", b)).collect::<Vec<_>>().join(" ");
                Span::styled(joined, Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
            } else {
                Span::raw("")
            };

            // Short hash styled bold cyan
            let hash_span = Span::styled(
                format!("{}", hash),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            );

            // Commit message
            let msg_span = Span::styled(msg.clone(), Style::default().fg(Color::White));

            // Compose the advanced row
            let line = Line::from(vec![
                graph_span,
                Span::raw(" "),
                branch_labels,
                Span::raw(" "),
                hash_span,
                Span::raw(" "),
                msg_span,
            ]);

            let mut item = ListItem::new(line);
            if absolute_index == selected {
                item = item.style(Style::default().bg(Color::Cyan).fg(Color::Black));
            }

            item
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Git Commit Graph").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    f.render_widget(list, area);
}
