#[derive(Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: i64,
    pub active: bool,
}

impl User {
    pub fn build_user(email: String, username: String) -> User {
        User {
            email, // Shorthand as in js
            username, // Shorthand as in js
            active: true,
            sign_in_count: 1,
        }
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    // "Constructor" function
    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn test() {
    // Structs examples
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
    let user3 = User::build_user(String::from("rty@ya.ru"), String::from("rty"));
    println!("email is: {}", user3.email);
    let rect1 = Rectangle {
        width: 10,
        height: 20
    };
    let sqr1 = Rectangle::square(300);
    println!("area of rectangle is: {}", rect1.area());
    println!("area of rectangle is: {}", rect1.area());
    println!("shape of rectangle is {:#?}", rect1);
    println!("sqare of square is {:#?}", sqr1);


    // Tuple structs examples
    struct Point(i32, i32, i32);
    let point1 = Point(0, 0, 0);
    println!("point is {}", point1.0);
    // let (x, y) = point1; // it dosen't work?
    // println!("x and y of a point is {}, and {}", x, y);
}