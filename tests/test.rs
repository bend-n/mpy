#[test]
fn main() {
    dbg!(mpy::eval! { "print([n for n in range(2)])" });
}
