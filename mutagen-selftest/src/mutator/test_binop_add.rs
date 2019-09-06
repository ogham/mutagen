mod test_sum_i32 {

    use ::mutagen::mutate;
    use ::mutagen::MutagenRuntimeConfig;

    // simple function that sums two values
    #[mutate(conf = local(expected_mutations = 1), mutators = only(binop_add))]
    fn sum_i32() -> i32 {
        5 + 1
    }
    #[test]
    fn sum_u32_inactive() {
        MutagenRuntimeConfig::test_without_mutation(|| {
            assert_eq!(sum_i32(), 6);
        })
    }
    #[test]
    fn sum_u32_active1() {
        MutagenRuntimeConfig::test_with_mutation_id(1, || {
            assert_eq!(sum_i32(), 4);
        })
    }
}

mod test_sum_u32 {

    use ::mutagen::mutate;
    use ::mutagen::MutagenRuntimeConfig;

    // simple function that sums 2 u32 values. Unfortunately, the tag `u32` is necessary
    #[mutate(conf = local(expected_mutations = 1), mutators = only(binop_add))]
    fn sum_u32() -> u32 {
        5 + 1
    }
    #[test]
    fn sum_u32_inactive() {
        MutagenRuntimeConfig::test_without_mutation(|| {
            assert_eq!(sum_u32(), 6);
        })
    }
    #[test]
    fn sum_u32_active1() {
        MutagenRuntimeConfig::test_with_mutation_id(1, || {
            assert_eq!(sum_u32(), 4);
        })
    }
}

mod test_str_add {

    use ::mutagen::mutate;
    use ::mutagen::MutagenRuntimeConfig;

    // strings cannot be subtracted, the mutation that changes `+` into `-` should panic
    #[mutate(conf = local(expected_mutations = 1), mutators = only(binop_add))]
    fn str_add() -> String {
        "a".to_string() + "b"
    }
    #[test]
    fn str_add_inactive() {
        MutagenRuntimeConfig::test_without_mutation(|| {
            assert_eq!(&str_add(), "ab");
        })
    }
    #[test]
    #[should_panic]
    fn str_add_active1() {
        MutagenRuntimeConfig::test_with_mutation_id(1, || {
            str_add();
        })
    }
}

mod test_multiple_adds {

    use ::mutagen::mutate;
    use ::mutagen::MutagenRuntimeConfig;

    // sum of multiple values without brackets
    #[mutate(conf = local(expected_mutations = 2), mutators = only(binop_add))]
    pub fn multiple_adds() -> usize {
        5 + 4 + 1
    }

    #[test]
    fn multiple_adds_inactive() {
        MutagenRuntimeConfig::test_without_mutation(|| {
            assert_eq!(multiple_adds(), 10);
        })
    }
    #[test]
    fn multiple_adds_active1() {
        MutagenRuntimeConfig::test_with_mutation_id(1, || {
            assert_eq!(multiple_adds(), 2);
        })
    }
    #[test]
    fn multiple_adds_active2() {
        MutagenRuntimeConfig::test_with_mutation_id(2, || {
            assert_eq!(multiple_adds(), 8);
        })
    }
}

mod test_add_f64 {

    use ::mutagen::mutate;
    use ::mutagen::MutagenRuntimeConfig;

    // sum of multiple values without brackets
    #[mutate(conf = local(expected_mutations = 1), mutators = only(binop_add))]
    pub fn add_f64() -> f64 {
        1.0 + 2.0
    }

    #[test]
    fn add_f64_inactive() {
        MutagenRuntimeConfig::test_without_mutation(|| {
            assert_eq!(add_f64(), 3.0);
        })
    }
    #[test]
    fn add_f64_active1() {
        MutagenRuntimeConfig::test_with_mutation_id(1, || {
            assert_eq!(add_f64(), -1.0);
        })
    }
}
