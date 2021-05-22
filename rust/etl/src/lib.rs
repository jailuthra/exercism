use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut out = BTreeMap::<char,i32>::new();
    for (points, letters) in h.iter() {
        for letter in letters {
            out.insert(letter.to_lowercase().to_string().chars().next().unwrap(), *points);
        }
    }
    out
}
