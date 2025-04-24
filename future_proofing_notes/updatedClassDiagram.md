classDiagram

%% Core Entities
class User {
    +String user_id
    +String name
    +String email
}

class WeatherReport {
    +String report_id
    +f64 temperature
    +f64 humidity
    +f64 wind_speed
    +String timestamp
}

%% Generic Repository Interface
class Repository~ID, T~ {
    <<interface>>
    +save(id: ID, entity: T)
    +find_by_id(id: &ID): Option<&T>
    +find_all(): Vec<&T>
    +delete(id: &ID)
}

%% Entity-specific Repository Interfaces
class UserRepository {
    <<interface>>
}
class WeatherReportRepository {
    <<interface>>
}

%% In-memory Implementations
class InMemoryUserRepository
class InMemoryWeatherReportRepository

%% Stubbed Future Implementation
class FileSystemWeatherReportRepository {
    +String file_path
    +save_all()
    +load_all()
}

%% Factory
class RepositoryFactory {
    +get_user_repository(storage: StorageType): Box<UserRepository>
    +get_weather_report_repository(storage: StorageType): Box<WeatherReportRepository>
}

%% Relationships
Repository <|-- UserRepository
Repository <|-- WeatherReportRepository
UserRepository <|-- InMemoryUserRepository
WeatherReportRepository <|-- InMemoryWeatherReportRepository
WeatherReportRepository <|-- FileSystemWeatherReportRepository
RepositoryFactory --> UserRepository
RepositoryFactory --> WeatherReportRepository

