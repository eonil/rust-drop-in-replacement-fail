
pub mod foo1 {
    pub mod foo2 {
        pub struct Bar3 {
        }
        pub enum Bar4 {
            Bar5,
            Bar6(Vec<Bar3>),
            Bar7{  bar8: Option<Bar3>, },
        }
    }
}


