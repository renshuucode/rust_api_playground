trait Printable {
    fn print(&self);
}

struct Book {
    title: String,
}

impl Printable for Book {
    fn print(&self) {
        println!("Book title: {}", self.title);
    }
}

fn main() {
    let book = Book {
        title: String::from("The Great Gatsby"),
    };
    book.print();
}