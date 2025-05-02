package assignment12.controller;

import assignment12.BookService;

public class BookController {

    private final BookService bookService = new BookService();

    public String addBook(String title, String author) {
        return bookService.addBook(title, author);
    }
}

