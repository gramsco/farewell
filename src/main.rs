use anyhow::Result;
use scraper::{Html, Selector};
use std::env;
use std::io::copy;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _ = match &args.get(1) {
        Some(country) => go_to_url(country),
        None => {
            println!("Please provide a country");
            println!("ie: cargo run colombie");
            display_list_of_countries()
        }
    };
}

fn fetch_img(url: &str, filename: &str) -> Result<()> {
    let mut body = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create(format!("{}.jpg", filename)).unwrap();
    copy(&mut body, &mut file)?;
    println!("{url}");
    Ok(())
}

static BASE: &str = "https://www.diplomatie.gouv.fr/";
static PATHNAME: &str = "fr/conseils-aux-voyageurs/conseils-par-pays-destination/";

fn display_list_of_countries() -> Result<()> {
    println!("You may use one of the following options: ");
    println!("");
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
    options.chunks(8).for_each(|chunk| {
        chunk.iter().for_each(|country| print!("{country}"));
        println!("");
    });

    Ok(())
}

fn go_to_url(country: &str) -> Result<()> {
    let url = format!("{}{}/{country}#securite", &BASE, &PATHNAME);
    let body = reqwest::blocking::get(url)?.text()?;

    let html = Html::parse_document(&body);
    let selector = Selector::parse("img").unwrap();

    for img in html.select(&selector) {
        let img_src = img.attr("src").unwrap();
        if img_src.contains(country) {
            let full_url = format!("https://www.diplomatie.gouv.fr/{img_src}");
            fetch_img(&full_url, country)?;
        }
    }

    Ok(())
}
