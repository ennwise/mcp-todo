# Branching Strategy: GitHub Flow

This document outlines the branching strategy adopted for the RustQuote Service project, which is GitHub Flow. This strategy is simple, effective, and well-suited for continuous delivery.

## Core Principles

1.  **`main` is always deployable:** The `main` branch contains production-ready code. All commits on `main` should be stable and deployable.
2.  **Create feature branches from `main`:** All new work, including features, bug fixes, and experiments, must be done on a separate branch (a "feature branch" or "topic branch"). These branches are created from the latest state of `main`.
    *   Branch naming convention: Use descriptive names, optionally prefixed with a type (e.g., `feature/`, `fix/`, `chore/`) and/or an issue tracker ID. For example: `feature/user-authentication` or `fix/TICKET-123-login-bug`.
3.  **Push to remote regularly:** Commit and push your work to the remote feature branch frequently. This helps in backing up your work and allows for early visibility and collaboration if needed.
4.  **Open a Pull Request (PR):** When the work on the feature branch is complete and ready for review, open a Pull Request to merge it into `main`.
    *   The PR should clearly describe the changes made, the problem it solves, and any relevant context.
    *   Ensure all automated checks (e.g., linters, tests, CI builds) pass before requesting a review.
5.  **Review and Discuss:** Team members will review the PR, provide feedback, and discuss any necessary changes. The author of the PR is responsible for addressing the feedback.
6.  **Merge to `main`:** Once the PR is approved and all checks have passed, it can be merged into the `main` branch. Prefer using "squash and merge" or "rebase and merge" to keep the `main` branch history clean and linear, though a regular merge is also acceptable.
7.  **Delete feature branch after merge:** After the feature branch is successfully merged into `main` and the changes are deployed (or confirmed stable), delete the feature branch from both the local and remote repositories. This keeps the repository clean.

## Workflow Summary

1.  Synchronize your local `main` branch with the remote repository.
2.  Create a new feature branch from `main`: `git checkout -b feature/my-new-feature main`
3.  Make your changes, commit them: `git commit -m "Add feature X"`
4.  Push your feature branch to the remote: `git push origin feature/my-new-feature`
5.  Open a Pull Request on the repository platform (e.g., GitHub, GitLab).
6.  Collaborate, address feedback, and wait for approval.
7.  Merge the Pull Request into `main`.
8.  Delete the feature branch.