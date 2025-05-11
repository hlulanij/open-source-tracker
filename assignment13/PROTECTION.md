# Branch Protection Rules

These rules are enabled to maintain code quality, encourage collaboration, and avoid accidental issues in production.

## Rules Applied to `main` Branch:
- ✅ All code must go through a Pull Request
- ✅ At least one approval is required
- ✅ All status checks must pass (CI tests)
- ✅ Direct pushes to main are disabled
- ✅ Optional: Admins are also restricted

## Why It Matters:
- Prevents bugs from being pushed directly
- Enforces code review best practices
- Automates quality checks via GitHub Actions
- Keeps main always in a deployable state

