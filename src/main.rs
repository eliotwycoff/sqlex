use extractsql::{parse_sql, simple_parse};

use std::{fs, path::Path};

fn main() {
    let res = simple_parse(Path::new("./sql/small.sql"));
    println!("res: {:#?}", res);
}
