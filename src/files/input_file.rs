use crate::error::RrenamerError;
use crate::error::RrenamerError::{InvalidFileExt, InvalidFilename, InvalidPath, MovieNotFound};
use crate::files::file_parts::FileParts;
use std::convert::TryFrom;
use std::path::{Path, PathBuf};
use std::str::FromStr;
pub struct InputFile {
    pub input_file_parts: FileParts,
    pub output_dir: Option<PathBuf>,

    pub input_path: PathBuf,

    pub output_file_name: Option<String>,
}

impl InputFile {
    pub fn new(path: &str, output_dir: Option<PathBuf>) -> Result<InputFile, RrenamerError> {
        let input_file_parts = FileParts::try_from(path)?;
        if let Some(output_dir) = output_dir.as_ref() {
            if !output_dir.exists() {
                return Err(InvalidPath(output_dir.to_string_lossy().to_string()));
            }
        }
        let input_file = InputFile {
            input_file_parts,
            output_dir,

            input_path: Path::new(path).to_path_buf(),
            output_file_name: None,
        };
        Ok(input_file)
    }

    pub fn set_output_file_name(self, output_file_name: String) -> Self {
        InputFile {
            output_file_name: Some(output_file_name),
            ..self
        }
    }

    pub fn output_path(&self) -> Result<PathBuf, RrenamerError> {
        let ext = &self.input_file_parts.ext;
        let input_parent = &self
            .input_path
            .parent()
            .ok_or(InvalidPath("".to_string()))?
            .to_path_buf();
        let parent = self.output_dir.as_ref().unwrap_or_else(|| input_parent);
        let output = parent.join(format!(
            "{}.{}",
            self.output_file_name.as_ref().ok_or(MovieNotFound)?,
            ext
        ));
        Ok(output)
    }

    pub fn rename_file(self) -> Result<PathBuf, RrenamerError> {
        let output_path = self.output_path()?;
        std::fs::rename(self.input_path, output_path.clone())?;
        Ok(output_path)
    }
}
