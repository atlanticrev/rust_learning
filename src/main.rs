fn takes_ownership(some_string: String) {
    // "some_string" will be freed at the end of this scope
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    // "some_string" will be moved to the caller scope
    let some_string = String::from("hello two");
    some_string
}

fn get_len(s: &String) -> usize {
    // refers to the value of "str" but does not own it
    s.len()
}

fn print_chars(s: &str) {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        println!("element [{}] is {}", i, item);
    }
}

fn main() {
    // Ownertship examples (moving)
    let s = String::from("hello");
    println!("{}", s);
    let s1 = s; // "s" moved to "s1" (moving works only with heap allocated data)
    println!("{}", s1);
    let s2 = s1.clone(); // "s" copied with heap data to "s2"
    println!("{}", s2);
    takes_ownership(s1); // "s1" moved to "takes_ownership" argument "some_string"


    // Copy of primitive types (no moving)
    let n = 5;
    let n1 = n;
    println!("{}, {}", n, n1);


    // Ownership with functions and references
    let str = gives_ownership();
    println!("{}", str);
    let len = get_len(&str);
    println!("length of a string is: {}", len);
    print_chars(&str);


    // Structs
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: i64,
        active: bool,
    }

    let user1 = User {
        username: String::from("asd"),
        email: String::from("asd@ya.ru"),
        active: false,
        sign_in_count: 1
    };

    let user2 = User {
        username: String::from("qwe"),
        email: String::from("qwe@mail.ru"),
        ..user1 // Shorthand as in js
    };
    println!("shape of user2 is: {:?}", user2);

    fn build_user(email: String, username: String) -> User {
        User {
            email, // Shorthand as in js
            username, // Shorthand as in js
            active: true,
            sign_in_count: 1,
        }
    }

    let user3 = build_user(String::from("rty@ya.ru"), String::from("rty"));
    println!("email is: {}", user3.email);


    // Tuple structs
    struct Point(i32, i32, i32);
    let point1 = Point(0, 0, 0);
    println!("point is {}", point1.0);
    // let (x, y) = point1; // it dosen't work?
    // println!("x and y of a point is {}, and {}", x, y);


    // Structs usage, methods of the structs, adding functionality with traits
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // "Constructor" function
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    let rect1 = Rectangle {
        width: 10,
        height: 20
    };

    let sqr1 = Rectangle::square(300);

    println!("area of rectangle is: {}", rect1.area());
    println!("area of rectangle is: {}", rect1.area());
    println!("shape of rectangle is {:#?}", rect1);
    println!("sqare of square is {:#?}", sqr1);


    // Enums and match
    #[derive(Debug)]
    enum IpAddrKind {
        V4(String),
        V6(String),
    }

    fn route(ip_kind: IpAddrKind) {
        println!("{:?}", ip_kind);
    }

    route(IpAddrKind::V4(String::from("127.0.0.1")));
    route(IpAddrKind::V6(String::from("::1")));

    #[derive(Debug)]
    enum Message {
        Quit,
        Move{x: i32, y: i32}, // Data associated with enum variant
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("{:?}", self)
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("state of the coin is: {:?}", state);
                25
            },
        }
    }

    println!("the coin is {}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
