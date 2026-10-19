struct Note {
    title: String,
    body: String,
}

// ---------- Part 1: borrow to read (&str) ----------

// How many times does `c` appear in `s`?
fn count_char(s: &str, c: char) -> usize {
    let mut count: usize;
    count = 0;
    for ch in s.chars() {
        if ch == c {
            count += 1;
        }
    }
    return count;
}

// The first word of `s` (up to the first space, or all of `s`), as a slice of `s`.
// No copying: return a &str that points into `s`. Use as_bytes and a while loop.
fn first_word(s: &str) -> &str {
    let mut i: usize;
    let mut i = 0;
    let bytes = s.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b' ' {
            return &s[..i];
        } else {
            i += 1;
        }
    }
    s
}

// A new String with the characters of `s` in reverse order.

fn reverse(s: &str) -> String {
    let mut txt = String::new();
    for ch in s.chars() {
        txt.insert(0, ch);
    }
    txt
}

// True if `s` reads the same backwards. Reuse reverse().
fn is_palindrome(s: &str) -> bool {
    if s == reverse(&s) {
        return true;
    } else {
        return false;
    }
}

// A new String made of `s` repeated n times.
fn repeat_str(s: &str, n: usize) -> String {
    let mut i = n;
    let mut rpt = String::new();
    while i > 0 {
        rpt.push_str(s);
        i -= 1;
    }
    rpt
}

// ---------- Part 2: mutable borrow (&mut String) ----------

// Append "[tag]" to the caller's string.
fn add_tag(s: &mut String, tag: &str) {
    s.push_str("[");
    s.push_str(tag);
    s.push_str("]");
}

// Replace every occurrence of `c` with '*' in the caller's string.
// Build a new String, then assign it back with *s = ...
fn censor(s: &mut String, c: char) {
    let mut std = String::new();
    for ch in s.chars() {
        if ch == c {
            std.push('*');
        } else {
            std.push(ch);
        }
    }
    *s = std;
}

// ---------- Part 3: ownership in, ownership out (String) ----------

// Take a string, make it uppercase, and give it back.
fn shout(s: String) -> String {
    let mut up = String::new();
    for ch in s.chars() {
        up.push(ch.to_ascii_uppercase());
    }
    return up;
}

// Take two strings and return the longer one (the first on a tie). The other is dropped.
fn longer_of(a: String, b: String) -> String {
    if a.len() >= b.len() {
        return a;
    } else {
        return b;
    }
}

// ---------- Part 4: Notebook (a struct that owns its strings) ----------

// Build a Note from two string slices. The Note must own its text.
fn make_note(title: &str, body: &str) -> Note {
    Note {
        title: String::from(title),
        body: String::from(body),
    }
}

// Number of words in the body, counted by hand with chars().
fn word_count(note: &Note) -> usize {
    let mut count: usize;
    count = 0;
    let mut in_word = false;
    for ch in note.body.chars() {
        if ch == ' ' {
            if in_word {
                count += 1;
                in_word = false;
            }
        } else {
            in_word = true;
        }
    }
    if in_word {
        count += 1;
    }
    return count;
}

// Add `text` to the end of the body, separated by one space
// (no leading space if the body is empty).
fn append(note: &mut Note, text: &str) {
    if note.body.len() > 0 {
        note.body.push(' ');
    }

    note.body.push_str(text);
}

// Use the note up and return "[ARCHIVED] <title>: <body>".
fn archive(note: Note) -> String {
    let mut arch = String::new();
    arch.push_str("[ARCHIVED] ");
    arch.push_str(&note.title);
    arch.push_str(": ");
    arch.push_str(&note.body);
    return arch;
}

// Combine two notes. The first is consumed and reused, the second is only read.
// Title becomes "<a>+<b>", body becomes "<a body> <b body>".
/*fn merge(a: Note, b: &Note) -> Note
{

}
*/

fn main() {
    // Part 1
    let text = String::from("level up your rust");
    println!("{}", count_char(&text, 'e'));
    println!("{}", count_char("banana", 'a'));
    println!("{}", first_word(&text));
    println!("{}", first_word("rust"));
    println!("{}", reverse(&text));
    println!("{} {}", is_palindrome("level"), is_palindrome(&text));
    println!("{}", repeat_str("ab", 3));

    // Part 2
    let mut s = String::from("hello world");
    add_tag(&mut s, "v2");
    println!("{s}");
    censor(&mut s, 'o');
    println!("{s}");

    // Part 3
    let loud = shout(s);
    println!("{loud}");
    let winner = longer_of(String::from("tiny"), loud);
    println!("{winner}");

    // Part 4
    let title = String::from("shopping");
    let mut n1 = make_note(&title, "milk");
    let n2 = make_note("ideas", "learn rust borrowing");
    println!("{title}");
    println!("{} word(s)", word_count(&n1));
    append(&mut n1, "eggs");
    append(&mut n1, "bread");
    println!("{}: {}", n1.title, n1.body);
    /* let merged = merge(n1, &n2);
    println!("{}: {}", merged.title, merged.body);
    println!("{} word(s)", word_count(&merged));
    println!("{}", archive(n2)); */
}
