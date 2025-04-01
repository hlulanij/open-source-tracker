# Activity Workflow Diagram for User Registration

```mermaid
graph TD
    A[Start] --> B[Enter Email]
    B --> C[Enter Password]
    C --> D[Submit Registration]
    D --> E[Email Validation]
    E -->|Valid| F[Account Creation]
    E -->|Invalid| G[Show Error - Re-enter Email]
    G --> B
    F --> H[Send Confirmation Email]
    H --> I[Registration Successful]
    I --> J[Redirect to Login]
    J --> K[End]
