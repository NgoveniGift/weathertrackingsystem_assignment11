// in-memory weather report repository

use std::collections::HashMap;
use crate::repository_interface::{Repository, WeatherReport, WeatherReportRepository};

pub struct InMemoryWeatherReportRepository {
    store: HashMap<String, WeatherReport>,
}

impl InMemoryWeatherReportRepository {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl Repository<String, WeatherReport> for InMemoryWeatherReportRepository {
    fn save(&mut self, id: String, entity: WeatherReport) {
        self.store.insert(id, entity);
    }

    fn find_by_id(&self, id: &String) -> Option<&WeatherReport> {
        self.store.get(id)
    }

    fn find_all(&self) -> Vec<&WeatherReport> {
        self.store.values().collect()
    }

    fn delete(&mut self, id: &String) {
        self.store.remove(id);
    }
}

impl WeatherReportRepository for InMemoryWeatherReportRepository {}

