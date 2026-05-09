use crate::app::App;
use crate::app::View;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

// drawing lobste.rs pages
pub fn draw(f: &mut Frame, app: &App) {
    match &app.view {
        View::Stories => draw_stories(f, app),
        View::Article(text) => draw_article(f, text),
    }
}

fn draw_stories(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // header
    let header = Paragraph::new(app.page.clone())
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // story list
    let items: Vec<ListItem> = app
        .stories
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let tags = s.tags.join(" ");
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(
                        format!("{:>2}. ", i + 1),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(
                        s.title.clone(),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(Span::styled(
                    format!(
                        "     ▲ {}  🗪  {}  @{}  {}",
                        s.score, s.comment_count, s.submitter_user, tags
                    ),
                    Style::default().fg(Color::DarkGray),
                )),
            ])
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.selected));

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Front Page"))
        .highlight_style(
            Style::default()
                .bg(Color::Red)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, chunks[1], &mut state);

    // footer
    let footer =
        Paragraph::new("| j/k  move |  | enter  open story |  | c  open comments |  | q  quit |")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);
}

// drawing story articles from html
fn draw_article(f: &mut Frame, text: &str) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Article — Esc to go back");
    let para = Paragraph::new(text)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(para, f.area());
}
