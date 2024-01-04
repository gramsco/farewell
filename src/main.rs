use anyhow::Result;
use scraper::{Html, Selector, Element};
use std::env;
use std::io::copy;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _ = match &args.get(1) {
        Some(country) => go_to_url(country),
        None => display_list_of_countries()
    };
}

fn fetch_img_by_url(url: &str, filename: &str) -> Result<()> {
    let mut body = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create(format!("{}_farewell.jpg", filename)).unwrap();
    copy(&mut body, &mut file)?;
    Ok(())
}

static BASE: &str = "https://www.diplomatie.gouv.fr/";
static PATHNAME: &str = "fr/conseils-aux-voyageurs/conseils-par-pays-destination/";

fn display_list_of_countries() -> Result<()> {
    let url = format!("{}{}", &BASE, PATHNAME);
    let body = reqwest::blocking::get(&url)?.text()?;
    let html = Html::parse_document(&body);
    let selector = Selector::parse("#recherche_pays").unwrap();
    let option = Selector::parse("option").unwrap();
    let mut options: Vec<String> = vec![];
    for s in html.select(&selector) {
        for option in s.select(&option) {
            let x = option.attr("value").unwrap_or("Nada");
            if x != PATHNAME {
                let o = format!("{} ", &x[PATHNAME.len()..x.len() - 1]);
                options.push(o);
            }
        }
    }
    options.iter().for_each(|country| {
     	println!("{country}");
    });

    Ok(())
}

fn go_to_url(country: &str) -> Result<()> {
    let url = format!("{}{}/{country}#securite", &BASE, &PATHNAME);
    let body = reqwest::blocking::get(url)?.text()?;
    let html = Html::parse_document(&body);
    let mediabox_selector = Selector::parse(".mediabox").unwrap();
    let mediabox = html.select(&mediabox_selector).nth(0).expect("Mediabox not found.");
    let img = mediabox.first_element_child().expect("No image in the mediabox");
    let img_src = img.attr("src").expect("There should be a src to this image. I'm smart, I know html.");
    let full_url = format!("https://www.diplomatie.gouv.fr/{img_src}");
    fetch_img_by_url(&full_url, country)
}
