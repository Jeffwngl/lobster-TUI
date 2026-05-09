use crate::api::Comment;
use crate::app::App;
use crate::app::View;
use crate::utils::strip_html;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub fn draw(f: &mut Frame, app: &App) {
    if app.loading {
        draw_loading(f);
        return;
    }
    match &app.view {
        View::Stories => draw_stories(f, app),
        View::Article(text) => draw_article(f, text),
        View::Comments(comments) => draw_comments(f, app, comments),
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
    let header = Paragraph::new("Lobste.rs - Hottest")
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
        Paragraph::new("| j/k  move |  | enter open story url |  | o open story in tui |  | v open comments in tui |  | c  open comments url |  | q  quit |")
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

fn draw_comments(f: &mut Frame, app: &App, comments: &[Comment]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = app
        .stories
        .get(app.selected)
        .map(|s| s.title.as_str())
        .unwrap_or("Comments");

    // header
    let header = Paragraph::new(format!("Lobste.rs - Comments - {}", title))
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = comments
        .iter()
        .map(|c| {
            // indent based on nesting level
            let indent = "  ".repeat(c.depth as usize);

            let meta = Line::from(vec![
                Span::raw(indent.clone()),
                Span::styled(
                    format!("@{}  ▲{}  {}", c.commenting_user, c.score, c.created_at),
                    Style::default().fg(Color::Red),
                ),
            ]);

            // wrap comment text with indent
            let body = Line::from(vec![
                Span::raw(indent),
                Span::styled(
                    strip_html(&c.comment).replace('\n', " "),
                    Style::default().fg(Color::White),
                ),
            ]);

            ListItem::new(vec![meta, body, Line::from("")])
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.comment_selected));

    // draw comments
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, chunks[1], &mut state);

    // footer
    let footer = Paragraph::new(" j/k  scroll    Esc  back to stories")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);
}

fn draw_loading(f: &mut Frame) {
    let area = f.area();

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Length(3),
            Constraint::Percentage(45),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(20),
            Constraint::Fill(1),
        ])
        .split(vertical[1]);

    let block = Block::default().borders(Borders::ALL);
    let text = Paragraph::new("Loading...")
        .block(block)
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Red));

    f.render_widget(text, horizontal[1]);
}
