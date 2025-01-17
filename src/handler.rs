use std::{path::PathBuf, process::Command};
use chrono::Utc;
use dirs::home_dir;
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
    let mut config_path: PathBuf = home_dir().expect("Couldn't find home dir");
    config_path.push(".config/newsterm/feeds");
    let feed_file = std::fs::read_to_string(config_path).unwrap();
    let mut downloaded: Vec<Article> = download_feeds(feed_file.lines().map(|x| Feed { url: x.to_string() } ).collect())
     .await
     .into_iter()
     .map(|(_, content)| {
         let rss_parse = parse_rss_feed(&content);
         let parsed_feed = match rss_parse {
             Some(v) => Some(v), 
             None => parse_atom_feed(&content)
         };

         parsed_feed.map(|v| v.into_iter())
     })
     .flatten()
     .flatten()
     .collect();
    
    
     downloaded.sort_by(|x, y| y.date.cmp(&x.date));
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
        KeyCode::Char('d') => {
            match &mut app.mode {
                AppState::Detail(dtl) if key_event.modifiers == KeyModifiers::CONTROL => {
                    let mn = dtl.scroll_index + (0.8*(app.area.height as f64 - 3.0)) as usize;
                    dtl.scroll_index = mn.min(dtl.content.lines().count());    
                }
                _ => {}
            }
        }
        KeyCode::Char('u') => {
            match &mut app.mode {
                AppState::Detail(dtl) if key_event.modifiers == KeyModifiers::CONTROL => {
                    let mn: i64 = dtl.scroll_index as i64 - (0.8*(app.area.height as f64 - 3.0)) as i64;
                    dtl.scroll_index = mn.max(0) as usize;    
                }
                _ => {}
            }
        }
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
        KeyCode::Char('o') => {
            let _ = Command::new("open")
                .arg("-a")
                .arg("Safari")
                .arg(app.articles[app.selected_article_index].link.clone())
                .output();
        },
        KeyCode::Char('r') => {
            app.selected_article_index = 0;
            app.mode = AppState::Normal;
            download_feed(app).await;
        }
        KeyCode::Enter => {
            match app.mode {
                AppState::Normal => {
                    let article = app.articles[app.selected_article_index].clone();
                    let content = download_article_detail(&article)
                        .map_or(None, |x| parse_article_detail(&x, app.area.width - 3)).unwrap_or("Could not download article".to_string()); // sub the line no
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
            if let AppState::Jump(cv) = app.mode {
                app.mode = AppState::Jump(cv / 10); 
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
