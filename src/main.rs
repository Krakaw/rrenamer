mod rename;

use dotenv;
use clap::{App, Arg};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv::dotenv().ok();

    let matches = App::new("Rrenamer")
        .version("0.0.1")
        .author("Krakaw")
        .about("Lookup and rename")
        .arg(
            Arg::new("rename")
                .short('r')
                .long("rename")
                .value_name("rename")
                .multiple(true)
                .about("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();


    // You can check the value provided by positional arguments, or option arguments
    if let Some(values) = matches.values_of("rename") {
        for value in values {
            let path = Path::new(value);
            let new_name = lookup_movie(path).await?;
            let s = path.parent().unwrap().join(new_name);
            std::fs::rename(path, s);
        }
    }

    Ok(())
}

async fn lookup_movie(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = dotenv::var("API_KEY").unwrap();
    let file_name = path.file_stem().unwrap().to_str().unwrap();
    let ext = path.extension().unwrap().to_str().unwrap();
    let re = regex::Regex::new(r"^(.*?)+\.?(\d{4})+.*?\.?([a-zA-Z0-9]{3,4})?$").unwrap();
    // let re = regex::Regex::new(r"^(.*?)+\.?([a-zA-Z0-9]{3,4})?$").unwrap();
    let caps = re.captures(file_name);
    eprintln!("caps = {:?}", caps);

    if caps.is_none() {
        return Ok("".to_string());
    }
    let mut search;
    let mut year;
    if let Some(cap) = caps {
        // search = format!("{} ({})", cap.get(1).map(|m|m.as_str().replace('.', " ")).unwrap(), cap.get(2).map(|m|m.as_str()).unwrap());
        search = format!("{}", cap.get(1).map(|m|m.as_str()).unwrap()).replace('.', " ");
        year = format!("{}", cap.get(2).map(|m|m.as_str()).unwrap()).replace('.', " ");
    } else {
        return Ok("".to_string());
    }
    let r = reqwest::get(format!(
        "https://api.themoviedb.org/3/search/movie?api_key={}&query={}&year={}",
        api_key,
        search,
        year
    ))
        .await?
        .json::<serde_json::Value>()
        .await?;


    let movie = r.get("results").unwrap()[0].clone();
    let title = movie.get("original_title").clone().unwrap().as_str().unwrap();
    let year = movie.get("release_date").unwrap().as_str().unwrap().split('-').next().unwrap();
    Ok(format!("{} ({}).{}", title, year, ext))
}
