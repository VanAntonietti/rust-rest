fn main() {
    hello_world("Tuturu");
}

fn hello_world(x: &str) -> &str {
    println!("{}", x);
    return x;
}

#[cfg(test)]
mod tests;