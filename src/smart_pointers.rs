// To determine how much space to allocate for a Message value,
// Rust goes through each of the variants to see which variant needs the most space.
enum _Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// "MyBox smart pointer with Deref trait on it"
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// "Cons list" example using Box<T>
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Custom smart pointer with Drop trait on it
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Droping Custom smart pointer with data {}", self.data);
    }
}

pub fn test() {
    // The deallocation happens for the box (stored on the stack)
    // and the data it points to (stored on the heap).
    let b = Box::new(5);
    println!("b = {}", b);

    let _list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

    let x = 5;
    let y = &x;
    // Dereference the pointer
    println!("{}, {}", x, *y);

    let a = 15;
    let b = Box::new(a);
    println!("{}, {}", a, *b);

    let c = MyBox::new(a);
    println!("{}, {}", a, *c);


    // CustomSmartPointer example
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");


    // ListRC example
    use std::rc::Rc;

    enum ListRC {
        Cons(i32, Rc<ListRC>),
        Nil,
    }

    let a = Rc::new(ListRC::Cons(5, Rc::new(ListRC::Cons(10, Rc::new(ListRC::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = ListRC::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        // The implementation of the Drop trait decreases
        // the reference count automatically when an Rc<T> value goes out of scope
        let _c = ListRC::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));


    // Interior mutability example (RefCell<T>)
}