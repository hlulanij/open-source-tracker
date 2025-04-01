# State Transition Diagram for User Account

```mermaid
stateDiagram-v2
    [*] --> Registered
    Registered --> Active : User logs in
    Active --> Suspended : Admin suspends account
    Suspended --> Active : User appeals suspension
    Active --> Deactivated : User deletes account
    Deactivated --> [*]




