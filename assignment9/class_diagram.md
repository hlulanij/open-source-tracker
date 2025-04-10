classDiagram
    class User {
        - userId: String
        - name: String
        + logIn()
        + viewProfile()
        + addContribution()
        + listContributions()
    }
    
    class Contribution {
        - contributionId: String
        - repositoryName: String
        - commitCount: Integer
        + add()
        + update()
        + delete()
    }

    class GitHubAPI {
        - apiUrl: String
        + fetchContributions()
    }

    User "1" -- "0..*" Contribution : logs
    User "1" -- "0..1" GitHubAPI : interacts with
    Contribution "0..*" -- "1" GitHubAPI : fetched by

