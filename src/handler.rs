use std::process::Command;

use crate::{app::{App, AppDetail, AppResult}, feedloader::{download_article_detail, parse_article_detail}};
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
    let mut latest = 0;
    for (idx, line) in big_string.lines().enumerate() {
        if clean(line).contains(&clean_sub) {
            return idx + 1; 
        }
    }
    latest 
}

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Char('q') if app.detail.is_none() => {
            app.quit();
        }
        KeyCode::Char('q') => {
            app.detail = None
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        },
        KeyCode::Char('j') if app.detail.is_none()  =>{
            let temp: i64 = app.selected_article_index as i64;  
            app.selected_article_index = (temp + 1).min(app.articles.len() as i64 - 1) as usize
        },
        KeyCode::Char('j') => {
            if let Some(di) = &mut app.detail { di.scroll_index += 1 }
        }
        KeyCode::Char('k') if app.detail.is_none() => {
            let temp: i64 = app.selected_article_index as i64;
            app.selected_article_index = (temp - 1).max(0) as usize;
        },
        KeyCode::Char('k') => {
            if let Some(di) = &mut app.detail { 
                let tmp = (di.scroll_index as i64 - 1).max(0);
                di.scroll_index = tmp as usize
            }
        },
        KeyCode::Char('d') => {
            if let Some(di) = &mut app.detail {
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
        KeyCode::Enter => {
            let article = app.articles[app.selected_article_index].1.clone();
            let content = download_article_detail(&article)
                .map_or(None, |x| parse_article_detail(&x, app.area.width)).unwrap_or("Could not download article".to_string());
            let scroll_index = find_line_with_substring(&content, &article.title);
            app.detail = Some(AppDetail { article, content, scroll_index })
        }
        _ => {}
    }
    Ok(())
}
