use std::rc::Rc;

struct User {
    name: String,
}

struct Profile {
    user: Rc<User>,
}

struct Prefs {
    user: Rc<User>,
}

fn main() {
    let first = Rc::new(User {
        name: String::from("Julia"),
    });

    let my_profile = Profile {
        user: Rc::clone(&first),
    };

    let my_prefs = Prefs {
        user: Rc::clone(&first),
    };

    println!("{}", my_profile.user.name);
    println!("{}", my_prefs.user.name);
}
