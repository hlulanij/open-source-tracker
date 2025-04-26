package assignment11.src.repositories.inmemory;

import assignment11.src.domain.Book;
import assignment11.src.repositories.BookRepository;

import java.util.*;

/**
 * In-memory implementation of BookRepository using a HashMap.
 */
public class InMemoryBookRepository implements BookRepository {
    private final Map<String, Book> storage = new HashMap<>();

    @Override
    public void save(Book book) {
        storage.put(book.getId(), book);
    }

    @Override
    public Optional<Book> findById(String id) {
        return Optional.ofNullable(storage.get(id));
    }

    @Override
    public List<Book> findAll() {
        return new ArrayList<>(storage.values());
    }

    @Override
    public void delete(String id) {
        storage.remove(id);
    }
}

