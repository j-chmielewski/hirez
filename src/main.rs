use scraper::{Html, Selector};
static URL: &str = "http://status.hirezstudios.com/";

fn main() {
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
        let _status = group.select(&status_selector).next().unwrap().inner_html();
        let status = _status.trim();
        println!("{:20} {}", group_name, status);
    } 
}
