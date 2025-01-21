use comp_macro::comp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res: Vec<_> = comp![x*2 for x in [1,2,3]].collect();
        assert_eq!(res, [2,4,6])
    }
}
