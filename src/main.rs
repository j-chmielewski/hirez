use clap::{App, Arg, ArgMatches};
use colored::*;
use scraper::{ElementRef, Html, Selector};
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

fn configure_clap<'a>() -> ArgMatches<'a> {
    App::new("hirez")
        .author("Jacek Chmielewski <jchmielewski@teonite.com>")
        .version(env!("CARGO_PKG_VERSION"))
        .args(&[
            Arg::with_name("game").help("If present, displays status for this game only."),
            Arg::with_name("compact")
                .long("compact")
                .short("c")
                .help("Compact mode, displays aggregated platforms."),
        ])
        .get_matches()
}

fn text_from_selector(element: &ElementRef, selector: &str) -> String {
    let untrimmed = element
        .select(&Selector::parse(selector).unwrap())
        .next()
        .unwrap()
        .inner_html();
    String::from(untrimmed.trim())
}

fn main() {
    let matches = configure_clap();
    let document = reqwest::blocking::get(URL)
        .expect("Failed to fetch Hi-Rez status website.")
        .text()
        .unwrap();
    let parsed = Html::parse_document(&document);
    let group_selector = Selector::parse("div.component-container.is-group").unwrap();
    let platform_selector =
        Selector::parse("div.child-components-container > div.component-inner-container").unwrap();
    println!();
    for group in parsed.select(&group_selector) {
        let group_name = text_from_selector(&group, "span.name > span:nth-child(2)");
        if let Some(game) = matches.value_of("game") {
            if !group_name.to_lowercase().contains(&game.to_lowercase()) {
                continue;
            }
        }
        if matches.is_present("compact") {
            let status = text_from_selector(&group, "span.component-status");
            println!("{:20} {}", group_name, color_status(&status));
        } else {
            for platform in group.select(&platform_selector) {
                let platform_name = text_from_selector(&platform, "span.name");
                let platform_status = text_from_selector(&platform, "span.component-status");
                println!("  {:20} {}", platform_name, color_status(&platform_status));
            }
            println!();
        }
    }
}
