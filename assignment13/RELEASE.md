# 📦 Release Artifacts

This project uses GitHub Actions to build and package a JAR file automatically when code is merged into the `main` branch.

## 🎯 Build Tool
- Apache Maven

## 📁 Artifact Location
- GitHub Actions → CI Pipeline → Artifacts

## ✅ Artifact Contents
- Compiled `.jar` file: `target/open-source-tracker-*.jar`

## 💡 How It Works
- Trigger: `push` to `main`
- Steps:
  - Checkout code
  - Set up Java
  - Build with Maven
  - Upload `.jar` file

