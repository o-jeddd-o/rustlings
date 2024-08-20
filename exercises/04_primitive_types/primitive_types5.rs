fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    let (name[cat.0], age(cat.1)) = cat;

    println!("{name} is {age} years old");
}
