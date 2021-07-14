use crate::error::RrenamerError;
use crate::error::RrenamerError::MovieNotFound;
use crate::lookup::tmdb_results::{TmdbResults, TmdbResult};
use crate::files::input_file::{FileParts, FileTypes};


pub struct Tmdb;


impl Tmdb {
    pub async fn lookup(file_parts: FileParts) -> Result<TmdbResults, RrenamerError> {
        match file_parts.file_type {
            FileTypes::Movie => {
                Tmdb::lookup_movie(file_parts).await
            }
            FileTypes::Series => {
                Tmdb::lookup_series(file_parts).await
            }
        }
    }

    pub async fn lookup_series(file_parts: FileParts) -> Result<TmdbResults, RrenamerError> {
        let search = file_parts.name;
        let season = file_parts.season.unwrap_or("".to_string());
        let episode = file_parts.episode.unwrap_or("".to_string());

        let api_key = dotenv::var("API_KEY").unwrap();
        let r = reqwest::get(format!(
            "https://api.themoviedb.org/3/search/tv?api_key={}&query={}",
            api_key,
            search,
        ))
            .await?
            .json::<serde_json::Value>()
            .await?;

        eprintln!("r = {:?}", r);
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

    pub async fn lookup_movie(file_parts: FileParts) -> Result<TmdbResults, RrenamerError> {
        let search = file_parts.name;
        let year = file_parts.year.unwrap_or("".to_string());

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
