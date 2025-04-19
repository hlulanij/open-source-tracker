import json
import os
from datetime import datetime

DATA_FILE = "contributions.json"

# Load existing data or create new
def load_contributions():
    if os.path.exists(DATA_FILE):
        with open(DATA_FILE, "r") as file:
            return json.load(file)
    else:
        return []

# Save to JSON
def save_contributions(data):
    with open(DATA_FILE, "w") as file:
        json.dump(data, file, indent=4)

# Add new contribution
def add_contribution():
    repo = input("Repository name: ")
    date = input("Date (YYYY-MM-DD): ")
    message = input("Commit message: ")
    try:
        commits = int(input("Number of commits: "))
    except ValueError:
        print("Invalid number of commits.")
        return

    contribution = {
        "repository": repo,
        "date": date,
        "message": message,
        "commits": commits
    }

    contributions = load_contributions()
    contributions.append(contribution)
    save_contributions(contributions)
    print("✅ Contribution added!\n")

# View all contributions
def view_contributions():
    contributions = load_contributions()
    if not contributions:
        print("No contributions found.\n")
        return

    for i, c in enumerate(contributions, 1):
        print(f"{i}. {c['date']} - {c['repository']} ({c['commits']} commits)")
        print(f"   Message: {c['message']}")
    print()

# View summary
def view_summary():
    contributions = load_contributions()
    total_commits = sum(c["commits"] for c in contributions)
    unique_repos = set(c["repository"] for c in contributions)
    print(f"📦 Total contributions: {len(contributions)}")
    print(f"📈 Total commits: {total_commits}")
    print(f"📁 Repositories involved: {len(unique_repos)}\n")

# Menu
def main():
    while True:
        print("=== Open Source Tracker ===")
        print("1. Add contribution")
        print("2. View all contributions")
        print("3. View summary")
        print("4. Exit")
        choice = input("Choose an option: ")

        if choice == "1":
            add_contribution()
        elif choice == "2":
            view_contributions()
        elif choice == "3":
            view_summary()
        elif choice == "4":
            print("👋 Goodbye!")
            break
        else:
            print("❌ Invalid choice. Try again.\n")

if __name__ == "__main__":
    main()

