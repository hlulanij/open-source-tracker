classDiagram
class User {
    -userId: String
    -name: String
    +borrowBook()
    +returnBook()
}
class Book {
    -bookId: String
    -title: String
    -status: String
    +checkOut()
    +return()
}
class Loan {
    -loanId: String
    -dueDate: Date
    +calculateFine()
}
User "1" -- "0..*" Loan : borrows
Book "1" -- "0..1" Loan : associatedWith
