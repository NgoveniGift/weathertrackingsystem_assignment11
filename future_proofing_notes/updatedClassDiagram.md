```mermaid
classDiagram
    class Repository {
        <<interface>>
        +save(entity: T)
        +find_by_id(id: ID): Option<T>
        +find_all(): Vec<T>
        +delete(id: ID)
    }

    class UserRepository {
        <<interface>>
    }

    class WeatherReportRepository {
        <<interface>>
    }

    class InMemoryUserRepository
    class InMemoryWeatherReportRepository
    class FileSystemWeatherReportRepository {
        todo!()
        FileSystem
        WorkARP
    }

    class RepositoryFactory {
        +get_user_repository()
        +get_weather_report_repository()
    }

    Repository <|-- UserRepository
    Repository <|-- WeatherReportRepository

    UserRepository <|-- InMemoryUserRepository
    WeatherReportRepository <|-- InMemoryWeatherReportRepository
    WeatherReportRepository <|-- FileSystemWeatherReportRepository

    RepositoryFactory --> UserRepository
    RepositoryFactory --> WeatherReportRepository




