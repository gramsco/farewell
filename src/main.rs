use anyhow::Result;
use std::env;
use std::io::copy;

mod scraping;
use scraping::{ScrapingOptions, get_country_by_name, get_countries};

fn main() {
    let args: Vec<String> = env::args().collect();
    let _ = match &args.get(1) {
        Some(country) => get_country_infos(country,ScrapingOptions::default()),
        None => display_list_of_countries()
    };
}

fn download_image(url: &str, filename: &str) -> Result<()> {
    let mut body = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create(format!("{}_farewell.jpg", filename)).unwrap();
    copy(&mut body, &mut file)?;
    Ok(())
}

fn display_list_of_countries() -> Result<()>{
    let countries = get_countries();
    countries.iter().for_each(|country_id| {
        println!("{country_id}");
    });
    Ok(())
}

fn get_country_infos(country: &str, options:ScrapingOptions) -> Result<()> {
    match get_country_by_name(country){
        Ok(infos) => {
            if options.download {
                download_image(&infos.img_src, country)?;
               };
        },
        _ => println!("Could not get country infos.")
    }
    
    Ok(())
}

