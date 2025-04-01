# State Transition Diagram for User Account

```mermaid
stateDiagram-v2
    [*] --> Registered
    Registered --> Active : User logs in
    Active --> Suspended : Admin suspends account
    Suspended --> Active : User appeals suspension
    Active --> Deactivated : User deletes account
    Deactivated --> [*]




stateDiagram-v2
    [*] --> Fetched : System retrieves contribution
    Fetched --> Processed : Contribution is analyzed
    Processed --> Stored : Contribution saved to database
    Stored --> Archived : User exports data
    Stored --> Deleted : User removes contribution
    Archived --> [*]
    Deleted --> [*]
