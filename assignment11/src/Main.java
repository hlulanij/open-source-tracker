package assignment11.src;

import assignment11.src.domain.Book;
import assignment11.src.repositories.BookRepository;
import assignment11.src.repositories.RepositoryFactory;

import java.util.List;

public class Main {
    public static void main(String[] args) {
        // Create a BookRepository using the Factory
        BookRepository bookRepo = RepositoryFactory.createBookRepository("memory");

        // Create and save a new Book
        Book book1 = new Book("1", "Open Source 101", "Jane Dev");
        bookRepo.save(book1);

        // Read by ID
        bookRepo.findById("1").ifPresent(book -> 
            System.out.println("Found: " + book.getTitle() + " by " + book.getAuthor())
        );

        // Add another book
        Book book2 = new Book("2", "Collaboration Culture", "John Hacker");
        bookRepo.save(book2);

        // Read all
        List<Book> allBooks = bookRepo.findAll();
        System.out.println("All books in repo:");
        for (Book b : allBooks) {
            System.out.println("- " + b.getTitle());
        }

        // Delete one
        bookRepo.delete("1");
        System.out.println("After deletion:");
        bookRepo.findAll().forEach(b -> System.out.println("- " + b.getTitle()));
    }
}

