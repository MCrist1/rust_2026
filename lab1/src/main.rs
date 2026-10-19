fn main() {
    let mut n: i32 = 2;
    while n < 101 {
        let mut ok = true;
        for i in 2..n {
            if n % i == 0 {
                ok = false;
                break;
            }
        }
        if ok {
            println!("{}", n);
        }
        n += 1;
    }
    if coprime(15, 28) {
        println!("Sunt coprime");
    } else {
        println!("Nu sunt coprime");
    }
}

fn coprime(a: i32, b: i32) -> bool {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a == 1
}
