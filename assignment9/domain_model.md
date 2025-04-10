# Domain Model

## Key Entities and Relationships

| **Entity**   | **Attributes**                                      | **Methods**                                            | **Relationships**                                            |
|--------------|-----------------------------------------------------|--------------------------------------------------------|-------------------------------------------------------------|
| **User**     | username, email, contributions[]                    | addContribution(), viewContributions(), updateProfile() | Can have multiple Contributions                               |
| **Contribution** | contributionId, date, commitCount, repositoryName | logCommit(), calculateContributions(), displaySummary() | Linked to one User and one Repository                        |
| **Repository** | repoName, description, owner, language             | addContributors(), getContributors()                   | Has many Contributions, belongs to one User                  |
| **Commit**   | commitId, message, date, changes                    | getChangeSummary(), displayCommitDetails()             | Part of a Contribution                                        |
| **Tracker**  | trackerId, contributions[]                          | loadContributions(), saveContributions()               | Manages multiple Users and Contributions                     |

## Business Rules:
1. **A User** can contribute to multiple repositories.
2. **A Contribution** is linked to a specific **User** and **Repository**.
3. **A Commit** belongs to a **Contribution**.
4. **Tracker** maintains all contributions and users in a persistent storage (JSON file).
5. **A User** can view all their contributions or track new ones.

