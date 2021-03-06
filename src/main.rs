use std::fmt;
use std::ops::Add;
use std::ops::Shl;

use ethereum_types::U512;

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        unimplemented!()
    }
}

fn set_compact(compact: u32) -> U512 {
    let size = compact >> 24;
    let mut word = compact & 0x007fffff;
    if size <= 3 {
        word >>= 8 * (3 - size);
        U512::from(word)
    } else {
        U512::from(word).shl(8 * (size - 3))
    }
}

// Implementing the Add trait to overload the + operator for Point instances
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Implementing the Add trait on Millimeters to add Millimeters to Meters
struct Millimeters(u32);

struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Creating a Wrapper type around Vec<String> to implement Display
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

macro_rules! times_five {
    ($e:expr) => {5 * $e};
}

macro_rules! to_string_vec {
    (
        // Start a repetition:
        $(
            // Each repeat must contain an expression...
            $element:expr
        )
        // ...separated by commas...
        ,
        // ...zero or more times.
        *
    ) => {
        // Enclose the expansion in a block so that we can use
        // multiple statements.
        {
            let mut v = Vec::new();

            // Start a repetition:
            $(
                // Each repeat will contain the following statement, with
                // $element replaced with the corresponding expression.
                v.push(format!("{:?}", $element));
            )*

            v
        }
    };
}

fn main() {
    let strs = to_string_vec![1, 2, 3];
    for str in strs.iter() {
        println!("{}", str)
    }
    println!{"2 x 5 = {}", times_five!{1+1}}
    let work = set_compact(0x18abcdef);
    println!("{}", work);
    println!("p = {}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!{"w = {}", w}
}
