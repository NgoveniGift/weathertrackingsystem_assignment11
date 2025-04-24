// Stub: FileSystemWeatherReportRepository (future-proofing example)
// This is a placeholder for a repository that will persist WeatherReport entities to a file system.
// No full implementation is required.

use crate::repository_interface::{Repository, WeatherReport};

pub struct FileSystemWeatherReportRepository {
    pub file_path: String,
}

impl FileSystemWeatherReportRepository {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

impl Repository<String, WeatherReport> for FileSystemWeatherReportRepository {
    fn save(&mut self, _id: String, _report: WeatherReport) {
        todo!("Persist WeatherReport to file system");
    }

    fn find_by_id(&self, _id: &String) -> Option<&WeatherReport> {
        todo!("Load WeatherReport by ID from file system");
    }

    fn find_all(&self) -> Vec<&WeatherReport> {
        todo!("Load all WeatherReports from file system");
    }

    fn delete(&mut self, _id: &String) {
        todo!("Delete WeatherReport from file system");
    }
}


