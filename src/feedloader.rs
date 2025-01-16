use std::process::{Stdio, Command};
use std::io::Write;
use crate::article::Article;
use chrono::DateTime;
use futures::future;
use rss::Channel;

fn parse_rfc_2822_date(date: &str) -> i64 {
    DateTime::parse_from_rfc2822(date).map_or(0, |d| d.timestamp())
}

/**
 * Feed
*/
#[derive(Debug, Clone)]
pub struct Feed {
    pub url: String,
    pub name: String,
}

/**
 * Downloader
*/
async fn download_link(url: &str) -> Option<String> {
    let response = reqwest::get(url).await.map_or(None, Some)?;
    let text = response.text().await.map_or(None, Some)?;
    Some(text)
}

pub async fn download_feeds(feeds: Vec<Feed>) -> Vec<(Feed, String)> {
    let tasks: Vec<tokio::task::JoinHandle<(Feed, Option<String>)>> = feeds
        .into_iter()
        .map(|f| {
            tokio::spawn(async move {
                let url = f.url.clone();
                let content = download_link(&url).await;
                (f, content)
            })
        })
        .collect();

    future::join_all(tasks)
        .await
        .into_iter()
        .filter_map(|x| {
            let Ok(x) = x else {
                return None;
            };
            let Some(content) = x.1 else {
                return None;
            };
            Some((x.0, content))
        })
        .collect()
}

/**
 * RSS feed parsers
*/

impl Article {
    fn from_rss_item(rss_item: rss::Item) -> Self {
        Self {
            title: rss_item.title().unwrap_or("").to_string(),
            summary: rss_item.content().unwrap_or("").to_string(),
            date: parse_rfc_2822_date(rss_item.pub_date().unwrap_or("")),
            link: rss_item.link().unwrap_or("").to_string(),
            publisher: rss_item.author().unwrap_or("").to_string(),
        }
    }
}

pub fn parse_rss_feed(content: &str) -> Option<Vec<Article>> {
    let channel = Channel::read_from(content.as_bytes()).map_or(None, Some);
    Some(
        channel?
            .into_items()
            .into_iter()
            .map(Article::from_rss_item)
            .collect(),
    )
}

/**
 * Atom parser
 */

impl Article {
    fn from_atom_item(
        atom_item: atom_syndication::Entry,
        atom_feed: &atom_syndication::Feed,
    ) -> Self {
        Self {
            title: atom_item.title().value.clone(),
            summary: atom_item
                .summary()
                .map_or("".to_string(), |x| x.value.clone()),
            link: atom_item.links().first().unwrap().href.clone(),
            date: atom_item.published().unwrap().timestamp(),
            publisher: atom_feed.title().value.clone(),
        }
    }
}

pub fn parse_atom_feed(content: &str) -> Option<Vec<Article>> {
    let feed = content.parse::<atom_syndication::Feed>().unwrap();

    Some(
        feed.entries()
            .iter()
            .map(|item| Article::from_atom_item(item.clone(), &feed))
            .collect(),
    )
}


/**
 * Article Downloader 
*/


pub fn download_article_detail(article: &Article) -> Result<String, std::io::Error> {
   let res = Command::new("curl").arg(&article.link).output()?;
   let out = String::from_utf8_lossy(&res.stdout);
   Ok(out.to_string())
}

pub fn parse_article_detail(detail: &str, width: usize) -> Option<String> {

    let Ok(mut process) = Command::new("lynx")
        .arg("-stdin") // Lynx flag to read from stdin
        .arg("-dump")  // Lx flag to output plain text
        .arg(format!("-width={width}"))
        .stdin(Stdio::piped())  // Open stdin for writing
        .stdout(Stdio::piped()) // Capture stdout
        .spawn() else {return None};
        


    // Write the HTML content to lynx's stdin
    if let Some(mut stdin) = process.stdin.take() {
        if stdin.write(detail.as_bytes()).is_err() { return None };
    }

    // Capture and print the output from lynx's stdout
    let Ok(output) = process.wait_with_output() else {return None};
    if output.status.success() {
        let response = String::from_utf8_lossy(&output.stdout);
        Some(response.to_string())
    } else {
        println!("Lynx command failed with error:\n{}", 
        String::from_utf8_lossy(&output.stderr));
        None
    }
}





























