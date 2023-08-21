use push_in_place::PushInPlace;

fn main() {
    let vector = Vec::new()
        .push_in_place(1)
        .push_in_place(2)
        .push_in_place(3);
    println!("{vector:?}");
}
