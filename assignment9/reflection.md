## Domain Model for Open Source Contribution Tracker

### Key Domain Entities

1. **User**
   - **Attributes:**
     - `userId`: Unique identifier for the user.
     - `name`: Name of the user.
     - `email`: Email address associated with the user's GitHub account.
   - **Methods:**
     - `addContribution()`: Adds a new contribution entry for the user.
     - `removeContribution()`: Removes a contribution from the user's record.
     - `viewContributions()`: Displays all contributions made by the user.
   - **Relationships:**
     - A `User` can have multiple `Contributions`. This is a **one-to-many** relationship.
   
2. **Contribution**
   - **Attributes:**
     - `repositoryName`: Name of the repository where the user contributed.
     - `commitCount`: Number of commits made by the user to the repository.
     - `contributionDate`: Date when the contribution was made.
   - **Methods:**
     - `add()`: Adds a contribution entry for a user.
     - `update()`: Updates the details of an existing contribution (e.g., commit count).
     - `delete()`: Deletes the contribution from the system.
   - **Relationships:**
     - A `Contribution` is linked to one `User` (many contributions to one user) and one `GitHubAPI` for fetching contribution data. This is a **many-to-one** relationship with `User` and a **many-to-one** relationship with `GitHubAPI`.

3. **GitHubAPI**
   - **Attributes:**
     - `apiUrl`: The URL endpoint of the GitHub API used to fetch contribution data.
     - `authenticationToken`: A token used for authentication when interacting with the API.
   - **Methods:**
     - `fetchContributionData()`: Fetches contribution data for a specific user and repository from the GitHub API.
     - `validateUser()`: Verifies if a user exists on GitHub using their username.
   - **Relationships:**
     - A `GitHubAPI` instance is used to fetch data for multiple `Contributions`. It is associated with many contributions, making this a **one-to-many** relationship.

### Relationships Between Entities

- **User ↔ Contribution**: A **one-to-many** relationship. A user can have multiple contributions, but each contribution is linked to a single user.
- **Contribution ↔ GitHubAPI**: A **many-to-one** relationship. A contribution is linked to one GitHub API instance, which fetches the data for the contribution.

### Business Rules
- **Contribution Limit**: A user can only log contributions for repositories they have contributed to.
- **User Login**: A user must log in before they can add or view contributions.
- **Unique Contribution**: Each contribution is uniquely identified by the combination of the `userId` and `repositoryName`.
- **GitHub Account Validation**: Contributions can only be added for users with verified GitHub accounts (authenticated through the GitHub API).
- **Data Integrity**: Contribution data must be updated whenever the number of commits is modified via the GitHub API.

### Conclusion
The domain model accurately reflects the business logic and relationships necessary to track open-source contributions. The entities are clearly defined with specific attributes and methods, and the relationships are modeled according to how users, contributions, and the GitHub API interact. This model supports the core functionality of the system, ensuring that contributions are tracked, logged, and managed efficiently.

