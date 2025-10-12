use owo_colors::OwoColorize;
pub fn colorize_brackets(input: &str) -> String {
    let colors: [&dyn Fn(&str) -> String; 5] = [
        &|s| s.red().to_string(),
        &|s| s.green().to_string(),
        &|s| s.yellow().to_string(),
        &|s| s.blue().to_string(),
        &|s| s.magenta().to_string(),
    ];

    let mut chars = input.chars().peekable();
    let mut out = String::with_capacity(input.len());
    let mut depth: usize = 0;

    let mut buf = String::new();
    let flush_buf = |buf: &mut String, out: &mut String, depth: usize, colors: &[&dyn Fn(&str) -> String]| {
        if buf.is_empty() { return; }
        if depth > 0 {
            let color_fn = colors[depth % colors.len()];
            out.push_str(&color_fn(&buf));
        } else {
            out.push_str(&buf);
        }
        buf.clear();
    };

    while let Some(c) = chars.next() {
        if c == '\x1b' && chars.peek() == Some(&'[') {
            flush_buf(&mut buf, &mut out, depth, &colors);

            let mut esc = String::new();
            esc.push('\x1b');
            if let Some(br) = chars.next() { esc.push(br); }
            while let Some(&nc) = chars.peek() {
                let nc = nc;
                chars.next();
                esc.push(nc);
                if nc == 'm' { break; }
            }
            out.push_str(&esc);
            continue;
        }

        match c {
            '(' => {
                flush_buf(&mut buf, &mut out, depth, &colors);
                let color_fn = colors[depth % colors.len()];
                out.push_str(&color_fn("("));
                depth = depth.saturating_add(1);
            }
            ')' => {
                flush_buf(&mut buf, &mut out, depth, &colors);
                if depth > 0 {
                    depth -= 1;
                    let color_fn = colors[depth % colors.len()];
                    out.push_str(&color_fn(")"));
                } else {
                    out.push(')');
                }
            }
            other => {
                buf.push(other);
            }
        }
    }

    (flush_buf)(&mut buf, &mut out, depth, &colors);

    out
}
