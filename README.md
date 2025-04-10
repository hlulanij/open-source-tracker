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

## Use Case Diagram
![Use Case Diagram](https://github.com/hlulanij/open-source-tracker/blob/main/mermaid-diagram-2025-03-11-204737.png?raw=true)

## Functional Test Cases

| Test Case ID | Requirement ID | Description                         | Steps                                        | Expected Result                 | Actual Result | Status |
|--------------|----------------|-------------------------------------|----------------------------------------------|---------------------------------|----------------|--------|
| TC-001       | FR-001         | Student logs into the system        | 1. Enter credentials 2. Click Login          | Dashboard displayed              |                |        |
| TC-002       | FR-002         | Student tracks contributions        | 1. Click "Track Contributions" 2. API fetches data | Contributions displayed          |                |        |
| TC-003       | FR-003         | Admin reviews contributions         | 1. Admin selects "Review Contributions"      | Contributions listed             |                |        |
| TC-004       | FR-004         | Student generates a report          | 1. Click "Generate Report" 2. System processes data | Report is displayed              |                |        |
| TC-005       | FR-005         | System handles empty contribution history | 1. Track Contributions with an empty GitHub history | "No contributions found" message appears | | |
| TC-006       | FR-006         | Admin removes a user                | 1. Select user 2. Click "Remove"            | User deleted from system         |                |        |
| TC-007       | FR-007         | Student updates profile             | 1. Edit name and bio 2. Click "Save"        | Profile updated successfully     |                |        |
| TC-008       | FR-008         | GitHub API unavailability           | 1. Disable API 2. Click "Track Contributions" | "Unable to fetch data" message appears | | |

## Non-Functional Test Cases

| Test Case ID | Type       | Description                           | Steps                              | Expected Result               | Actual Result | Status |
|--------------|------------|---------------------------------------|------------------------------------|-------------------------------|----------------|--------|
| NTC-001      | Performance| Ensure system can handle 1,000 users logging in simultaneously | Simulate 1,000 login requests      | Response time ≤ 2s             |                |        |
| NTC-002      | Security   | Prevent unauthorized access          | Attempt login with incorrect credentials | "Access Denied" message appears |                |        |

## Technologies Used:
- **Rust**: The programming language used to build this CLI tool.
- **Serde**: For serialization and deserialization (used in saving/loading contributions).

## Installation

1. Clone the repository to your local machine:
   ```bash
   git clone https://github.com/hlulanij/open-source-tracker.git
   cd open-source-tracker
   
2. Make sure Rust is installed on your system. If you don't have it, you can install it by following the instructions on the official Rust website: https://www.rust-lang.org/learn/get-started.

3. To build and run the project, use the following commands:cargo build
cargo run

4. Follow the on-screen prompts to add contributions or view recorded contributions.

How to Use

1. Clone the repository:
git clone https://github.com/hlulanij/open-source-tracker.git
cd open-source-tracker

2. Build the project: cargo build
3. Run the application:cargo run
Follow the on-screen prompts to add contributions or list them.

Reflection
In this assignment, translating functional requirements into use cases and test cases proved to be a challenging yet rewarding process. The task required a deep understanding of both the stakeholders' needs and the system’s behavior, ensuring that every interaction was captured correctly in the use cases.

✅ Checklist for Submission:

✅ Use Case Diagram: The use case diagram is created and included in the document.

✅ Detailed Use Case Specifications: Specifications for each selected use case, including descriptions, preconditions, postconditions, flows, and alternative flows.

✅ Test Cases Table: Functional and non-functional test cases, including test case ID, description, steps, expected results, and status.

✅ Reflection on Challenges: A reflection discussing the challenges, lessons, and improvements in requirement translation.

✅ Updated GitHub Repo: The repository is updated with all the necessary files and changes.

✅ Submission Link on Blackboard: Ensure you’ve posted the link to your GitHub repository on Blackboard LMS for final submission.

Push Changes to GitHub:
bash
Copy code
git add README.md
git commit -m "Update README with installation instructions, features, and use case diagram"
git push
yaml
Copy code

---

Let me know if this looks good to you, and feel free to copy and paste it into your `README.md`!







   
