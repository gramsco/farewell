use scraper::{Html, Selector, Element};
use anyhow::Result;

pub struct ScrapingOptions {
    pub download:bool
}

impl ScrapingOptions {
    pub fn default() -> ScrapingOptions {
        ScrapingOptions {
            download:true
        }
    }
}

trait UseCases {
    fn get_country_by_name(&self) -> Result<Country,()>;
    fn get_list_of_countries_ids(&self) -> Vec<String>;
}

pub struct Country {
    pub img_src:String
}

static BASE: &str = "https://www.diplomatie.gouv.fr/";
static PATHNAME: &str = "fr/conseils-aux-voyageurs/conseils-par-pays-destination/";

impl UseCases for Html {

    fn get_country_by_name(&self) -> Result<Country,()> {
        let mediabox_selector = Selector::parse(".mediabox").unwrap();
        let mediabox = self.select(&mediabox_selector).nth(0).expect("Mediabox not found.");
        let img = mediabox.first_element_child().expect("No image in the mediabox");
        let img_src = img.attr("src").expect("There should be a src to this image. I'm smart, I know html.");
        let img_src = format!("https://www.diplomatie.gouv.fr/{img_src}");
        Ok(Country {
            img_src
        })
    }

    fn get_list_of_countries_ids(&self) -> Vec<String> {
        let selector = Selector::parse("#recherche_pays").unwrap();
        let option = Selector::parse("option").unwrap();
        let mut options: Vec<String> = vec![];
        for s in self.select(&selector) {
            for option in s.select(&option) {
                let option_value = option.attr("value").unwrap_or("Nada");
                if option_value != PATHNAME {
                    let opt = format!("{}", &option_value[PATHNAME.len()..option_value.len() - 1]);
                    options.push(opt);
                }
            }
        }
        options  
    }
}

fn get_document(url:&str) -> String {
    return match reqwest::blocking::get(url) {
        Ok(a) => a.text().expect("fatal error"),
        Err(_) => unimplemented!()
    };
}

pub fn get_countries() -> Vec<String> {
    let url = format!("{}{}", &BASE, PATHNAME);
    let document = get_document(&url);
    Html::parse_document(&document).get_list_of_countries_ids()
}

pub fn get_country_by_name(country:&str) -> Result<Country,()> {
    let document = get_document(&format!("{}{}/{country}#securite", &BASE, &PATHNAME));
    Html::parse_document(&document).get_country_by_name()
}