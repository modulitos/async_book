use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
struct TestNoPin {
    a: String,
    b: *const String,
}

impl TestNoPin {
    fn new(txt: &str) -> Self {
        TestNoPin {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        unsafe { &*(self.b) }
    }
}

#[test]
fn test_no_pin() {
    // Our example works fine if we don't move any of our data around as you can observe by running
    // this example:

    let mut test1 = TestNoPin::new("test1");
    test1.init();
    let mut test2 = TestNoPin::new("test2");
    test2.init();

    // println!("a: {}, b: {}", test1.a(), test1.b());
    assert_eq!(test1.a(), "test1");
    assert_eq!(test1.b(), "test1");

    // what do you expect this to do?
    std::mem::swap(&mut test1, &mut test2);

    test1.a = "I've totally changed now!".to_string();
    // println!("a: {}, b: {}", test2.a(), test2.b());
    assert_eq!(test2.a(), "test1");
    assert_eq!(test2.b(), "I've totally changed now!");
}

#[derive(Debug)]
struct TestStackPin {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl TestStackPin {
    fn new(txt: &str) -> Self {
        Self {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // This makes our type `!Unpin`
        }
    }
    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a(self: Pin<&'_ Self>) -> &'_ str {
        &self.get_ref().a
    }

    fn b(self: Pin<&'_ Self>) -> &'_ String {
        unsafe { &*(self.b) }
    }
}

#[test]
fn test_stack_pin() {
    // test1 is safe to move before we initialize it
    let mut test1 = TestStackPin::new("test1");
    // Notice how we shadow `test1` to prevent it from being accessed again
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    TestStackPin::init(test1.as_mut());

    let mut test2 = TestStackPin::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    TestStackPin::init(test2.as_mut());

    println!(
        "a: {}, b: {}",
        TestStackPin::a(test1.as_ref()),
        TestStackPin::b(test1.as_ref())
    );
    assert_eq!(TestStackPin::a(test1.as_ref()), "test1");
    assert_eq!(TestStackPin::b(test1.as_ref()), "test1");

    // This will no longer compile, because the types are pinned!
    // std::mem::swap(test1.get_mut(), test2.get_mut());

    println!(
        "a: {}, b: {}",
        TestStackPin::a(test2.as_ref()),
        TestStackPin::b(test2.as_ref())
    );
    assert_eq!(TestStackPin::a(test2.as_ref()), "test2");
    assert_eq!(TestStackPin::b(test2.as_ref()), "test2");
}

#[derive(Debug)]
struct TestHeapPin {
    a: String,
    b: *const String,
    c: PhantomPinned
}

impl TestHeapPin {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let test = Self {
            a: String::from(txt),
            b: std::ptr::null(),
            c: PhantomPinned,
        };

        let mut boxed = Box::pin(test);

        // init the self-referential pointer:
        let self_ptr: *const String = &boxed.as_ref().a;
        // using type annotations for clarity, but this also works:
        // let self_ptr = &boxed.as_ref().a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr }

        boxed
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        unsafe { &*(self.b) }
    }
}

#[test]
fn test_heap_pin() {
    let test1 = TestHeapPin::new("test1");
    let test2 = TestHeapPin::new("test2");

    assert_eq!(test1.as_ref().b(), "test1");
    assert_eq!(test2.as_ref().b(), "test2");
}


fn main() {
    // some misc pointer deref tests:
    //
    // let n = 5;
    // let mut x: *const u8 = std::ptr::null();
    // println!("x: {:?}", x);
    // // println!("*x: {:?}", unsafe { *x });
    // x = &n;
    // println!("x: {:?}", x);
    // println!("*x: {:?}", unsafe { *x });
    //

    // test_stack_pin();
}
