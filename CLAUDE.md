# HealthyMe Development Guidelines

## Project Overview
HealthyMe is a Rust-native mobile application (`healthifyme_mobile`) providing offline nutritional food databases, workout scheduling, BMR/TDEE calculations, and automated GitHub Releases in-app updates.

## Key Commands
- **Run Tests**: `cargo test --manifest-path app/Cargo.toml`
- **Build CLI / App**: `cargo build --manifest-path app/Cargo.toml`
- **Build Android APK**: `cargo apk build --manifest-path app/Cargo.toml`
- **Build Release APK**: `cargo apk build --release --manifest-path app/Cargo.toml`

## Project Structure
- `app/Cargo.toml`: Rust crate configuration & Android metadata (`[package.metadata.android]`)
- `app/src/food.rs`: Nutritional food database and search utilities
- `app/src/workout.rs`: Workout schemas and category filtering
- `app/src/calculator.rs`: Mifflin-St Jeor BMR, TDEE, and macro calculations
- `app/src/state.rs`: Mobile app state and daily calorie tracking
- `app/src/updater.rs`: GitHub Releases API update checker
- `app/src/main.rs`: CLI / Entry point
- `.github/workflows/release.yml`: CI/CD workflow (tests on all branches, APK release build on `v*` tags)
