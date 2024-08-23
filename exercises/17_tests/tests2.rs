// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Test the function `power_of_2` with some values.
        assert_eq!(power_of_2(2), 4, "The power of 2 to 2 should be 4");
        assert_eq!(power_of_2(3), 8, "The power of 3 to 2 should be 9");
        assert_eq!(power_of_2(4), 16, "The power of 4 to 2 should be 16");
        assert_eq!(power_of_2(5), 32, "The power of 5 to 2 should be 25");
    }
}
