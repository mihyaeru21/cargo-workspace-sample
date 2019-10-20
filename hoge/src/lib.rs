pub struct Hoge {
    pub a: i64,
    pub b: String,
}

impl Hoge {
    pub fn new() -> Self {
        Hoge {
            a: 1,
            b: "faaa".into(),
        }
    }
}
