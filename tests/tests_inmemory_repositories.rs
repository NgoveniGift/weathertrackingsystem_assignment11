// test in-memory repositories

use crate::repositories::inmemory::{user_repo::InMemoryUserRepository, weather_repo::InMemoryWeatherReportRepository};
use crate::repository_interface::{Repository, User, WeatherReport};

#[test]
fn test_user_repository_crud() {
    let mut repo = InMemoryUserRepository::new();
    let user = User {
        user_id: "u1".into(),
        name: "Alice".into(),
        email: "alice@example.com".into(),
    };

    repo.save(user.user_id.clone(), user.clone());
    assert_eq!(repo.find_by_id(&"u1".into()), Some(&user));

    let all_users = repo.find_all();
    assert_eq!(all_users.len(), 1);

    repo.delete(&"u1".into());
    assert!(repo.find_by_id(&"u1".into()).is_none());
}

#[test]
fn test_weather_report_repository_crud() {
    let mut repo = InMemoryWeatherReportRepository::new();
    let report = WeatherReport {
        report_id: "r1".into(),
        temperature: 22.0,
        humidity: 60.0,
        wind_speed: 10.5,
        timestamp: "2025-04-01T10:00:00Z".into(),
    };

    repo.save(report.report_id.clone(), report.clone());
    assert_eq!(repo.find_by_id(&"r1".into()), Some(&report));

    let all_reports = repo.find_all();
    assert_eq!(all_reports.len(), 1);

    repo.delete(&"r1".into());
    assert!(repo.find_by_id(&"r1".into()).is_none());
}
