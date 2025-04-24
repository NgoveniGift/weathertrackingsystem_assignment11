
use crate::repository_interface::*;
use crate::repositories::inmemory::{user_repo::InMemoryUserRepository, weather_repo::InMemoryWeatherReportRepository};

pub enum StorageType {
    InMemory,
    FileSystem, // Stub for future support
}

pub struct RepositoryFactory;

impl RepositoryFactory {
    pub fn get_user_repository(storage: StorageType) -> Box<dyn UserRepository> {
        match storage {
            StorageType::InMemory => Box::new(InMemoryUserRepository::new()),
            StorageType::FileSystem => unimplemented!("FileSystemUserRepository not yet implemented"),
        }
    }

    pub fn get_weather_report_repository(storage: StorageType) -> Box<dyn WeatherReportRepository> {
        match storage {
            StorageType::InMemory => Box::new(InMemoryWeatherReportRepository::new()),
            StorageType::FileSystem => unimplemented!("FileSystemWeatherReportRepository not yet implemented"),
        }
    }
}
