# Project Planning Workflow

* **File Name Suggestion:** `project_planning_workflow.md`
* **Version:** 1.0
* **Last Updated:** 2025-05-21
* **Author/Maintainer:** ManagementMode

## 1. Purpose & Goal

* **Primary Objective:** To create a comprehensive project plan that outlines the project's scope, objectives, deliverables, timelines, resources, risks, and communication plan.
* **Scope:**
    * **In Scope:** Defining project goals, identifying stakeholders, outlining deliverables, creating a work breakdown structure (WBS), estimating effort and duration, scheduling tasks, allocating resources, identifying potential risks, and establishing a communication plan.
    * **Out of Scope:** Detailed execution of project tasks, ongoing project monitoring and control (covered by other workflows), specific technical design documents (unless a high-level overview is part of the plan).
* **Intended Users/Modes:** ManagementMode, ArchitectMode, LeadDeveloperMode, potentially QAMode for input on testing timelines.
* **Success Indicators (High-Level):**
    * A clear, actionable project plan document is produced.
    * Stakeholders understand and agree on the project scope, objectives, and timelines.
    * The plan provides a solid foundation for project execution.

## 2. Guiding Principles & Philosophy

* **Clarity & Precision:** The project plan must be clear, concise, and unambiguous.
* **Comprehensiveness:** Address all critical aspects of project planning.
* **Realism:** Timelines, resource allocation, and risk assessment should be realistic.
* **Collaboration:** Involve relevant stakeholders and modes in the planning process.
* **Adaptability:** While aiming for a solid plan, acknowledge that plans may need adjustments as the project progresses (though the initial plan should be as robust as possible).

## 3. Key Roles Involved (Typical Modes & Responsibilities)

* **Primary Coordinator(s):**
    * **Mode/Role:** ManagementMode
    * **Key Responsibilities:** Orchestrating the planning process, consolidating inputs, ensuring all sections of the plan are covered, facilitating reviews.
* **Key Contributor(s):**
    * **Mode/Role:** ArchitectMode
    * **Key Responsibilities:** Defining technical scope, high-level system design considerations, identifying technical risks, input on technical task breakdown.
    * **Mode/Role:** LeadDeveloperMode
    * **Key Responsibilities:** Input on task estimation, resource requirements for development, potential technical challenges.
    * **Mode/Role:** (Potentially) QAMode
    * **Key Responsibilities:** Input on testing scope, effort estimation for QA activities, and integration of QA into the project timeline.
    * **Mode/Role:** (Potentially) TechnicalWriterMode
    * **Key Responsibilities:** Assisting in drafting and structuring the project plan document.
* **Stakeholder(s) (for review/approval/consultation):**
    * **Mode/Role:** User/Client representative (if applicable, simulated or actual)
    * **Key Responsibilities:** Providing project requirements, clarifying objectives, approving the final plan.

## 4. Workflow Phases / Stages / Cycles

---
### Phase 1: Initiation & Scope Definition

* **Objective:** To clearly define the project's purpose, objectives, high-level scope, and identify key stakeholders.
* **Triggers (Inputs):**
    * [ ] User Request: "Create a project plan for [Project X]"
    * [ ] Initial project idea or problem statement.
* **Key Activities & Mode Assignments:**
    1.  **Clarify Project Goals & Objectives:**
        *   **Mode:** ManagementMode (leading), with input from ArchitectMode.
        *   **Output:** Documented project goals and objectives.
    2.  **Define High-Level Scope:**
        *   **Mode:** ManagementMode, ArchitectMode.
        *   **Output:** Initial scope statement (inclusions, exclusions).
    3.  **Identify Key Stakeholders:**
        *   **Mode:** ManagementMode.
        *   **Output:** List of key stakeholders and their roles/interests.
* **Deliverables/Outputs:**
    * [ ] Approved Project Charter (or equivalent initial document).
    * [ ] High-level Scope Statement.
    * [ ] Stakeholder Register.
* **Success Criteria:**
    * [ ] Project objectives are SMART (Specific, Measurable, Achievable, Relevant, Time-bound).
    * [ ] Scope boundaries are clearly understood by key stakeholders.

---
### Phase 2: Detailed Planning & WBS

* **Objective:** To break down the project into manageable tasks, estimate effort and duration, and define deliverables.
* **Triggers (Inputs):**
    * [ ] Approved Project Charter / High-level Scope Statement.
    * [ ] Stakeholder Register.
* **Key Activities & Mode Assignments:**
    1.  **Develop Work Breakdown Structure (WBS):**
        *   **Mode:** ManagementMode (facilitating), LeadDeveloperMode, ArchitectMode.
        *   **Output:** Hierarchical WBS.
    2.  **Define Detailed Tasks & Deliverables:**
        *   **Mode:** LeadDeveloperMode, ArchitectMode, (potentially) QAMode.
        *   **Output:** List of tasks with descriptions and associated deliverables.
    3.  **Estimate Effort & Duration for Tasks:**
        *   **Mode:** LeadDeveloperMode, (potentially) QAMode.
        *   **Output:** Effort (e.g., person-days) and duration estimates for each task.
