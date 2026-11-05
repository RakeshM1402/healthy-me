# 🥗 HealthyMe Mobile (Rust-Native Android App & Core)

[![📥 Download Latest Android APK](https://img.shields.io/badge/Download-Latest%20Android%20APK-brightgreen?style=for-the-badge&logo=android)](https://github.com/RakeshM1402/healthy-me/releases/latest)

> **HealthyMe** is a high-performance, lightweight health and fitness companion powered by a native Rust core (`healthifyme_mobile`) providing offline nutritional data, workout scheduling, BMR/TDEE calculations, and automated in-app update checking.

---

## 📲 How to Download & Side-Load the APK

Follow this step-by-step checklist to install the latest release of HealthyMe on your Android device:

1. **📥 Download the APK**:
   - Visit the [HealthyMe Releases Page](https://github.com/RakeshM1402/healthy-me/releases/latest) or click the download badge above.
   - Download the latest signed `healthifyme_mobile.apk` file directly to your mobile device or computer.

2. **⚙️ Enable Unknown Sources (If Required)**:
   - On your Android device, navigate to **Settings > Security** (or **Apps > Special app access**).
   - Enable **Install unknown apps** for your browser or file manager.

3. **🔄 Transfer & Install**:
   - If downloaded on a computer, transfer the `.apk` file via USB cable, Google Drive, or AirDrop.
   - Tap the `healthifyme_mobile.apk` file in your file manager and select **Install**.

4. **🚀 Launch & Preserve State**:
   - Open **HealthyMe** from your app drawer.
   - The app automatically queries the GitHub Releases API for future updates and preserves your local diary logs and nutritional profiles across upgrades.

---

## 🛠️ Tech Stack & Architecture

- **Core Engine**: Pure Rust (`healthifyme_mobile`, serde, ureq)
- **Mobile Framework**: Android Native Activity (`android-activity`, `cargo-apk`)
- **CI/CD Pipeline**: GitHub Actions (`.github/workflows/release.yml`) with automated test suites and APK release packaging.
