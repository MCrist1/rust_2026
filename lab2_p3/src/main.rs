fn add_space(s: &mut String, n: usize) {
    for _ in 0..n {
        s.push(' ');
    }
}

fn add_str(s: &mut String, text: &str) {
    s.push_str(text);
}

fn add_integer(s: &mut String, n: i32) {
    if n == 0 {
        s.push('0');
        return;
    }
    let mut n = n;
    let mut count = 0;
    let mut buffer: Vec<char> = Vec::new();
    while n > 0 {
        let digit = n % 10;
        buffer.push((digit as u8 + b'0') as char);
        count += 1;
        n /= 10;
        if count % 3 == 0 && n > 0 {
            buffer.push('_');
        }
    }
    buffer.reverse();
    for c in buffer {
        s.push(c);
    }
}

fn add_float(s: &mut String, f: f32) {
    let int_part = f as i32;
    add_integer(s, int_part);
    s.push('.');
    let mut frac = f - int_part as f32;
    for _ in 0..3 {
        frac *= 10.0;
        let digit = frac as u8;
        s.push((digit + b'0') as char);
        frac -= digit as f32;
    }
}

fn main() {
    let mut s = String::new();
    add_space(&mut s, 40);
    add_str(&mut s, "I");
    add_space(&mut s, 1);
    add_str(&mut s, "💚\n");
    add_space(&mut s, 40);
    add_str(&mut s, "RUST.\n\n");
    add_space(&mut s, 4);
    add_str(&mut s, "Most");
    add_space(&mut s, 12);
    add_str(&mut s, "crate");
    add_space(&mut s, 6);
    add_integer(&mut s, 306437968);
    add_space(&mut s, 11);
    add_str(&mut s, "and");
    add_space(&mut s, 5);
    add_str(&mut s, "lastest");
    add_space(&mut s, 9);
    add_str(&mut s, "is\n");
    add_space(&mut s, 9);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 8);
    add_str(&mut s, "has");
    add_space(&mut s, 13);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 9);
    add_str(&mut s, "version");
    add_space(&mut s, 4);
    add_float(&mut s, 2.038);
    add_str(&mut s, ".\n");

    print!("{}", s);
}
