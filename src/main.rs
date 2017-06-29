fn main() {
    println!("Hello, world!");
}

fn square(mut value_to_square: i32) -> i32 {
    value_to_square *= value_to_square;
    value_to_square
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_is_squared() {
        assert_eq!(4, square(2))
    }
}