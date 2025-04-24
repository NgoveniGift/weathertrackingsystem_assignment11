// Weather Tracking System - Generic Repository Interface

use std::collections::HashMap;
use std::fmt::Debug;

pub trait Repository<ID, T> {
    fn save(&mut self, id: ID, entity: T);
    fn find_by_id(&self, id: &ID) -> Option<&T>;
    fn find_all(&self) -> Vec<&T>;
    fn delete(&mut self, id: &ID);
}

// Example entity: User
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub email: String,
}

pub trait UserRepository: Repository<String, User> {}

// In-memory User Repository
pub struct InMemoryUserRepository {
    store: HashMap<String, User>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self { store: HashMap::new() }
    }
}

impl Repository<String, User> for InMemoryUserRepository {
    fn save(&mut self, id: String, entity: User) {
        self.store.insert(id, entity);
    }

    fn find_by_id(&self, id: &String) -> Option<&User> {
        self.store.get(id)
    }

    fn find_all(&self) -> Vec<&User> {
        self.store.values().collect()
    }

    fn delete(&mut self, id: &String) {
        self.store.remove(id);
    }
}

impl UserRepository for InMemoryUserRepository {}

// Example entity: WeatherReport
#[derive(Debug, Clone, PartialEq)]
pub struct WeatherReport {
    pub report_id: String,
    pub temperature: f64,
    pub humidity: f64,
    pub wind_speed: f64,
    pub timestamp: String,
}

pub trait WeatherReportRepository: Repository<String, WeatherReport> {}

// In-memory WeatherReport Repository
pub struct InMemoryWeatherReportRepository {
    store: HashMap<String, WeatherReport>,
}

impl InMemoryWeatherReportRepository {
    pub fn new() -> Self {
        Self { store: HashMap::new() }
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

