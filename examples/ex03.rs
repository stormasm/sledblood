fn main() {
    let db = sled::open("sled-ex03.db").expect("failed to open");

    db.insert("rick", "storm").expect("failed to insert");
    db.insert(&[1, 2, 3], vec![0]).expect("failed to insert 2");
    db.insert(&[4, 5, 6], vec![1]).expect("failed to insert 3");

    let value01 = db.get("rick").unwrap().unwrap();
    let value02 = db.get(&[1, 2, 3]);
    let value03 = db.get(&[4, 5, 6]);
    let value04 = db.get(&[7, 8, 9]);

    println!("value01 -> {:?}", String::from_utf8_lossy(&value01));

    /*
    println!("value02 -> {:?}", String::from_utf8_lossy(&value02));
    println!("value03 -> {:?}", String::from_utf8_lossy(&value03));
    println!("value04 -> {:?}", String::from_utf8_lossy(&value04));
    */

    println!("value02 -> {:?}", value02.unwrap());
    println!("value03 -> {:?}", value03.unwrap());
    println!("value04 -> {:?}", value04.unwrap());

    //assert_eq!(db.insert(&["a", "b", "c"], vec![0]).unwrap(), None);
    assert_eq!(
        db.insert(&[1, 2, 3], vec![1]).unwrap(),
        Some(sled::InlineArray::from(&[0]))
    );
}
