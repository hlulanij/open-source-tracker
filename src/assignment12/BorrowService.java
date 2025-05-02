package assignment12.services;

import assignment12.models.Book;
import assignment12.models.User;
import assignment12.repositories.BookRepository;
import assignment12.repositories.UserRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

@Service
public class BorrowService {

    private final BookRepository bookRepository;
    private final UserRepository userRepository;

    @Autowired
    public BorrowService(BookRepository bookRepository, UserRepository userRepository) {
        this.bookRepository = bookRepository;
        this.userRepository = userRepository;
    }

    public String borrowBook(String userId, String bookId) {
        User user = userRepository.findById(userId).orElseThrow(() -> new RuntimeException("User not found"));
        Book book = bookRepository.findById(bookId).orElseThrow(() -> new RuntimeException("Book not found"));

        // Example business logic for limiting books a user can borrow
        if (user.getBorrowedBooks().size() >= 5) {
            throw new RuntimeException("User cannot borrow more than 5 books");
        }

        user.getBorrowedBooks().add(book);
        book.setCheckedOut(true);
        bookRepository.save(book);
        userRepository.save(user);
        return "Book borrowed successfully";
    }
}
