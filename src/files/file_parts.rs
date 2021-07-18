use crate::error::RrenamerError;
use crate::error::RrenamerError::{InvalidFileExt, InvalidFilename};
use std::convert::TryFrom;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum FileTypes {
    Movie,
    Series,
}

impl Default for FileTypes {
    fn default() -> Self {
        Self::Movie
    }
}

#[derive(Default, Debug, Clone)]
pub struct FileParts {
    pub file_type: FileTypes,
    pub name: String,
    pub year: Option<String>,
    pub season: Option<String>,
    pub episode: Option<String>,
    pub title: Option<String>,
    pub ext: String,
}

impl TryFrom<&str> for FileParts {
    type Error = RrenamerError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let path = Path::new(s);
        let file_name = path
            .file_stem()
            .ok_or(InvalidFilename(s.to_string()))?
            .to_string_lossy()
            .to_string();
        let ext = path
            .extension()
            .ok_or(InvalidFileExt(s.to_string()))?
            .to_string_lossy()
            .to_string();
        let movie_regex =
            regex::Regex::new(r"^(.*?)+\.?(\d{4})+.*?\.?([a-zA-Z0-9]{3,4})?$").unwrap();
        let series_regex =
            regex::Regex::new(r"^(.*?)+\.?([sS]+(\d+)[eE]+(\d+))+.*?\.?([a-zA-Z0-9]{3,4})?$")
                .unwrap();

        let mut file_parts = None;
        if let Some(cap) = movie_regex.captures(&file_name) {
            let name = format!("{}", cap.get(1).map(|m| m.as_str()).unwrap())
                .replace('.', " ")
                .as_str()
                .to_string();
            let year = cap.get(2).map(|m| m.as_str()).unwrap().replace('.', " ");
            file_parts = Some(FileParts {
                name,
                file_type: FileTypes::Movie,
                year: Some(year),
                ext,
                ..Default::default()
            });
        };

        if let Some(cap) = series_regex.captures(&file_name) {
            let name = cap
                .get(1)
                .and_then(|m| Some(m.as_str()))
                .ok_or(InvalidFilename("".to_string()))?
                .to_string();
            let episode = cap
                .get(3)
                .and_then(|m| Some(m.as_str()))
                .ok_or(InvalidFilename("".to_string()))?
                .to_string();
            let season = cap
                .get(4)
                .and_then(|m| Some(m.as_str()))
                .ok_or(InvalidFilename("".to_string()))?
                .to_string();
            file_parts = Some(FileParts {
                name,
                episode: Some(episode),
                season: Some(season),
                file_type: FileTypes::Series,
                ..Default::default()
            });
        }
        if file_parts.is_some() {
            let parts = file_parts.unwrap();
            return Ok(parts.clone());
        }
        return Err(InvalidFilename(s.to_string()));
    }
}
