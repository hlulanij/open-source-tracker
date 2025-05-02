package assignment12.controller;

import assignment12.BorrowService;

public class BorrowController {

    private final BorrowService borrowService = new BorrowService();

    public String borrowBook(String userId, String bookId) {
        return borrowService.borrowBook(userId, bookId);
    }
}

