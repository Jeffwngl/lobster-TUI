use crate::api::Story;

pub struct App {
    pub stories: Vec<Story>,
    pub selected: usize,
}

impl App {
    pub fn new(stories: Vec<Story>) -> Self {
        Self {
            stories,
            selected: 0,
        }
    }

    pub fn next(&mut self) {
        if self.selected + 1 < self.stories.len() {
            self.selected += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.selected > 0 {
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
