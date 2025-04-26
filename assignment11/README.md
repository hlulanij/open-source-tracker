# Assignment 11: Implementing a Persistence Repository Layer

## ✅ Objective
This assignment implements a persistence layer for storing domain entities using the Repository design pattern and abstraction via a factory mechanism.

---

## 📁 Project Structure

---

## ✅ Design Justification

- **Generic Repository Interface**:  
  We used generics in the `Repository<T, ID>` interface to avoid duplication across different entity repositories (e.g. `BookRepository`).

- **In-Memory Implementation**:  
  CRUD operations are implemented using a `HashMap`, allowing fast prototyping.

- **Factory Pattern for Abstraction**:  
  A `RepositoryFactory` decouples the service from storage logic, enabling future switching to JSON, SQL, or API storage with minimal changes.

---

## 🔍 Example Usage

Run `Main.java` to:

- Add books to the repository
- Read a book by ID
- View all books
- Delete a book

---

## 📌 Backlog / Issues

- [x] Create a generic `Repository<T, ID>` interface
- [x] Create entity-specific `BookRepository` interface
- [x] Implement `InMemoryBookRepository` using `HashMap`
- [x] Create a `RepositoryFactory` to abstract storage choice
- [x] Demonstrate usage in `Main.java`
- [ ] Future: Add FileSystem (JSON/XML) storage backend
- [ ] Future: Add SQL/NoSQL storage options (MySQL, MongoDB)
- [ ] Future: Add external API-based repository option

---

## 🧠 Reflections

This modular design ensures high flexibility, low coupling, and easy testability. By abstracting the storage logic, the system remains open to extension but closed to modification — following SOLID principles.
