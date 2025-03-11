# Open Source Contribution Tracker

## Introduction

The Open Source Contribution Tracker is a command-line application written in Rust that allows users to track their open-source contributions. It enables users to log contributions by adding details such as the GitHub username, repository name, and the number of commits made. The tracker also provides an option to list all recorded contributions.

## Use Case Diagram



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
   cd open-source-tracker
   ```
Build the project:

bash
Copy
Edit
cargo build --release
Run the application:

bash
Copy
Edit
cargo run
Available Commands:
Add Contribution: Adds a new contribution. You will be prompted to input the GitHub username, repository name, and the number of commits.
List Contributions: Displays a list of all recorded contributions.
Exit: Exits the program.
Technologies Used:
Rust: The programming language used to build this CLI tool.
Serde: For serialization and deserialization (used in saving/loading contributions).
Contribution
Feel free to fork the repository and submit a pull request for any improvements or bug fixes.

License
This project is licensed under the MIT License - see the LICENSE file for details.


Technologies Used:

Rust: The programming language used to build this CLI tool.
Serde: For serialization and deserialization (used in saving/loading contributions).
Contribution
Feel free to fork the repository and submit a pull request for any improvements or bug fixes.

License
This project is licensed under the MIT License - see the LICENSE file for details.


Test Cases
1. Functional Test Cases
### 1. Functional Test Cases

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

### 2. Non-Functional Test Cases

| Test Case ID | Type       | Description                           | Steps                              | Expected Result               | Actual Result | Status |
|--------------|------------|---------------------------------------|------------------------------------|-------------------------------|----------------|--------|
| NTC-001      | Performance| Ensure system can handle 1,000 users logging in simultaneously | Simulate 1,000 login requests      | Response time ≤ 2s             |                |        |
| NTC-002      | Security   | Prevent unauthorized access          | Attempt login with incorrect credentials | "Access Denied" message appears |                |        |


Reflection
In this assignment, translating functional requirements into use cases and test cases proved to be a challenging yet rewarding process. The task required a deep understanding of both the stakeholders' needs and the system’s behavior, ensuring that every interaction was captured correctly in the use cases. Here are some key reflections:

Understanding Stakeholder Needs: The first challenge was thoroughly understanding the stakeholders' concerns, which were presented in Assignment 4. These concerns were not always straightforward and often involved trade-offs. For example, balancing between providing a smooth user experience while ensuring system security was a critical factor. Ensuring these concerns were accurately reflected in the use cases required revisiting the functional requirements multiple times.

Defining Clear Actors and Use Cases: Identifying the actors and their roles was another hurdle. Although it was clear that we had core users like Student and Admin, understanding all the interactions each actor would have with the system was crucial. This step required a detailed breakdown of every possible action the actors might take, which was not always obvious at first. Once these were identified, creating the use case diagram was straightforward, but defining the basic flow and alternative flows for each use case was much more time-consuming.

Handling Complex Use Cases: Some use cases, such as "Track Contributions", had many possible alternative flows (e.g., handling GitHub API unavailability or no contributions found). This complexity required more attention to detail to ensure all possible outcomes were covered, and the system would behave as expected in all scenarios.

Test Case Development: Test case development was another challenge, particularly for non-functional requirements. Functional test cases were relatively easy to derive from the use cases, but for performance and security, additional considerations were needed. For example, testing system performance under high load required simulating a high number of users, which involved external tools and thought-out scenarios. Ensuring that security tests covered all possible edge cases, such as unauthorized access attempts, was also critical to guarantee the robustness of the system.

Iterative Refinement: Lastly, the process of iterating through use case specifications, refining them based on new insights, and constantly validating with the requirements was a significant learning experience. It taught me the importance of feedback loops in system design and testing.

Lessons Learned: One key lesson I learned was the importance of comprehensive testing. While functional requirements were straightforward to translate, non-functional aspects such as performance and security had to be tested in real-world conditions. This taught me how critical these elements are in ensuring a robust, scalable system. Additionally, I now understand how detailed and thorough use case documentation can help ensure every edge case is accounted for, which reduces errors during implementation.

In conclusion, this assignment taught me the significance of comprehensive system modeling and the need for clear communication of requirements through use cases and test cases. While it was a challenging task, it was immensely rewarding to see how these documents help in shaping the behavior and functionality of a system.

Submission Checklist
✅ Use Case Diagram (Mermaid.js): The use case diagram is created using Mermaid.js and included in the document.
✅ Detailed Use Case Specifications: Specifications for each selected use case, including descriptions, preconditions, postconditions, flows, and alternative flows.
✅ Test Cases Table (Functional & Non-Functional): Functional and non-functional test cases, including test case ID, description, steps, expected results, and status.
✅ Reflection on Challenges: A 500-word reflection discussing the challenges, lessons, and improvements in requirement translation.
✅ Updated GitHub Repo: The repository is updated with all the necessary files and changes.
✅ Submission Link on Blackboard: The GitHub repo link has been submitted through Blackboard for assessment.
