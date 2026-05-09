use crate::api::{Comment, Story};

pub enum View {
    Stories,
    Article(String),
    Comments(Vec<Comment>),
}

pub struct App {
    pub stories: Vec<Story>,
    pub selected: usize,
    pub view: View,
    pub comment_selected: usize,
    pub loading: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            stories: vec![],
            selected: 0,
            view: View::Stories,
            comment_selected: 0,
            loading: true,
        }
    }

    pub fn next(&mut self) {
        if let View::Comments(comments) = &self.view {
            if self.comment_selected + 1 < comments.len() {
                self.comment_selected += 1;
            }
        } else if self.selected + 1 < self.stories.len() {
            self.selected += 1;
        }
    }

    pub fn prev(&mut self) {
        if let View::Comments(_) = &self.view {
            if self.comment_selected > 0 {
                self.comment_selected -= 1;
            }
        } else if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn open_story(&self) {
        if let Some(s) = self.stories.get(self.selected) {
            let _ = open::that(&s.url);
        }
    }

    pub fn open_comments(&self) {
        if let Some(s) = self.stories.get(self.selected) {
            let _ = open::that(&s.comments_url);
        }
    }
}
