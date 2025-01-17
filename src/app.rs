use std::error;



use crate::article::Article;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;


#[derive(Debug)]
pub struct AppArea {
    pub width: usize, 
    pub height: usize
}


#[derive(Debug)]
pub struct AppDetail {
   pub article: Article,
   pub content: String, 
   pub scroll_index: usize
}


#[derive(Debug)]
pub enum AppState {
    Normal, 
    Detail(AppDetail),
    Jump(usize)
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub articles: Vec<Article>,
    pub selected_article_index: usize, 
    pub area: AppArea,
    pub last_update_timestamp: i64,
    pub mode: AppState 
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            articles: Vec::new(),
            selected_article_index: 0,
            mode: AppState::Normal,
            area: AppArea { width: 0, height: 0 }, 
            last_update_timestamp: 0,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub async fn tick(&mut self) {}
    
   /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
