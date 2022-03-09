use extendr_api::prelude::*;

struct Source {
    path: String,
}

#[extendr]
impl Source {
    fn new(path: String) -> Self {
        Source { path }
    }

    fn path(&self) -> String {
        self.path.clone()
    }

    fn stuffs(&self) -> Strings {
        Strings::from_values(vec![String::from("THING")])
    }
}

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    impl Source;
    fn hello_world;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_source() {
        let source = Source::new(String::from("source path"));
        assert_eq!(source.path(), String::from("source path"));
        assert_eq!(source.stuffs().elt(1), "THING");
    }
}
