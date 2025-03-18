# Agile Planning Document

## 1. User Stories

| Story ID | User Story | Acceptance Criteria | Priority |
|----------|-----------|----------------------|----------|
| US-001   | As a student, I want to register an account so that I can track my contributions. | Successful registration with email validation. | High |
| US-002   | As a student, I want to log in securely so that my data is protected. | Authentication must be secure with hashed passwords. | High |
| US-003   | As a student, I want to add my GitHub username so that my contributions are automatically fetched. | System retrieves contributions from GitHub API. | High |
| US-004   | As a student, I want to view my open-source contributions so that I can track my progress. | Contributions displayed in an organized format. | High |
| US-005   | As a system admin, I want user data encrypted with AES-256 so that security compliance is met. | All sensitive data is stored in encrypted format. | High |
| US-006   | As a student, I want to filter my contributions by repository so that I can analyze my work. | Contributions can be filtered by repository name. | Medium |
| US-007   | As a student, I want to export my contribution data so that I can use it in reports. | Data can be exported in CSV format. | Medium |
| US-008   | As a system admin, I want to monitor system performance so that I can ensure reliability. | System logs track response times and errors. | Low |

## 2. Product Backlog

| Story ID | User Story | Priority (MoSCoW) | Effort Estimate | Dependencies |
|----------|-----------|------------------|----------------|--------------|
| US-001   | Register an account | Must-have | 3 | None |
| US-002   | Log in securely | Must-have | 3 | US-001 |
| US-003   | Add GitHub username | Must-have | 5 | US-002 |
| US-004   | View contributions | Must-have | 5 | US-003 |
| US-005   | Encrypt user data | Must-have | 8 | US-001, US-002 |
| US-006   | Filter contributions | Should-have | 3 | US-004 |
| US-007   | Export contribution data | Could-have | 2 | US-004 |
| US-008   | Monitor system performance | Could-have | 5 | None |

**Justification:** Must-have stories are essential for the core functionality of tracking open-source contributions. Security is prioritized to protect user data. Filtering and exporting are secondary but valuable features.

## 3. Sprint Plan

**Sprint Goal:** Implement core user authentication and GitHub integration for tracking contributions.

| Task ID  | Task Description | Assigned To | Estimated Hours | Status |
|----------|-----------------|-------------|----------------|--------|
| T-001    | Develop user registration system | Dev Team | 8 | To Do |
| T-002    | Implement secure login | Dev Team | 6 | To Do |
| T-003    | Integrate GitHub username feature | Dev Team | 10 | To Do |
| T-004    | Display user contributions | Dev Team | 12 | To Do |

## 4. Reflection

### **Challenges in Prioritization**
Prioritizing user stories was a complex task, especially in balancing security and usability. Security features such as encrypted user data and secure authentication were marked as *must-have* since they directly impact user trust. However, certain usability aspects, like filtering and exporting contributions, had to be deprioritized to *should-have* or *could-have* to ensure core functionality was implemented first. Choosing which stories belonged in Sprint 1 required analyzing dependencies, user needs, and technical feasibility.

### **Challenges in Estimation**
Effort estimation was another challenge. Since Agile relies on relative effort rather than absolute time, using story points helped, but accurately predicting the complexity of tasks was difficult. The *Add GitHub username* story had a higher effort estimate (5) than *Register an account* (3) due to API integration complexity. Similarly, *Encrypt user data* was estimated at 8 due to encryption implementation requirements. Without prior implementation experience in certain areas, there was uncertainty in estimation, which may require future adjustments as we gather more development insights.

### **Aligning Agile with Stakeholder Needs**
In large projects, multiple stakeholders provide varying perspectives, making prioritization easier. In this case, I was both the developer and the stakeholder, meaning I had to balance personal preferences and technical feasibility. At times, I had internal conflicts over whether usability improvements should be included in Sprint 1 or postponed. Ultimately, I decided that ensuring a functional MVP with authentication and contribution tracking was more important than secondary features like data exporting.

### **Lessons Learned**
One key lesson was the importance of dependencies in backlog prioritization. Some tasks, such as *View contributions*, depended on *Add GitHub username*, which in turn required authentication. Identifying these dependencies early helped structure the sprint efficiently. Another lesson was that estimation is an evolving process—some features might take longer than expected, requiring adjustments in future sprints. Finally, breaking down large stories into smaller, testable tasks helped make development more manageable.

### **Future Considerations**
For future sprints, I plan to refine estimation techniques by analyzing Sprint 1 outcomes and improving task breakdown. Additionally, usability improvements should be integrated iteratively instead of being postponed entirely. Agile’s iterative nature means that features like filtering and exporting can be revisited based on user feedback and project constraints. 

Overall, this Agile planning exercise demonstrated the importance of prioritization, estimation, and iterative development in delivering a functional and efficient open-source tracker system.

## 5. GitHub Management
- [ ] Create Issues for User Stories
- [ ] Organize Backlog in GitHub Projects
- [ ] Assign Milestones for Sprint Planning

---


