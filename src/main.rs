use scraper::{Html, Selector};
use colored::*;
use clap::{App, Arg};
static URL: &str = "http://status.hirezstudios.com/";

fn color_status(status: &str) -> ColoredString {
    // FIXME: can we use smart pattern-matching here?
    let status_lower = status.to_lowercase();
    if status_lower.contains("operational") {
        status.bright_green()
    } else if status_lower.contains("maintenance") {
        status.bright_blue()
    } else if status_lower.contains("degraded") {
        status.bright_yellow()
    } else {
        status.bright_red()
    }
}
fn main() {
    let matches = App::new("hirez")
        .author("Jacek Chmielewski <jchmielewski@teonite.com>")
        .version(env!("CARGO_PKG_VERSION"))
        .args(&[
            Arg::with_name("game").help("If present, displays status for this game only.")
        ])
        .get_matches();
    let game = matches.value_of("game");
    let document = reqwest::blocking::get(URL)
        .expect("Failed to fetch Hi-Rez status website.")
        .text()
        .unwrap();
    let parsed = Html::parse_document(&document);
    let group_selector = Selector::parse("div.component-container.is-group").unwrap();
    let group_name_selector = Selector::parse("span.name > span:nth-child(2)").unwrap();
    let status_selector = Selector::parse("span.component-status").unwrap();
    for group in parsed.select(&group_selector) {
        let _group_name = group.select(&group_name_selector).next().unwrap().inner_html();
        let group_name = _group_name.trim();
        if let Some(game) = game {
            if !group_name.to_lowercase().contains(&game.to_lowercase()) {
                continue;
            }
        }
        let _status = group.select(&status_selector).next().unwrap().inner_html();
        let status = _status.trim();
        println!("{:20} {}", group_name, color_status(status));
    } 
}
