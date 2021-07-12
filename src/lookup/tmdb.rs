use crate::error::RrenamerError;
use crate::error::RrenamerError::MovieNotFound;
use crate::lookup::tmdb_results::{TmdbResults, TmdbResult};


pub struct Tmdb;


impl Tmdb {
    pub async fn lookup(search: String, year: String) -> Result<TmdbResults, RrenamerError> {
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

        let results = r.get("results").ok_or(MovieNotFound)?.as_array().ok_or(MovieNotFound)?.iter().map(|m| {
            let title = m.get("original_title").map(|v| {
                if v.is_string() {
                    v.as_str().unwrap()
                } else {
                    ""
                }
            }).unwrap_or("").to_string();
            let year = m.get("release_date").map(|v| {
                if v.is_string() {
                    v.as_str().unwrap().split('-').next().unwrap_or("")
                } else {
                    ""
                }
            }).unwrap_or("").to_string();
            TmdbResult {
                title,
                year,
            }
        }).collect::<_>();

        Ok(TmdbResults(results))
    }
}
