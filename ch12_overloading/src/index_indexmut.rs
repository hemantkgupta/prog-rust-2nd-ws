use std::collections::HashMap;
use std::ops::Index;

pub fn index_indexmut_work(){
    // index
    let mut m = HashMap::new();
    m.insert("十", 10);
    m.insert("百", 100);
    m.insert("千", 1000);
    m.insert("万", 1_0000);
    m.insert("億", 1_0000_0000);

    assert_eq!(m["十"], 10);
    assert_eq!(m["千"], 1000);

    assert_eq!(*m.index("十"), 10);
    assert_eq!(*m.index("千"), 1000);

    // index_mut
    let mut desserts =
    vec!["Howalon".to_string(), "Soan papdi".to_string()];
    desserts[0].push_str(" (fictional)");
    desserts[1].push_str(" (real)");

    use std::ops::IndexMut;
    (*desserts.index_mut(0)).push_str(" (fictional)");
    (*desserts.index_mut(1)).push_str(" (real)");
}