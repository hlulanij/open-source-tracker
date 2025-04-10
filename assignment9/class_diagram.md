classDiagram
class User {
    -userId: String
    -name: String
    -email: String
    +addContribution(repo: String, numCommits: Int): void
    +listContributions(): List<Contribution>
}

class Contribution {
    -contributionId: String
    -repoName: String
    -numCommits: Int
    +updateCommits(numCommits: Int): void
}

class Repository {
    -repoId: String
    -name: String
    -url: String
    +fetchContributors(): List<User>
}

User "1" -- "0..*" Contribution : makes
Contribution "0..*" -- "1" Repository : belongsTo
