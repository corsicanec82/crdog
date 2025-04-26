pub fn do_nothing() {
    // do nothing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        do_nothing();
    }
}
