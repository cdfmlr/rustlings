// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}

// https://doc.rust-lang.org/std/keyword.ref.html
//
// & vs ref
// 
// - & denotes that your pattern expects a reference to an object. 
//   Hence & is a part of said pattern: &Foo matches different objects than Foo does.
// - ref indicates that you want a reference to an unpacked value.
//   It is not matched against: Foo(ref foo) matches the same objects as Foo(foo).

