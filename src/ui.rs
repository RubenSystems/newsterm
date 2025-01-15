use std::collections::VecDeque;

use chrono::{Utc, TimeZone};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect}, style::{Color, Modifier, Style}, widgets::{Block, BorderType, Cell, Paragraph, Row, Table}, Frame, text::Span
};

use crate::{app::{App, AppArea}, article::Article, feedloader::Feed};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    //
    let area = frame.area();
    app.area = AppArea { width: area.width as usize, height: area.height as usize }; 
    if let Some(dtl) = &app.detail {
        let constraints = vec![Constraint::Percentage(20), Constraint::Percentage(80)];
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(constraints)
            .split(frame.area());
        
        frame.render_widget(
            render_headlines(&app.articles, app.selected_article_index)
            .block(
                Block::bordered()
                    .title("Main Feed")
                    .title_alignment(Alignment::Left)
                    .border_type(BorderType::Rounded),
            ),
            layout[0],
        );

        frame.render_widget(
            render_detail(dtl.content.clone(), dtl.scroll_index)
            .block(
                Block::bordered()
                    .title(dtl.article.title.clone())
                    .title_alignment(Alignment::Left)
                    .border_type(BorderType::Rounded),
            ),
            layout[1],
        );
    } else {
        frame.render_widget(
            render_headlines(&app.articles, app.selected_article_index)
            .block(
                Block::bordered()
                    .title("Main Feed")
                    .title_alignment(Alignment::Left)
                    .border_type(BorderType::Rounded),
            ),
            frame.area(),
        );
    }
}


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

fn render_headlines(articles: &VecDeque<(Feed, Article)>, selected_index: usize) -> ratatui::widgets::Table {
    let widths = vec![
        Constraint::Percentage(3),
        Constraint::Percentage(20),
        Constraint::Percentage(3),
        Constraint::Percentage(74),
    ];
    let rows : Vec<Row> = articles.iter().enumerate().map(|(idx, (feed, article))| {
        if idx == selected_index  {
            Row::new(vec![
                Cell::from(idx.to_string()), 
                Cell::from(feed.name.to_string()), 
                Cell::from(time_ago(article.date)),
                Cell::from(article.title.to_string()).style(Style::default().add_modifier(Modifier::BOLD))
            ]).style(Style::default().bg(Color::Rgb(64, 64, 64)))
        } else {
            Row::new(vec![
                Cell::from(idx.to_string()).style(Style::default().fg(Color::Rgb(128,128,128))), 
                Cell::from(feed.name.to_string()), 
                Cell::from(time_ago(article.date)).style(Style::default().fg(Color::Rgb(128,128,128))),
                Cell::from(article.title.to_string()).style(Style::default().add_modifier(Modifier::BOLD))
            ])
        }
    }).collect();
    Table::new(rows, widths)
}

fn render_detail(detail: String, offset: usize) -> ratatui::widgets::Paragraph<'static> {
    Paragraph::new(detail).wrap(ratatui::widgets::Wrap { trim: false }).scroll((offset as u16, 0))
}



























