# System Specification Document

## 2.2 Introduction

### Project Title: Open Source Contribution Tracker

### Domain:
The domain of this project falls under **Open Source Software Development**. The system is focused on tracking and managing user contributions to open-source repositories, providing developers with an easy way to log and review their contributions across various GitHub repositories.

### Problem Statement:
In the world of open-source software development, tracking contributions can become cumbersome, especially when managing multiple repositories and users. This system aims to simplify the process of tracking and recording user contributions by providing a clear interface to log contributions (including the username, repository, and commit count) and easily view past entries.

### Individual Scope:
The scope of this system is feasible given the context of a command-line application, which allows developers to track their contributions in a simple and lightweight manner. The project leverages local file storage (JSON) to persist data and avoids the complexity of integrating with external APIs or databases.

## 2.3 C4 Architectural Diagrams

### Project Title: Open Source Contribution Tracker

### Domain:
The domain of this project is **Open Source Software Development**.

### Problem Statement:
This system is designed to assist developers by providing a simple and efficient way to track and manage their contributions to various open-source repositories.

### Individual Scope:
The project is feasible with the use of a command-line interface and local file storage. The system will allow users to add and view their contributions and store this information in a JSON file.

## 2.3.1 System Context Diagram

```plaintext
    +------------------------+
    |  Open Source Tracker   |
    |                        |
    |   - Add Contribution    |
    |   - List Contributions  |
    +------------------------+
          |      ^
    User   |      |
    Inputs  |      |
          v       |
    +------------------------+
    |   Local Storage        |
    |  (contributions.json)  |
    +------------------------+

2.3.2 Container Diagram

    +-----------------------------------------+
    |         Open Source Tracker            |
    |                                         |
    |   +----------------+     +----------+   |
    |   |   Command-Line | <--->| Local    |   |
    |   |   Interface    |     | Storage  |   |
    |   |                |     | (JSON)   |   |
    |   +----------------+     +----------+   |
    +-----------------------------------------+
2.3.3 Component Diagram
    +-----------------------------+
    | Open Source Tracker         |
    |                             |
    |  +-------------------------+|
    |  | TrackerApp               ||
    |  | - load_from_file         ||
    |  | - save_to_file           ||
    |  | - add_contribution       ||
    |  | - list_contributions     ||
    |  +-------------------------+|
    +-----------------------------+ 
               |
               v
    +-----------------------------+
    | Contributions (JSON)        |
    |                             |
    |  - username                 |
    |  - repo                     |
    |  - commits                  |
    +-----------------------------+
