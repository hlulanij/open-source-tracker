package assignment11.src.repositories;

import java.util.List;
import java.util.Optional;

/**
 * Generic Repository interface defining basic CRUD operations.
 */
public interface Repository<T, ID> {
    void save(T entity);                      // Create or Update
    Optional<T> findById(ID id);              // Read single
    List<T> findAll();                        // Read all
    void delete(ID id);                       // Delete
}

