fn main() {
    let mut cfg = sled::Config::default();
    cfg.cache_capacity_bytes = 256;
    cfg.path = "sled-names.db".into();
    let db = cfg.open::<64>().unwrap();

    db.insert("rick", "storm").expect("failed to insert");

    let value = db.get("rick").unwrap().unwrap();
    println!("{:?}", String::from_utf8_lossy(&value));
}
