use std::path::{Path, PathBuf};
use crate::error::RrenamerError;
use crate::error::RrenamerError::{InvalidFilename, MovieNotFound};

pub struct InputFile {
    pub input_path: PathBuf,
    pub movie_name: Option<String>,
}

impl InputFile {
    pub fn new (path: &str)-> Result<InputFile, RrenamerError> {
        Ok(InputFile {
            input_path: Path::new(path).to_path_buf(),
            movie_name: None
        })
    }

    pub fn set_movie_name(&mut self, movie_name: String) {
        self.movie_name = Some(movie_name);
    }

    pub fn lookup_parts(&self) -> Result<(String, String, String), RrenamerError> {
        let file_name = self.input_path.file_stem().ok_or(InvalidFilename)?.to_string_lossy();
        let ext = self.input_path.extension().ok_or(InvalidFilename)?.to_string_lossy();
        let re = regex::Regex::new(r"^(.*?)+\.?(\d{4})+.*?\.?([a-zA-Z0-9]{3,4})?$").unwrap();
        let caps = re.captures(&file_name);

        let search;
        let year;
        if let Some(cap) = caps {
            search = format!("{}", cap.get(1).map(|m|m.as_str()).unwrap()).replace('.', " ");
            year = format!("{}", cap.get(2).map(|m|m.as_str()).unwrap()).replace('.', " ");
        } else {
            return Err(InvalidFilename);
        }
        Ok((search, year, ext.into()))
    }

    pub fn output_path(&self) -> Result<PathBuf, RrenamerError> {
        let ext = self.input_path.extension().ok_or(InvalidFilename)?.to_string_lossy();
        let output = self.input_path.parent().ok_or(InvalidFilename)?.join(format!("{}.{}", self.movie_name.as_ref().ok_or(MovieNotFound)?, ext));
        Ok(output)
    }

    pub fn rename_file(self) -> Result<PathBuf, RrenamerError> {
        let output_path = self.output_path()?;
        std::fs::rename(self.input_path, output_path.clone())?;
        Ok(output_path)
    }




}
