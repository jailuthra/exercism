pub fn encode(source: &str) -> String {
    let mut out = String::new();
    if source.len() == 0 { return out; }
    let mut chars = source.chars();
    let mut last = chars.next().unwrap_or('0');
    let mut count = 1;
    for c in chars {
        if c.is_ascii_alphabetic() || c == ' ' {
            if last == c {
                count += 1;
                continue;
            }
            if count > 1 {
                out.push_str(&count.to_string());
            }
            out.push(last);
            last = c;
            count = 1;
        }
    }
    if count > 1 {
        out.push_str(&count.to_string());
    }
    out.push(last);
    return out;
}

enum Mode {
    Character,
    Repetition,
}

pub fn decode(source: &str) -> String {
    let mut out = String::new();
    let mut mode = Mode::Character;
    let mut count = 0;
    for c in source.chars() {
        if c.is_digit(10) {
            mode = Mode::Repetition;
            count = count*10 + c.to_digit(10).unwrap_or(0);
        } else {
            match mode {
                Mode::Character => {out.push(c)},
                Mode::Repetition => {
                    mode = Mode::Character;
                    for _ in 0..count {
                        out.push(c);
                    }
                    count = 0;
                },
            }
        }
    }
    return out;
}
