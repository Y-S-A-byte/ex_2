use book_lib::{Book, Magazine};
use crate::book_lib::Publication;
mod book_lib;
fn main() {
    let _books: Vec<Publication> = vec![
        Publication::Book(Book {
            title: String::from("The Rust Programming Language"),
            author: String::from("Steve Klabnik and Carol Nichols"),
            page_count: 569,
        }),
        Publication::Magazine(Magazine {
            title: String::from("Time"),
            issue: 1025,
            topic: String::from("Technology"),
        }),
    ];

    read_to_array(&_books);
}

fn read_to_array(_book_list: &Vec<Publication>){
    for book in _book_list {
        print_book(book);
    }
}

fn print_book(_book: &Publication) {
    println!();
    match _book {
        Publication::Book(book) => {
            println!(
                "Kitap: {}\nyazar: {}\nsayfa: {}\n",
                book.title, book.author, book.page_count
            );
        }
        Publication::Magazine(magazine) => {
            println!(
                "Dergi: {}\nSayı: {}\nKonu: {}\n",
                magazine.title, magazine.issue, magazine.topic
            );
        }
    }
}