# Branch Protection Rules

These rules are enabled to maintain code quality, encourage collaboration, and avoid accidental issues in production.

## Rules Applied to `main` Branch:
- ✅ All code must go through a Pull Request
- ✅ At least one reviewer must approve each PR
- ✅ All status checks (CI tests) must pass before merging
- ✅ Direct pushes to `main` are blocked to avoid accidental changes
- ✅ Optional: Administrators must also follow these rules

## Why This Matters:
- 🔒 Prevents broken or untested code from being merged into `main`
- 👥 Encourages team collaboration through PR reviews
- 🧪 Ensures CI workflows validate the code before it goes live
- 📈 Builds trust and stability in the project over time
