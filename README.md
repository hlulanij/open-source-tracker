# Open Source Contribution Tracker

## Introduction
The Open Source Contribution Tracker is a command-line application written in Rust that allows users to track their open-source contributions. It enables users to log contributions by adding details such as the GitHub username, repository name, and the number of commits made. The tracker also provides an option to list all recorded contributions.

## Project Objective
This project is designed to:
- Track open-source contributions made by an individual.
- Store and retrieve contributions from a JSON file.
- Provide a simple command-line interface (CLI) for interacting with the tracker.

## Features
The application has the following features:
- **Add Contribution**: Allows the user to input their GitHub username, repository name, and the number of commits.
- **List Contributions**: Displays all contributions stored in the system.
- **Save to File**: Contributions are saved to a JSON file, ensuring persistence.
- **Load from File**: Contributions are loaded from the JSON file when the application starts.

## How to Use
To run the application, ensure you have Rust installed on your machine. If you don't have Rust installed, follow the instructions on the official website: [https://www.rust-lang.org/](https://www.rust-lang.org/).

### Steps to Run
1. Clone the repository:
   ```bash
   git clone https://github.com/hlulanij/open-source-tracker.git
# Open Source Contribution Tracker
A command-line tool built with Rust to track open-source contributions for students.

## Features:
- **Add Contribution**: Add a new contribution, specifying the user's name, project, and number of commits.
- **List Contributions**: View all recorded contributions.
- **Exit**: Exit the program.

## Technologies Used:
- **Rust**: The programming language used to build this CLI tool.
- **Serde**: For serialization and deserialization (used in saving/loading contributions).

## Installation

1. Clone the repository to your local machine:
   ```bash
   git clone https://github.com/hlulanij/open-source-tracker.git
   cd open-source-tracker
