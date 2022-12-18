//https://www.codewars.com/kata/54dc6f5a224c26032800005c
// Original
pub fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.is_empty() || list_cat.is_empty() {
        return String::new();
    }
    let mut counts = list_cat
        .iter()
        .map(|cat| (cat, 0_u32))
        .collect::<std::collections::HashMap<_, _>>();
    list_art
        .iter()
        .map(|a| {
            let (first_char, _) = a.split_at(1);
            let num: u32 = a
                .split_whitespace()
                .last()
                .and_then(|n| n.parse().ok())
                .unwrap();
            (first_char, num)
        })
        .for_each(|(c, n)| {
            if let Some(t) = counts.get_mut(&c) {
                *t += n;
            }
        });
    list_cat
        .iter()
        .map(|cat| format!("({} : {})", cat, counts.get(cat).unwrap_or(&0)))
        .collect::<Vec<String>>()
        .join(" - ")
}

// Better
pub fn stock_list2(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.is_empty() || list_cat.is_empty() {
        return String::new();
    }

    let mut counts = std::collections::HashMap::new();
    for art in list_art {
        let num: u32 = art
            .split_whitespace()
            .last()
            .and_then(|n| n.parse().ok())
            .unwrap();
        *counts.entry(&art[0..1]).or_insert(0) += num
    }
    list_cat
        .iter()
        .map(|cat| format!("({} : {})", cat, counts.get(cat).unwrap_or(&0)))
        .collect::<Vec<String>>()
        .join(" - ")
}
