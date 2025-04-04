# Architecture Document

## 1. Project Title: Open Source Contribution Tracker

### 1.1 Domain:
This system operates within the **Open Source Software Development** domain. It helps developers track their contributions to open-source repositories by storing relevant details (such as GitHub username, repository name, and number of commits) and making it easier for them to review their contributions over time.

### 1.2 Problem Statement:
As open-source contributors accumulate numerous contributions across different repositories, tracking them manually becomes difficult. This system automates the tracking process and enables developers to keep an organized record of their contributions.

### 1.3 Individual Scope:
The system’s scope is focused on providing a simple and intuitive user interface for tracking contributions, using local file storage (JSON) to persist data. The application operates through a command-line interface and does not require any external databases or APIs.

---

## 2. C4 Architectural Diagrams

### 2.1 System Context Diagram

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

2.2 Container Diagram
    +-----------------------------------------+
    |         Open Source Tracker            |
    |                                         |
    |   +----------------+     +----------+   |
    |   |   Command-Line | <--->| Local    |   |
    |   |   Interface    |     | Storage  |   |
    |   |                |     | (JSON)   |   |
    |   +----------------+     +----------+   |
    +-----------------------------------------+

2.3 Component Diagram
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
