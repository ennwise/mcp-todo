# Agile Development Lifecycle Workflow

* **File Name Suggestion:** `agile_development_lifecycle_workflow.md`
* **Version:** 1.1
* **Last Updated:** 2025-05-20
* **Author/Maintainer:** ManagementMode

## 1. Purpose & Goal

* **Primary Objective:** To consistently deliver valuable, high-quality software iteratively and incrementally, adapting to changing requirements and incorporating continuous feedback to maximize value for users and the business.
* **Scope:**
    * **In Scope:** This workflow covers the end-to-end agile software development lifecycle, including product backlog management, sprint planning, iterative development and testing, sprint reviews, sprint retrospectives, release planning, and continuous improvement of the process. It orchestrates and integrates specialized sub-workflows for feature development, bug fixing, etc.
    * **Out of Scope:** Detailed pre-project portfolio management, long-term strategic business planning (though it consumes outputs from these), specific hardware infrastructure provisioning (though it relies on available environments).
* **Intended Users/Modes:** ManagementMode (as primary orchestrator/facilitator), ProductManagerMode, LeadDeveloperMode, RustDeveloperMode, DeveloperMode(s), QAMode.
* **Success Indicators (High-Level):**
    * Regular delivery of working software.
    * High stakeholder and user satisfaction.
    * Adaptability to changing priorities.

## 2. Guiding Principles & Philosophy

* **Agile Manifesto Adherence:** Prioritize individuals and interactions, working software, customer collaboration, and responding to change.
* **Iterative & Incremental Delivery:** Deliver functional software in short, regular cycles (Sprints) to gather feedback and provide value sooner.
* **Continuous Feedback & Learning:** Actively seek feedback from all stakeholders and use retrospectives to improve processes and team performance.
* **Empowered Cross-Functional Teams:** Teams have the autonomy and skills needed to deliver an increment of working software.
* **Technical Excellence & Sustainable Pace:** Maintain high code quality and design standards while ensuring the team can work sustainably.
* **Transparency:** Make progress, impediments, and plans visible to all relevant parties.

## 3. Key Roles Involved (Typical Modes & Responsibilities)

* **Primary Coordinator(s):**
    * **Mode/Role:** ManagementMode
    * **Key Responsibilities:** Facilitating the overall agile process, ensuring adherence to agile principles, removing organizational impediments, resource coordination (if needed beyond team self-organization), reporting on overall progress and workflow health.
    * **Mode/Role:** ProductManagerMode (Product Owner proxy)
    * **Key Responsibilities:** Defining product vision, owning and prioritizing the Product Backlog, representing customer and stakeholder needs, accepting completed work.
    * **Mode/Role:** LeadDeveloperMode (Scrum Master/Tech Lead proxy)
    * **Key Responsibilities:** Facilitating sprint events, coaching the team on agile practices, guiding technical execution, helping resolve technical impediments, ensuring Definition of Done is followed.
* **Key Contributor(s) (The Development Team):**
    * **Mode/Role:** DeveloperMode(s), RustDeveloperMode
    * **Key Responsibilities:** Implementing backlog items (features, bug fixes, tech debt), writing unit tests, participating in code reviews, collaborating on design.
    * **Mode/Role:** QAMode(s)
    * **Key Responsibilities:** Test planning and execution within sprints, developing automated tests, ensuring quality standards, bug reporting and verification.
    * **Mode/Role:** ArchitectMode (Consulted)
    * **Key Responsibilities:** Providing guidance on significant architectural decisions, reviewing designs for alignment with an overall technical strategy.
    * **Mode/Role:** TechnicalWriterMode (As needed)
    * **Key Responsibilities:** Creating and updating user/developer documentation for delivered increments.
* **Stakeholder(s) (for review/approval/consultation):**
    * **Mode/Role:** BusinessStakeholderMode
    * **Key Responsibilities:** Providing input on priorities, participating in Sprint Reviews, providing feedback on the product increment.
    * **Mode/Role:** EndUserRepresentativeMode (or insights from ProductManagerMode)
    * **Key Responsibilities:** Providing user perspective and feedback.

## 4. Workflow Phases / Stages / Cycles

This workflow primarily consists of iterative Sprint Cycles, supported by continuous activities.

---
### **Cycle A: Sprint Execution (Iterative Cycle; e.g., 2 Weeks)**

#### **A.1. Phase: Sprint Planning**

* **Objective:** Define the Sprint Goal and select Product Backlog Items (PBIs) that the Development Team forecasts it can complete to achieve that goal, along with a plan for how to do so.
* **Triggers (Inputs):**
    * [X] Prioritized & Refined Product Backlog (from ongoing Product Backlog Refinement).
    * [X] Latest team velocity/capacity data.
    * [X] Business priorities & feedback from previous Sprint Review.
    * [X] Defined "Definition of Done" (DoD).
* **Key Activities & Jobs (Checklist & Task Templates):**
    * [X] **Activity A.1.1:** Define Sprint Goal
        * **Description:** ProductManagerMode proposes a business objective; the entire Scrum Team collaborates to craft a Sprint Goal.
        * **Typical Mode(s) Responsible:** ProductManagerMode, LeadDeveloperMode, DeveloperMode(s), QAMode.
        * **Expected Output:** A clear, concise Sprint Goal.
        * **Acceptance Criteria:** Sprint Goal is understood and agreed upon by the team.
    * [X] **Activity A.1.2:** Select Product Backlog Items
        * **Description:** The Development Team selects PBIs from the top of the Product Backlog that they forecast they can complete within the Sprint to meet the Sprint Goal. ProductManagerMode clarifies items.
        * **Typical Mode(s) Responsible:** DeveloperMode(s), QAMode, LeadDeveloperMode, with input from ProductManagerMode.
        * **Inputs:** Refined PBIs (often outputs from processes like `./.ennwise/management_workflows/feature_development_plan.md` or `./.ennwise/management_workflows/bug_triage_resolution_workflow.md`).
        * **Expected Output:** Selected PBIs forming the initial Sprint Backlog.
        * **Acceptance Criteria:** Team is confident they can complete selected PBIs to DoD.
    * [X] **Activity A.1.3:** Decompose Selected PBIs into Tasks
        * **Description:** The Development Team breaks down selected PBIs into smaller, actionable tasks (often 1-2 days of work) necessary to complete each PBI according to the DoD.
        * **Typical Mode(s) Responsible:** DeveloperMode(s), QAMode, LeadDeveloperMode.
        * **Task Checklist (Template for each PBI):**
            * [ ] Design task (if needed)
            * [ ] Implementation sub-task(s)
            * [ ] Unit test development sub-task
            * [ ] Code review sub-task
            * [ ] QA testing task (test case creation & execution)
            * [ ] Documentation update task (if applicable)
        * **Expected Output:** Detailed Sprint Backlog with tasks and initial estimates in `task-manager-server`.
        * **Acceptance Criteria:** All selected PBIs have a corresponding plan of tasks.
* **Tools & Resources:** `task-manager-server`, collaborative whiteboard/tool.
* **Deliverables & Outputs for this Phase (Checklist):**
    * [X] Sprint Goal