# Reflection on Domain Model and Class Diagram Design

## Challenges Faced
Designing the domain model and class diagram involved several challenges. One of the biggest difficulties was abstracting the system into key entities that captured all relevant aspects without becoming too complex. I had to carefully define the boundaries of each class and determine which attributes and methods were essential for fulfilling the system's core functionality.

The relationships between entities were also tricky, especially deciding on the multiplicities and associations. For example, the **User** can contribute to many repositories, but the **Contribution** must be linked to a **Repository** and a **User**, which required a precise understanding of their interactions.

## Alignment with Prior Work
The class diagram and domain model closely align with the previous assignments:
- The **User** entity reflects the user data model used in the use case diagrams from Assignment 5.
- The **Contribution** class is a direct reflection of the contributions tracked in the system’s functional requirements.
- Relationships like **User–Contribution** and **Contribution–Repository** align with how the system was intended to work, as outlined in the use cases and system specifications.

## Trade-Offs and Design Decisions
A few trade-offs were made to simplify the design:
- I decided to use **association** instead of **composition** between classes like **User** and **Contribution**, as the relationship doesn't imply that a **User** "owns" a **Contribution** permanently.
- I avoided complex inheritance hierarchies and kept it simple by focusing on essential relationships.

This approach keeps the system flexible and easy to maintain as new features can be added without significant changes to the existing model.

## Lessons Learned about Object-Oriented Design
Through this process, I gained a deeper understanding of object-oriented design principles, particularly the importance of clearly defining entity

