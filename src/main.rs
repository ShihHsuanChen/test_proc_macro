use comp_macro::comp;

fn main() {
    let res: Vec<_> = comp![x*2 for x in [1,2,3]].collect();
    assert_eq!(res, [2,4,6])
}
