package com.example.bookservice.controller;

import com.example.bookservice.model.Book;
import com.example.bookservice.service.BookService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/books")
public class BookController {

    private final BookService bookService;

    // Injecting the BookService
    @Autowired
    public BookController(BookService bookService) {
        this.bookService = bookService;
    }

    // 1. Fetch all books (GET /api/books)
    @GetMapping
    public List<Book> getAllBooks() {
        return bookService.getAllBooks();  // Returns a list of all books
    }

    // 2. Create a new book (POST /api/books)
    @PostMapping
    public Book createBook(@RequestBody Book book) {
        return bookService.createBook(book);  // Create a new book and return it
    }

    // 3. Update a book (PUT /api/books/{id})
    @PutMapping("/{id}")
    public Book updateBook(@PathVariable String id, @RequestBody Book bookDetails) {
        return bookService.updateBook(id, bookDetails);  // Update book details based on ID
    }

    // 4. Checkout a book (POST /api/books/{id}/checkout)
    @PostMapping("/{id}/checkout")
    public ResponseEntity<String> checkoutBook(@PathVariable String id) {
        try {
            Book book = bookService.checkoutBook(id);  // Attempt to checkout the book
            return ResponseEntity.ok("Book checked out: " + book.getTitle());
        } catch (RuntimeException e) {
            return ResponseEntity.status(404).body("Error: " + e.getMessage());  // Error if not found
        }
    }
}
