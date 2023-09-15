// We define a struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // Now we instantiate a struct
    let mut user1 = User {
        email: String::from("someuser@example.com"),
        username: String::from("This person"),
        active: true,
        sign_in_count: 0,
    };

    // In order to perform this mutation, the entire User
    // instance must be mutable.
    user1.email = String::from("whoops@differentdomain.com");

    let user2 = User {
        email: String::from("dummy string"),
        username: String::from("another dummy string"),
        active: false,
        sign_in_count: 2,
    };

    // Can't do this unless all of user2 is mutable!
    // Rust does not permit structs to have some mutable
    // fields while other fields are immutable.
    //
    // user2.email = String::from("corrected@email.com");
    println!("User1:\nName:\t{}\nMail:\t{}\nActv:\t{}\nSeen:\t{}\n", user1.username, user1.email, user1.active, user1.sign_in_count);
    println!("User2:\nName:\t{}\nMail:\t{}\nActv:\t{}\nSeen:\t{}\n", user2.username, user2.email, user2.active, user2.sign_in_count);

    // It can be a pain to have to manually instantiate structs. Let's use a factory
    let user3 = user_factory_singleton("hopeful1@protonmail.ch".to_string(), "Bob Hope".to_string());
    let mut user4 = user_factory_singleton("watchtower@somemail.cz".to_string(), "Bob Dylan".to_string());

    give_user_details(&user3);
    give_user_details(&user4);

    // And here's a handy way to update some user data:
    user1 = update_username(user1, "Janice Joplin".to_string());
    user1 = update_active(user1);
    user4 = update_email(user4, "watchtower@allalong.tx".to_string());
    user4 = update_sign_in_count(user4);

    give_user_details(&user1);
    give_user_details(&user4);
}

fn user_factory_singleton(email: String, username: String) -> User {
    User {
        active: true,
        sign_in_count: 0,
        username,           //field init shorthand, instead of username: username,
        email,              //field init shorthand, instead of email: email,
    }
}

fn give_user_details(user: &User) {
    println!("User Details:\nName:\t{}\nMail:\t{}\nActv:\t{}\nSeen:\t{}\n", user.username, user.email, user.active, user.sign_in_count);
}

fn update_username(user: User, new_name: String) -> User {
    User {
        username: new_name,
        ..user
    }
}

fn update_email(user: User, new_email: String) -> User {
    User {
        email: new_email,
        ..user
    }
}

fn update_active(user: User) -> User {
    User {
        active: !user.active,
        ..user
    }
}

fn update_sign_in_count(user: User) -> User {
    User {
        sign_in_count: user.sign_in_count + 1,
        ..user
    }
}