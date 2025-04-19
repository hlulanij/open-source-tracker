# Architecture Documentation

## 🏛️ C4 Diagrams

The following diagrams are part of the **C4 Architecture** model, which divides the system into different levels of abstraction. We’ll cover three levels here:

1. **Context Diagram** (C4 Level 1)
2. **Container Diagram** (C4 Level 2)
3. **Component Diagram** (C4 Level 3)

### 📍 1. Context Diagram (C4 Level 1)
This diagram shows the system’s high-level interactions with external entities. It provides a big-picture view of how users and external systems interact with the tracker.

```mermaid
graph LR
    User((User))
    OpenSourceTracker((Open Source Tracker))
    User --> OpenSourceTracker
    OpenSourceTracker -->|Logs Contributions| GitHub((GitHub))

    class User,OpenSourceTracker,GitHub external

