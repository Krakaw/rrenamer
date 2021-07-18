use std::path::{Path, PathBuf};
use crate::error::RrenamerError;
use crate::error::RrenamerError::{InvalidFilename, MovieNotFound, InvalidFileExt, InvalidPath};

pub struct InputFile {
    pub input_path: PathBuf,
    pub movie_name: Option<String>,
    pub output_dir: Option<PathBuf>,
}

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

impl InputFile {
    pub fn new(path: &str, output_dir: Option<PathBuf>) -> Result<InputFile, RrenamerError> {
        Ok(InputFile {
            input_path: Path::new(path).to_path_buf(),
            movie_name: None,
            output_dir,
        })
    }

    pub fn set_movie_name(&mut self, movie_name: String) {
        self.movie_name = Some(movie_name);
    }

    pub fn lookup_parts(&self) -> Result<FileParts, RrenamerError> {
        let file_name = self.input_path.file_stem().ok_or(InvalidFilename)?.to_string_lossy();
        let ext = self.input_path.extension().ok_or(InvalidFilename)?.to_string_lossy();
        let movie_regex = regex::Regex::new(r"^(.*?)+\.?(\d{4})+.*?\.?([a-zA-Z0-9]{3,4})?$").unwrap();
        let series_regex = regex::Regex::new(r"^(.*?)+\.?([sS]+(\d+)[eE]+(\d+))+.*?\.?([a-zA-Z0-9]{3,4})?$").unwrap();


        if let Some(cap) = movie_regex.captures(&file_name) {
            let name = format!("{}", cap.get(1).map(|m| m.as_str()).unwrap()).replace('.', " ");
            let year = format!("{}", cap.get(2).map(|m| m.as_str()).unwrap()).replace('.', " ");
            return Ok(FileParts {
                name,
                file_type: FileTypes::Movie,
                year: Some(year),
                ext: ext.to_string(),
                ..Default::default()
            });
        }
        if let Some(cap) = series_regex.captures(&file_name) {
            eprintln!("cap = {:?}", cap);
            let name = cap.get(1).and_then(|m| Some(m.as_str())).ok_or(InvalidFilename)?.to_string();
            let episode = cap.get(3).and_then(|m| Some(m.as_str())).ok_or(InvalidFilename)?.to_string();
            let season = cap.get(4).and_then(|m| Some(m.as_str())).ok_or(InvalidFilename)?.to_string();
            return Ok(FileParts {
                name,
                episode: Some(episode),
                season: Some(season),
                file_type: FileTypes::Series,
                ..Default::default()
            });
        }

        return Err(InvalidFilename);
    }

    pub fn output_path(&self) -> Result<PathBuf, RrenamerError> {
        let ext = self.input_path.extension().ok_or(InvalidFileExt("".to_string()))?.to_string_lossy();
        let input_parent = &self.input_path.parent().ok_or(InvalidPath("".to_string()))?.to_path_buf();
        let parent = self.output_dir.as_ref().unwrap_or_else(|| input_parent);
        let output = parent.join(format!("{}.{}", self.movie_name.as_ref().ok_or(MovieNotFound)?, ext));
        Ok(output)
    }

    pub fn rename_file(self) -> Result<PathBuf, RrenamerError> {
        let output_path = self.output_path()?;
        std::fs::rename(self.input_path, output_path.clone())?;
        Ok(output_path)
    }
}
