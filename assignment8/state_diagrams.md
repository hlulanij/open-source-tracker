# State Transition Diagrams

```mermaid
stateDiagram-v2
    [*] --> Registered
    Registered --> Active : User logs in
    Active --> Suspended : Admin suspends account
    Suspended --> Active : User appeals suspension
    Active --> Deactivated : User deletes account
    Deactivated --> [*]

    [*] --> Fetched : System retrieves contribution
    Fetched --> Processed : Contribution is analyzed
    Processed --> Stored : Contribution saved to database
    Stored --> Archived : User exports data
    Stored --> Deleted : User removes contribution
    Archived --> [*]
    Deleted --> [*]

    [*] --> Requested : User selects export option
    Requested --> Processing : System compiles data
    Processing --> Completed : Report is generated
    Completed --> Downloaded : User downloads the report
    Completed --> Expired : Report expires after time limit
    Expired --> [*]
    Downloaded --> [*]











