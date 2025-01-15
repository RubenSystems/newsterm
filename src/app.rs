use std::{collections::VecDeque, error};


use chrono::Utc;

use crate::{article::Article, feedloader::Feed};

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


/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub articles: Vec<(Feed, Article)>,
    pub selected_article_index: usize, 
    pub area: AppArea,
    pub last_update_timestamp: i64,
    pub detail: Option<AppDetail>
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            articles: Vec::new(),
            selected_article_index: 0,
            detail: None,
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
    pub async fn tick(&mut self) {
        let current_timestamp = Utc::now().timestamp();
        if current_timestamp - self.last_update_timestamp > 5*60 {
        //    let feeds = vec![
        //        Feed {
        //            url: "http://feeds.bbci.co.uk/news/rss.xml".into(),
        //            feed_type: FeedType::RSS,
        //            name: "BBC News".into(),
        //        },
        //        Feed {
        //            url: "https://feeds.theguardian.com/theguardian/uk/rss".into(),
        //            feed_type: FeedType::RSS,
        //            name: "The Guardian".into(),
        //        },
        //        Feed {
        //            url: "https://www.theverge.com/rss/index.xml".into(),
        //            feed_type: FeedType::Atom,
        //            name: "The Verge".into(),
        //        },
        //        Feed {
        //            url: "https://www.wired.com/feed/rss".into(),
        //            feed_type: FeedType::RSS,
        //            name: "WIRED".into(),
        //        },
        //        Feed {
        //            url: "https://www.quantamagazine.org/feed/".into(),
        //            feed_type: FeedType::RSS,
        //            name: "Quanta".into(),
        //        },

        //    ];


        //    let mut downloaded: Vec<(Feed, Article)> = download_feeds(feeds)
        //     .await
        //     .into_iter()
        //     .map(|(feed, content)| {
        //         let parsed_feed = match feed.feed_type {
        //             FeedType::RSS => parse_rss_feed(&content),
        //             FeedType::Atom => parse_atom_feed(&content),
        //         };

        //         match parsed_feed {
        //             Some(v) => Some(v.into_iter().map(move |x| (feed.clone(), x))),
        //             None => None,
        //         }
        //     })
        //     .filter_map(|x| x)
        //     .flat_map(|x| x)
        //     .filter(|x| x.1.date >= self.last_update_timestamp)
        //     .collect();
        //    
        //    
        //     downloaded.sort_by(|x, y| x.1.date.cmp(&y.1.date));
        //     downloaded.into_iter().for_each(|a| self.articles.push_front(a));
        //     self.last_update_timestamp = current_timestamp;
        }
    }
    
   /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
