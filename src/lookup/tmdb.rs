use std::path::Path;
use crate::error::RrenamerError;

pub struct Tmdb {

}

impl Tmdb {
    pub async fn lookup(search: String, year:String) -> Result<String, RrenamerError> {
        let api_key = dotenv::var("API_KEY").unwrap();
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
        Ok(format!("{} ({})", title, year))
    }
}
