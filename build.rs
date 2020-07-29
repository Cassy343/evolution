use std::env;
use std::fs;
use std::path::Path;

const HOMOGENOUS_TUPLE_FILE_NAME: &str = "homogenous_tuple.rs";

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("Could not find OUT_DIR");

    let homogenous_tuple_path = Path::new(&out_dir).join(HOMOGENOUS_TUPLE_FILE_NAME);
    let mut homogenous_tuple_src = String::new();

    for i in 1..=12 {
        homogenous_tuple_src.push_str(&format!(
            r#"impl<T> IndexedCollection for ({}) {{
    type Item = T;

    #[inline(always)]
    fn size(&self) -> usize {{
        {}
    }}

    #[inline]
    fn get(&self, index: usize) -> Option<&Self::Item> {{
        match index {{
{}
        }}
    }}

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item> {{
        match index {{
{}
        }}
    }}
}}

"#,
            repeat("T,".to_owned(), i),
            i,
            homotup_match_arms(i, false),
            homotup_match_arms(i, true)
        ));
    }

    fs::write(&homogenous_tuple_path, &homogenous_tuple_src)
        .expect("Failed to write homogenous tuple helper file.");

    println!("cargo:rerun-if-changed=build.rs");
}

fn homotup_match_arms(size: usize, mutable: bool) -> String {
    let mut arms = String::new();
    let mutable = if mutable { "mut" } else { "" };
    for i in 0..size {
        arms.push_str(&format!("\t\t\t{0} => Some(&{1} self.{0}),\n", i, mutable));
    }
    arms.push_str("\t\t\t_ => None");
    arms
}

fn repeat(mut string: String, times: usize) -> String {
    let clone = string.clone();
    string.reserve(string.len() * times);
    for _ in 0..times - 1 {
        string.push_str(&clone);
    }
    string
}
