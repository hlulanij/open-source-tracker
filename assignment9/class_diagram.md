# Class Diagram (Mermaid.js)

```mermaid
classDiagram
    class User {
        -username: String
        -email: String
        -contributions: List<Contribution>
        +addContribution()
        +viewContributions()
        +updateProfile()
    }

    class Contribution {
        -contributionId: String
        -date: Date
        -commitCount: Int
        -repositoryName: String
        +logCommit()
        +calculateContributions()
        +displaySummary()
    }

    class Repository {
        -repoName: String
        -description: String
        -owner: String
        -language: String
        +addContributors()
        +getContributors()
    }

    class Commit {
        -commitId: String
        -message: String
        -date: Date
        -changes: String
        +getChangeSummary()
        +displayCommitDetails()
    }

    class Tracker {
        -trackerId: String
        -contributions: List<Contribution>
        +loadContributions()
        +saveContributions()
    }

    User "1" --> "0..*" Contribution : creates
    Contribution "1" --> "1" Repository : logsTo
    Contribution "1" --> "0..*" Commit : includes
    Tracker "1" --> "0..*" Contribution : manages

🔍 Explanation of Design Decisions:
User–Contribution is a one-to-many relationship.

Contribution–Repository is a one-to-one link (a contribution is tied to a single repository).

Contribution–Commit is a one-to-many relationship.

Tracker manages all contributions in the system.

We use simple inheritance/association instead of over-complicating with composition at this point.
