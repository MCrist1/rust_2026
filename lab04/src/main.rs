use std::fs;

fn rot13(text: &str) -> Result<String, &'static str> {
    let mut result = String::new();
    for c in text.chars() {
        if !c.is_ascii() {
            return Err("Eroare : Textul contine caractere non-ASCII");
        }

        let rotated = match c {
            'a'..='m' | 'A'..='M' => (c as u8 + 13) as char,
            'n'..='z' | 'N'..='Z' => (c as u8 - 13) as char,
            _ => c,
        };
        result.push(rotated);
    }
    Ok(result)
}

fn prelungit(word: &str) -> &str {
    match word {
        "pt" | "ptr" => "pentru",
        "dl" => "domnul",
        "dna" => "doamna",
        _ => word,
    }
}

fn p3(txt: &str) {
    let t = match fs::read_to_string(txt) {
        Ok(text) => text,
        Err(_e) => {
            println!("Eroare la citirea fisierului: {}", txt);
            return;
        }
    };
    let mut result_text = String::new();
    for word in t.split_whitespace() {
        if !result_text.is_empty() {
            result_text.push(' ');
        }
        result_text.push_str(prelungit(word));
    }
    println!("Test Expandat: {}", result_text);
}

fn format_hosts(path: &str) {
    let text = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            println!("Eroare la citirea fisierului {}: {}", path, e);
            return;
        }
    };

    for line in text.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        let mut ip = "";
        let mut hostname = "";

        for part in trimmed_line.split_whitespace() {
            if ip.is_empty() {
                ip = part;
            } else if hostname.is_empty() {
                hostname = part;
            }
        }
        println!("\nIP: {}, Hostname: {}", ip, hostname);
    }
}

fn main() {
    let n = fs::read_to_string("file1.txt").unwrap();
    println!("{}", n);

    let mut max_bytes = "";
    let mut max_chars = "";

    for line in n.lines() {
        if line.len() > max_bytes.len() {
            max_bytes = line;
        }
        if line.chars().count() > max_chars.chars().count() {
            max_chars = line;
        }
    }
    println!("Max line (by bytes): {}", max_bytes);
    println!("Max line (by characters): {}", max_chars);

    let success = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    match rot13(success) {
        Ok(rotated) => println!("Rot13: {}", rotated),
        Err(e) => println!("Error: {}", e),
    }

    let fail = "Hello, ©®!";
    match rot13(fail) {
        Ok(rotated) => println!("Rot13: {}", rotated),
        Err(e) => println!("Error: {}", e),
    }

    p3("fraza.txt");
    format_hosts("hosts.txt");
}