* **Deliverables/Outputs:**
    * [ ] Detailed Work Breakdown Structure (WBS).
    * [ ] Task list with effort and duration estimates.
    * [ ] List of key project deliverables.
* **Success Criteria:**
    * [ ] WBS is comprehensive and covers all defined scope.
    * [ ] Task estimates are realistic and agreed upon by the team.

---
### Phase 3: Scheduling & Resource Allocation

* **Objective:** To create a project schedule, assign resources to tasks, and identify dependencies.
* **Triggers (Inputs):**
    * [ ] Detailed WBS.
    * [ ] Task list with effort and duration estimates.
* **Key Activities & Mode Assignments:**
    1.  **Sequence Activities & Identify Dependencies:**
        *   **Mode:** ManagementMode, LeadDeveloperMode.
        *   **Output:** Task dependency map/network diagram.
    2.  **Develop Project Schedule (Timeline):**
        *   **Mode:** ManagementMode.
        *   **Output:** Project schedule with milestones and deadlines (e.g., Gantt chart).
    3.  **Allocate Resources:**
        *   **Mode:** ManagementMode, with input from LeadDeveloperMode on technical resource needs.
        *   **Output:** Resource allocation plan.
* **Deliverables/Outputs:**
    * [ ] Project Schedule (Gantt chart or similar).
    * [ ] Resource Allocation Plan.
* **Success Criteria:**
    * [ ] Schedule is realistic and accounts for dependencies.
    * [ ] Resources are appropriately assigned to tasks.
    * [ ] Critical path is identified.

---
### Phase 4: Risk Management & Communication Planning

* **Objective:** To identify potential project risks, plan mitigation strategies, and establish a communication plan.
* **Triggers (Inputs):**
    * [ ] Project Schedule.
    * [ ] Resource Allocation Plan.
    * [ ] WBS.
* **Key Activities & Mode Assignments:**
    1.  **Identify Potential Risks:**
        *   **Mode:** ManagementMode, ArchitectMode, LeadDeveloperMode, QAMode.
        *   **Output:** Risk Register with identified risks.
    2.  **Analyze Risks & Plan Responses:**
        *   **Mode:** ManagementMode (leading), with input from relevant modes.
        *   **Output:** Risk Register updated with impact, probability, and mitigation/contingency plans.
    3.  **Develop Communication Plan:**
        *   **Mode:** ManagementMode.
        *   **Output:** Communication plan (stakeholders, frequency, methods).
* **Deliverables/Outputs:**
    * [ ] Risk Register.
    * [ ] Communication Plan.
* **Success Criteria:**
    * [ ] Major risks are identified and have mitigation plans.
    * [ ] Communication plan meets stakeholder needs.

---
### Phase 5: Plan Consolidation & Review

* **Objective:** To consolidate all planning components into a single project plan document and get stakeholder approval.
* **Triggers (Inputs):**
    * [ ] All deliverables from Phases 1-4.
* **Key Activities & Mode Assignments:**
    1.  **Draft Full Project Plan Document:**
        *   **Mode:** ManagementMode, (potentially) TechnicalWriterMode.
        *   **Output:** Draft Project Plan Document.
    2.  **Internal Review:**
        *   **Mode:** ManagementMode, ArchitectMode, LeadDeveloperMode.
        *   **Output:** Feedback on the draft plan.
    3.  **Revise and Finalize Plan:**
        *   **Mode:** ManagementMode.
        *   **Output:** Final Project Plan Document.
    4.  **Stakeholder Review & Approval:**
        *   **Mode:** ManagementMode (presenting), Stakeholders (reviewing).
        *   **Output:** Approved Project Plan.
* **Deliverables/Outputs:**
    * [ ] **Final, Approved Project Plan Document.**
* **Success Criteria:**
    * [ ] Project plan is comprehensive, clear, and internally consistent.
    * [ ] Key stakeholders have reviewed and approved the plan.

## 5. Tools & Technologies (Optional)

* **Task Management:** [e.g., task-manager-server, Jira, Asana, Trello]
* **Document Management:** [e.g., Confluence, SharePoint, Google Drive]
* **Diagramming (for WBS, flowcharts):** [e.g., Lucidchart, draw.io, Visio]
* **Communication:** [e.g., Slack, Microsoft Teams, Email]

## 6. Metrics & Reporting (for the planning process itself)

* **Cycle Time for Planning:** Time taken from initiation to approved project plan.
* **Completeness Score:** Percentage of required plan sections completed thoroughly.
* **Stakeholder Feedback Score:** Satisfaction level of stakeholders with the planning process and output.
* **Number of Revisions:** Iterations required to get the plan approved.

## 7. Continuous Improvement

* **Review & Retrospective:** After a project plan is completed (or a few have been done using this workflow), review the effectiveness of this workflow.
* **Update Triggers:** Significant changes in available tools, mode capabilities, or recurring issues in project planning.
* **Feedback Channels:** [e.g., Direct feedback to ManagementMode, comments in the workflow document.]