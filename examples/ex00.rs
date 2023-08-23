fn main() {
    let mut cfg = sled::Config::default();
    cfg.cache_capacity_bytes = 256;
    cfg.path = "sled.db".into();
    let db = cfg.open::<64>().unwrap();

    for v in 0..1_024u32 {
        db.insert(v.to_be_bytes(), &[42u8; 4096][..])
            .expect("failed to insert");
        if v % 128 == 0 {
            eprintln!("{}", v as f64)
        }
    }
}
