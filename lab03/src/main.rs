fn is_prime(n: u16) -> bool {
    if n < 2 {
        return false;
    }
    let mut d: u32 = 2;
    while d * d <= n as u32 {
        if (n as u32) % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn next_prime(x: u16) -> Option<u16> {
    let mut current = x;
    loop {
        if current == u16::MAX {
            return None;
        }
        current += 1;
        if is_prime(current) {
            return Some(current);
        }
    }
}

fn checked_add(x: u32, y: u32) -> u32 {
    let sum = x as u64 + y as u64;
    if sum > u32::MAX as u64 {
        panic!("Overflow addition(panic)");
    } else {
        sum as u32
    }
}

fn checked_mul(x: u32, y: u32) -> u32 {
    let product = x as u64 * y as u64;
    if product > u32::MAX as u64 {
        panic!("Overflow multiplication (panic)");
    } else {
        product as u32
    }
}

fn res_checked_add(x: u32, y: u32) -> Result<u32, String> {
    let sum = x as u64 + y as u64;
    if sum > u32::MAX as u64 {
        Err("Overflow addition".to_string())
    } else {
        Ok(sum as u32)
    }
}

fn res_checked_mul(x: u32, y: u32) -> Result<u32, String> {
    let product = x as u64 * y as u64;
    if product > u32::MAX as u64 {
        Err("Overflow multiplication".to_string())
    } else {
        Ok(product as u32)
    }
}

enum CharError {
    NotAscii,
    NotDigit,
    NotBase16,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, CharError> {
    if !c.is_alphabetic() {
        Err(CharError::NotLetter)
    } else {
        Ok(c.to_ascii_uppercase())
    }
}

fn to_lowercase(c: char) -> Result<char, CharError> {
    if !c.is_alphabetic() {
        Err(CharError::NotLetter)
    } else {
        Ok(c.to_ascii_lowercase())
    }
}

fn print_char(c: char) -> Result<(), CharError> {
    if c.is_control() {
        Err(CharError::NotPrintable)
    } else {
        print!("{c}");
        Ok(())
    }
}

fn char_to_number(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if !c.is_ascii_digit() {
        return Err(CharError::NotDigit);
    }
    Ok(c as u32 - '0' as u32)
}

fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if !c.is_ascii_hexdigit() {
        return Err(CharError::NotBase16);
    }
    match c {
        '0'..='9' => Ok(c as u32 - '0' as u32),
        'a'..='f' => Ok(c as u32 - 'a' as u32 + 10),
        'A'..='F' => Ok(c as u32 - 'A' as u32 + 10),
        _ => Err(CharError::NotBase16),
    }
}
fn print_error(err: CharError) {
    match err {
        CharError::NotAscii => println!("Error: Character is not ASCII!"),
        CharError::NotDigit => println!("Error: Character is not a decimal digit!"),
        CharError::NotBase16 => println!("Error: Character is not a base-16 (hex) digit!"),
        CharError::NotLetter => println!("Error: Character is not a letter!"),
        CharError::NotPrintable => println!("Error: Character is not printable!"),
    }
}

fn main() {
    let a = checked_add(5, 10);
    let b = checked_mul(5, 10);
    let c = 0; //checked_add(u32::MAX, 1);
    let d = 0; // checked_mul(u32::MAX, 2);
    let e = res_checked_add(u32::MAX, 1);
    let f: Result<u32, String> = res_checked_mul(u32::MAX, 2);
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
    println!("e: {:?}, f: {:?}", e, f);
    let mut current = 0;
    while let Some(next) = next_prime(current) {
        println!("{}", next);
        current = next;
    }
}
