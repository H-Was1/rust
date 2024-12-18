use std::cell::RefCell;
use std::rc::Rc;

struct Database {
    max_connections: u32,
}

struct Auth_Service {
    db: Rc<RefCell<Database>>,
}

struct User_Service {
    db: Rc<RefCell<Database>>,
}

fn main() {
    // This code demonstrates a common pattern in Rust using Rc (Reference Counting)
    // and RefCell for interior mutability to share mutable state
    let db = Rc::new(RefCell::new(Database {
        max_connections: 10,
    }));
    // Create a Database instance wrapped in Rc and RefCell
    // - Rc allows multiple owners of the data
    // - RefCell allows mutable access at runtime
    let auth_service = Auth_Service { db: Rc::clone(&db) };
    let user_service = User_Service { db: Rc::clone(&db) };
    // Create services that share the same database connection
    // Rc::clone creates a new reference to the same data
    db.borrow_mut().max_connections = 20;
    // Mutably borrow the database to modify max_connections
    // Note: At runtime, RefCell ensures there's only one mutable borrow at a time
    // If another mutable borrow exists, this would panic

    println!("max_connections: {}", db.borrow().max_connections);
}
