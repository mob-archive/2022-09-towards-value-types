extern crate rand;

pub fn random() -> f32 {
    rand::random()
}

#[cfg(test)]
mod tests {

    use crate::random::random;

    #[test]
    fn it_should_return_a_number_between_0_and_1_excluding() {
        let r = random();
        assert!(r >= 0.0);
        assert!(r < 1.0);
    }

    #[test]
    fn it_should_seldom_return_same_numbers() {
        // This might fail since it is randome and this case can (rarely) happen
        assert!(random() != random() || random() != random());
    }
}
