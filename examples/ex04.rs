fn main() {
    let _ = env_logger::try_init();

    let db = sled::open("sled-ex04.db").expect("failed to open");

    db.insert("abc", "defghi").expect("failed to insert 1");
    db.insert([1, 2, 3], vec![4, 5, 6])
        .expect("failed to insert 2");

    let value01 = db.get("abc").unwrap().unwrap();
    let value02 = db.get([1, 2, 3]);

    println!("value01 -> {:?}", String::from_utf8_lossy(&value01));
    println!("value02 -> {:?}\n", value02.unwrap());

    let mut x = db.iter();
    //println!("{:?}", x.next());

    while let Some(element) = x.next() {
        println!("{:?}", element);
    }

    println!("\nsize_on_disk = {:?}", db.size_on_disk());
}
