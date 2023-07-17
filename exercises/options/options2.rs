// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        // word = optional_target {
        // assert_eq!(word, target);
        // }
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // pop is not guarenteed determined
        while let Some(integer) = optional_integers.pop() {
            // Stack another Some() because this is a list of optional integers
            if let Some(integer) = integer {
                assert_eq!(integer, range);
                range -= 1;
            }
        }

        assert_eq!(cursor, 0);
    }
}
