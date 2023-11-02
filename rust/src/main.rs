use rust::line::Line;

fn main() {
    let l: Line = Line::from_str("abc123");
    let l2: Line = Line::from_str("123xyz");
    println!("{}", l2.get_addr());
    l2.set_prev(l.get_addr());
    l.set_next(l2.get_addr());
    println!("{:?}\n{:?}", l, l2);
    println!("{}, {}", l.to_string(), l2.to_string());
}
