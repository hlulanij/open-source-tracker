package assignment12.services;

import assignment12.models.Book;
import assignment12.repositories.BookRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.Optional;

@Service
public class BookService {

    private final BookRepository bookRepository;

    @Autowired
    public BookService(BookRepository bookRepository) {
        this.bookRepository = bookRepository;
    }

    public Book checkoutBook(String bookId) {
        Optional<Book> book = bookRepository.findById(bookId);
        if (!book.isPresent()) {
            throw new RuntimeException("Book not found");
        }
        Book existingBook = book.get();
        if (existingBook.isCheckedOut()) {
            throw new RuntimeException("Book is already checked out");
        }
        existingBook.setCheckedOut(true);
        return bookRepository.save(existingBook);
    }

    // Other business logic methods (e.g., create book, update book, etc.)
}
