mod api;
mod app;
mod ui;

use app::App;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use crate::api::extract_article;
use crate::app::View;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stories = api::fetch().await?;
    let mut app = App::new(stories);

    enable_raw_mode()?;
    let mut stdout = io::stdout(); // get access to terminal output
    execute!(stdout, EnterAlternateScreen)?; // open a fresh blank screen/buffer
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?; // create terminal backend

    loop {
        // render
        terminal.draw(|f| ui::draw(f, &app))?;

        // listening for inputs
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('j') | KeyCode::Down => app.next(),
                KeyCode::Char('k') | KeyCode::Up => app.prev(),
                KeyCode::Char('o') => {
                    if let Some(s) = app.stories.get(app.selected) {
                        let html = reqwest::get(&s.url).await?.text().await?;
                        if let Some(text) = extract_article(&html, &s.url) {
                            app.view = View::Article(text);
                        }
                    }
                }
                KeyCode::Enter => app.open_story(),
                KeyCode::Char('c') => app.open_comments(),
                KeyCode::Esc => app.view = View::Stories,
                _ => {}
            }
        }
    }

    // exit tui
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
