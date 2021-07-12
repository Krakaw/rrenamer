use std::fmt::{Display, Formatter};

pub struct TmdbResults(pub Vec<TmdbResult>);

impl Display for TmdbResults {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, result) in self.0.iter().enumerate() {
            writeln!(f, "{}) - {}", i, result).unwrap_or(());
        }
        Ok(())
    }
}

pub struct TmdbResult {
    pub title: String,
    pub year: String,
}

impl Display for TmdbResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}
