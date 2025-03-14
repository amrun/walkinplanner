use std::fs::File;
use std::io::{BufWriter, Write};

pub struct FileHandler {
    lines: Vec<String>,
}

impl FileHandler {
    pub fn new() -> Self {
        FileHandler { lines: Vec::new() }
    }

    pub fn add_header_line(&mut self) {
        self.lines
            .push("Datum,Wochentag,Vormittag,Nachmittag".to_string());
    }

    pub fn add_line(&mut self, line: &str) {
        self.lines.push(line.to_string());
    }

    pub fn write_to_file(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file); // Use BufWriter for efficiency

        for line in &self.lines {
            // TODO: check for filtering options; eg. Weekends, etc.
            writeln!(writer, "{}", line)?; // Write each line with a newline
        }

        writer.flush()?; // Ensure all data is written
        Ok(())
    }
}
