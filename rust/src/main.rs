use rust::line::Line;

fn main() {
    let l: Line = Line::from_str("abc123");
    let l2: Line = Line::from_str("123xyz");
    println!("{}", l2.get_addr());
    l2.set_prev(l.get_addr());
    l.set_next(l2.get_addr());
    println!("{:?}\n{:?}", l, l2);
    println!("{}, {}", l.to_string(), l2.to_string());
    // l.set_char(0, 'c' as u8);
    // l.pop_char();
    l.push_char('0' as u8);
    println!("{}, {}", l.len(), l.cap());
    l.pop_char();
    l.pop_char();
    l.pop_char();
    l.pop_char();
    println!("{}, {}", l.len(), l.cap());
    println!("{}", l.remove_char(0));
    println!("{}, {}", l.len(), l.cap());
    l.pop_char();
    println!("{}, {}", l.len(), l.cap());
    // l.remove_char(5);
    // l.insert_char(5, '0' as u8);
    println!("{}", l.to_string());
}
