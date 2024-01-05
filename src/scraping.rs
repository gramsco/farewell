use anyhow::Result;
use scraper::{Element, Html, Selector};

pub struct ScrapingOptions {
    pub download: bool,
}

impl ScrapingOptions {
    pub fn default() -> ScrapingOptions {
        ScrapingOptions { download: true }
    }
}

pub struct LastMinute {
    pub infos: Vec<Info>,
}

pub struct Info {
    pub title: String,
    pub date: String,
    pub body: String,
}

trait UseCases {
    fn get_country_by_name(&self) -> Result<Safety>;
    fn get_list_of_countries_ids(&self) -> Vec<String>;
    fn get_last_minute_infos(&self) -> Result<LastMinute>;
}

pub struct Safety {
    pub img_src: String,
}

static BASE: &str = "https://www.diplomatie.gouv.fr/";
static PATHNAME: &str = "fr/conseils-aux-voyageurs/conseils-par-pays-destination/";

fn parse_date(s:String) -> String {
    use regex::Regex;

let re = Regex::new(r"(\d{2})/(\d{2})/(\d{4})").unwrap();
let c = re.captures(&s).unwrap();
return c[0].to_string()
}

impl UseCases for Html {
    fn get_last_minute_infos(&self) -> Result<LastMinute> {
        let date_selector = Selector::parse(".date_derniere_minute").unwrap();
        let last_minute_date = self.select(&date_selector).nth(0).unwrap();

        let title_containers = last_minute_date.next_sibling_element().unwrap();

        let h3_selector = Selector::parse("h3").unwrap();
        for title in title_containers.select(&h3_selector) {
            let h3 = title.inner_html();
            let date = title.next_sibling_element().unwrap().first_element_child().unwrap().inner_html();
            println!("{} {}", parse_date(date), h3);
        }

        let infos = vec![Info {
            body: String::from(""),
            title: String::from(""),
            date: String::from(""),
        }];

        Ok(LastMinute { infos })
    }

    fn get_country_by_name(&self) -> Result<Safety> {
        let mediabox_selector = Selector::parse(".mediabox").unwrap();
        let mediabox = self
            .select(&mediabox_selector)
            .nth(0)
            .expect("Mediabox not found.");
        let img = mediabox
            .first_element_child()
            .expect("No image in the mediabox");
        let img_src = img
            .attr("src")
            .expect("There should be a src to this image. I'm smart, I know html.");
        let img_src = format!("https://www.diplomatie.gouv.fr/{img_src}");
        Ok(Safety { img_src })
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

fn get_document(url: &str) -> String {
    return match reqwest::blocking::get(url) {
        Ok(a) => a.text().expect("fatal error"),
        Err(_) => unimplemented!(),
    };
}

pub fn get_countries() -> Vec<String> {
    let url = format!("{}{}", &BASE, PATHNAME);
    let document = get_document(&url);
    Html::parse_document(&document).get_list_of_countries_ids()
}

pub fn get_country_safety(country: &str) -> Result<Safety> {
    let document = get_document(&format!("{}{}/{country}#securite", &BASE, &PATHNAME));
    Html::parse_document(&document).get_country_by_name()
}

pub fn get_country_last_minute(country: &str) -> Result<LastMinute> {
    let url = format!("{}{}/{country}#derniere", &BASE, &PATHNAME);
    let document = get_document(&url);
    Html::parse_document(&document).get_last_minute_infos()
}
