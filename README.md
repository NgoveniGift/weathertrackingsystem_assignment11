# weathertrackingsystem_assignment11

# 🌦️ Assignment 11 – Persistence Layer for Weather Tracking System

This assignment implements a flexible persistence repository for the Weather Tracking System using a generic interface, in-memory storage, factory-based abstraction, and a stub for future backend extension.

---

## 📚 Contents

### 1. 🧱 Repository Interfaces
- Generic `Repository<ID, T>` trait in `repository_interface.rs`
- Entity-specific traits: `UserRepository`, `WeatherReportRepository`
- Stored in `/repositories/`

### 2. 🧠 In-Memory Implementation
- Located in `/repositories/inmemory/`
- Repositories: `InMemoryUserRepository`, `InMemoryWeatherReportRepository`
- CRUD with `HashMap` storage per entity

### 3. 🏗️ Abstraction Mechanism (Factory)
- Factory Pattern defined in `/factories/repository_factory.rs`
- Switchable `StorageType` enum with `InMemory` and stub for `FileSystem`

### 4. 🔮 Future-Proofing
- `StorageType::FileSystem` uses `unimplemented!()`
- Explained in `/documentation/future_proofing_notes.md`
- Supports easy scale-up to database, file system, or remote API
- UML class diagram in `/diagrams/uml_persistence_layer.png`

### 5. 🧪 Testing
- All unit tests in `/tests/tests_inmemory_repositories.rs`
- Coverage includes:
  - Saving
  - Finding by ID
  - Listing all
  - Deleting
  - Factory integration

---

## 🧠 Design Benefits
- Decoupled data access via trait abstraction
- Easily extendable without modifying client code
- Factory ensures runtime configurability
- Compliant with SOLID design principles

---

## 📁 File Structure
```
weather-tracking-system/
├── repositories/
│   ├── repository_interface.rs
│   └── inmemory/
│       ├── user_repo.rs
│       └── weather_repo.rs
│
├── factories/
│   └── repository_factory.rs
│
├── documentation/
│   └── future_proofing_notes.md
│
├── diagrams/
│   └── uml_persistence_layer.png
│
├── tests/
│   └── tests_inmemory_repositories.rs
│
├── README_assignment11.md

