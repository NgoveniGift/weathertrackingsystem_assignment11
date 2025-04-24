// Stub: FileSystemWeatherReportRepository (future-proofing example)
use crate::repository_interface::{Repository, WeatherReport};
use std::collections::HashMap;

// Stub structure for file-backed repository
pub struct FileSystemWeatherReportRepository {
    pub file_path: String,
}

impl FileSystemWeatherReportRepository {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    fn load_all(&self) -> HashMap<String, WeatherReport> {
        // Placeholder: in future, read JSON from self.file_path
        todo!("Load from file")
    }

    fn save_all(&self, _data: &HashMap<String, WeatherReport>) {
        // Placeholder: in future, write JSON to self.file_path
        todo!("Save to file")
    }
}

impl Repository<String, WeatherReport> for FileSystemWeatherReportRepository {
    fn save(&mut self, id: String, report: WeatherReport) {
        let mut all = self.load_all();
        all.insert(id, report);
        self.save_all(&all);
    }

    fn find_by_id(&self, id: &String) -> Option<&WeatherReport> {
        let all = self.load_all();
        all.get(id)
    }

    fn find_all(&self) -> Vec<&WeatherReport> {
        let all = self.load_all();
        all.values().collect()
    }

    fn delete(&mut self, id: &String) {
        let mut all = self.load_all();
        all.remove(id);
        self.save_all(&all);
    }
}

