package assignment11.src.repositories;

import assignment11.src.repositories.inmemory.InMemoryBookRepository;

/**
 * Factory for creating repository instances.
 */
public class RepositoryFactory {

    public static BookRepository createBookRepository(String storageType) {
        if (storageType.equalsIgnoreCase("memory")) {
            return new InMemoryBookRepository();
        }

        // Placeholder: Add support for other storage types (e.g., "json", "sql") here.
        throw new UnsupportedOperationException("Storage type not supported: " + storageType);
    }
}

