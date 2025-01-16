use std::process::Command;
use chrono::Utc;
use crate::{app::{App, AppDetail, AppResult, AppState}, feedloader::{Feed, download_feeds, download_article_detail, parse_article_detail, parse_rss_feed, parse_atom_feed}, article::Article};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn clean(string: &str) -> String {
    string.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn find_line_with_substring(big_string: &str, substring: &str) -> usize {
    // Iterate through each line
    let clean_sub: String = clean(substring);
    for (idx, line) in big_string.lines().enumerate() {
        if clean(line).contains(&clean_sub) {
            return idx + 1; 
        }
    }
    0 
}

async fn download_feed(app: &mut App) {
    let feeds = vec![
        Feed {
            url: "http://feeds.bbci.co.uk/news/rss.xml".into(),
            name: "BBC News".into(),
        },
        Feed {
            url: "https://feeds.theguardian.com/theguardian/uk/rss".into(),
            name: "The Guardian".into(),
        },
        Feed {
            url: "https://www.theverge.com/rss/index.xml".into(),
            name: "The Verge".into(),
        },
        Feed {
            url: "https://www.wired.com/feed/rss".into(),
            name: "WIRED".into(),
        },
        Feed {
            url: "https://www.quantamagazine.org/feed/".into(),
            name: "Quanta".into(),
        },

    ];


    let mut downloaded: Vec<(Feed, Article)> = download_feeds(feeds)
     .await
     .into_iter()
     .map(|(feed, content)| {
         let rss_parse = parse_rss_feed(&content);
         let parsed_feed = match rss_parse {
             Some(v) => Some(v), 
             None => parse_atom_feed(&content)
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
     app.articles = downloaded;
     app.last_update_timestamp = Utc::now().timestamp();

}

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Char('q')  => {
            match app.mode {
                AppState::Normal => app.quit(), 
                _ => app.mode = AppState::Normal
            }
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        },
        KeyCode::Char('j')  => {
            match &mut app.mode {
                AppState::Normal => {
                    let temp: i64 = app.selected_article_index as i64;  
                    app.selected_article_index = (temp + 1).min(app.articles.len() as i64 - 1) as usize
                }
                AppState::Detail(di) => {
                    di.scroll_index += 1
                }
                _ => {}
            }
        },
        KeyCode::Char('k') => {
            match &mut app.mode { 
                AppState::Normal => {
                    let temp: i64 = app.selected_article_index as i64;
                    app.selected_article_index = (temp - 1).max(0) as usize;
                },
                AppState::Detail(di) => {
                    let tmp = (di.scroll_index as i64 - 1).max(0);
                    di.scroll_index = tmp as usize
                }
                _ => {}
            }
        },
        KeyCode::Char('d') => {
            if let AppState::Detail(di) = &mut app.mode {
                di.scroll_index += 0
            }
        }
        KeyCode::Char('o') => {
            let _ = Command::new("open")
                .arg("-a")
                .arg("Safari")
                .arg(app.articles[app.selected_article_index].1.link.clone())
                .output();
        },
        KeyCode::Char('r') => {
            app.selected_article_index = 0;
            download_feed(app).await;
        }
        KeyCode::Enter => {
            match app.mode {
                AppState::Normal => {
                    let article = app.articles[app.selected_article_index].1.clone();
                    let content = download_article_detail(&article)
                        .map_or(None, |x| parse_article_detail(&x, app.area.width)).unwrap_or("Could not download article".to_string());
                    let scroll_index = find_line_with_substring(&content, &article.title);
                    app.mode = AppState::Detail(AppDetail { article, content, scroll_index });
                }
                AppState::Jump(cv) => {
                    app.selected_article_index = cv;
                    app.mode = AppState::Normal;
                }
                _ => {}
            }
        }
        KeyCode::Backspace => {
            match app.mode {
                AppState::Jump(cv) => {
                    app.mode = AppState::Jump(cv / 10); 
                },
                _=>{}
            }
        }
        KeyCode::Char(v) if v.is_numeric() => {
            let numeric = (v as usize) - '0' as usize;
            match app.mode {
                AppState::Jump(cv) => {
                    let ncv = cv * 10 + numeric;
                    app.mode = AppState::Jump(ncv);
                }, 
                _ => { app.mode = AppState::Jump(numeric) } 
            }
        }
        _ => {}
    }
    Ok(())
}
