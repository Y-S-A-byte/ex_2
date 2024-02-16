
pub struct Book {
    pub title : String,
    pub author: String,
    pub page_count: i32
}

pub struct Magazine {
    pub title : String,
    pub issue: i32,
    pub topic: String 
}


pub enum Publication {
    Book(Book),
    Magazine(Magazine)
}