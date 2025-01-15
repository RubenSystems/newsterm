pub mod article;
pub mod feedloader;

use article::Article;
use chrono::{Utc, TimeZone};
use feedloader::{download_feeds, parse_atom_feed, parse_rss_feed, Feed, FeedType};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, process::Command};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Row, Table},
    Terminal,
};


fn time_ago(timestamp: i64) -> String {
    // Convert the provided timestamp (assumed to be in seconds) to a datetime
    let time = chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0)
        .map(|t| chrono::DateTime::<Utc>::from_utc(t, Utc))
        .unwrap_or_else(|| Utc.timestamp(timestamp, 0));
    let now = Utc::now();

    // Calculate the duration between now and the provided time
    let duration = now - time;

    if duration.num_seconds() < 60 {
        format!("{}s", duration.num_seconds())
    } else if duration.num_minutes() < 60 {
        format!("{}m", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}hr", duration.num_hours())
    } else if duration.num_days() < 7 {
        format!("{}d", duration.num_days())
    } else if duration.num_days() < 30 {
        format!("{}wk", duration.num_days() / 7)
    } else if duration.num_days() < 365 {
        format!("{} months", duration.num_days() / 30)
    } else {
        format!("{}yr", duration.num_days() / 365)
    }
}



#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let feeds = vec![
        Feed {
            url: "http://feeds.bbci.co.uk/news/rss.xml".into(),
            feed_type: FeedType::RSS,
            name: "BBC News".into(),
        },
        Feed {
            url: "https://feeds.theguardian.com/theguardian/uk/rss".into(),
            feed_type: FeedType::RSS,
            name: "The Guardian".into(),
        },
        Feed {
            url: "https://www.theverge.com/rss/index.xml".into(),
            feed_type: FeedType::Atom,
            name: "The Verge".into(),
        },
        Feed {
            url: "https://www.wired.com/feed/rss".into(),
            feed_type: FeedType::RSS,
            name: "WIRED".into(),
        },
    ];
    let mut downloaded: Vec<(Feed, Article)> = download_feeds(feeds)
        .await
        .into_iter()
        .map(|(feed, content)| {
            let parsed_feed = match feed.feed_type {
                FeedType::RSS => parse_rss_feed(&content),
                FeedType::Atom => parse_atom_feed(&content),
            };

            match parsed_feed {
                Some(v) => Some(v.into_iter().map(move |x| (feed.clone(), x))),
                None => None,
            }
        })
        .filter_map(|x| x)
        .flat_map(|x| x)
        .collect();

    downloaded.sort_by(|x, y| y.1.date.cmp(&x.1.date));
    Ok(())
}
