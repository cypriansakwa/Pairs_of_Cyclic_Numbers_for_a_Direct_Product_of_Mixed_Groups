fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn is_valid_m(m: u32) -> bool {
    if m == 2 || m == 4 {
        return true;
    }

    if m % 2 == 0 {
        let half = m / 2;
        if half > 1 && is_prime(half) {
            return true;
        }
    }

    if is_prime(m) {
        return true;
    }

    let mut p = 3;
    while p * p <= m {
        if is_prime(p) {
            let mut power = p;
            while power <= m {
                if power == m {
                    return true;
                }
                power *= p;
            }
        }
        p += 2;
    }

    false
}

fn generate_cyclic_pairs(limit: u32) -> Vec<(u32, u32)> {
    let mut pairs = Vec::new();

    for n in 1..=limit {
        for m in 1..=limit {
            if is_valid_m(m) {
                pairs.push((n, m));
            }
        }
    }
    pairs
}

fn main() {
    let limit = 10; // Set the limit for n and m values
    let pairs = generate_cyclic_pairs(limit);

    for (n, m) in pairs {
        println!("({}, {})", n, m);
    }
}







