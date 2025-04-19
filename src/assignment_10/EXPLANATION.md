# Mapping Code Implementation to UML Class Diagram

This document explains how the implemented Rust code corresponds to the UML class diagram designed using Mermaid.js.

---

## 🧑‍💻 `User` Class

### UML:
- Attributes: `username`, `email`, `contributions`
- Methods: `addContribution()`, `viewContributions()`, `updateProfile()`

### Code:
Implemented as `User` struct in `user.rs` with a `Vec<Contribution>`. Each method was implemented as per the class diagram:
- `add_contribution()` to add a new contribution.
- `view_contributions()` to display all contributions.
- `update_profile()` to change user email.

---

## 🔄 `Contribution` Class

### UML:
- Attributes: `contributionId`, `date`, `commitCount`, `repositoryName`
- Methods: `logCommit()`, `calculateContributions()`, `displaySummary()`

### Code:
Implemented in `contribution.rs`. The `Contribution` struct holds commit data and repository name.
- `log_commit()` appends a new commit.
- `calculate_contributions()` returns the number of commits.
- `display_summary()` prints a summary.

---

## 🧱 `Commit` Class

### UML:
- Attributes: `commitId`, `message`, `date`, `changes`
- Methods: `getChangeSummary()`, `displayCommitDetails()`

### Code:
Implemented in `commit.rs`.
- `get_change_summary()` returns a brief change summary.
- `display_commit_details()` prints commit details.

---

## 📦 `Repository` Class

### UML:
- Attributes: `repoName`, `description`, `owner`, `language`
- Methods: `addContributors()`, `getContributors()`

### Code:
In `repository.rs`, the `Repository` struct contains contributor details.
- `add_contributors()` adds a contributor to the list.
- `get_contributors()` returns a copy of all contributors.

---

## 📊 `Tracker` Class

### UML:
- Attributes: `trackerId`, `contributions`
- Methods: `loadContributions()`, `saveContributions()`

### Code:
In `tracker.rs`, tracks multiple contributions. These methods can be expanded to persist/load data (e.g., using file I/O in future).

---

## 🔗 Relationships

### As per the class diagram:
- `User` → `Contribution`: Implemented via `Vec<Contribution>`
- `Contribution` → `Repository`: Linked via `repository_name` field
- `Contribution` → `Commit`: Implemented via `Vec<Commit>`
- `Tracker` → `Contribution`: `Vec<Contribution>` is used internally

---

The code successfully maps all classes, attributes, and relationships described in the UML class diagram into working Rust code.
