//! Print binary numbers, as in the "j" format in the mdb
//! debugger for illumos/Solaris.

// There are a number of arrow characters to choose from:
// △,↑,˄,˰,⌃,˄,⭡,▲, and ^.  We use ▴.
// Similarly, for corners we can use └, ╰, or +.

fn lexnum(num: &str) -> (u32, &str) {
    match num {
        "0" => (10, "0"),
        s if s.starts_with("0x") || s.starts_with("0X") => (16, &s[2..]),
        s if s.starts_with("0t") || s.starts_with("0T") => (10, &s[2..]),
        s if s.starts_with("0b") || s.starts_with("0B") => (2, &s[2..]),
        s if s.starts_with("0") => (8, &s[0..]),
        s => (10, s),
    }
}

const PREFIX: &str = "                ";

fn puts_suffix(s: &[char], suffix: &str) {
    print!("{PREFIX}");
    for &c in s.iter() {
        print!("{c}");
    }
    print!("{suffix}");
}

fn puts(s: &[char]) {
    puts_suffix(s, "");
    println!();
}

fn jfmt(num: u128) {
    let n = 128 - num.leading_zeros() as usize;

    let mut v = Vec::new();
    let mut ones = Vec::new();
    for k in 0..n {
        let bit = (num >> k) & 0b1 == 0b1;
        v.push(bit);
        if bit {
            ones.push(k);
        }
    }
    v.reverse();

    let mut cs = vec![' '; n];

    println!("{PREFIX}{num:b}");
    for k in 0..v.len() {
        cs[k] = if v[k] { '▴' } else { ' ' };
    }
    puts(&cs);

    let max1 = ones.iter().last().map_or(0, |&l| l);
    let bit_width = max1.checked_ilog10().unwrap_or(0) as usize + 1;
    let mask_width = (max1 + 4) / 4;
    for this1 in ones.into_iter() {
        let off = n - 1 - this1;
        for (k, &b) in v.iter().enumerate() {
            cs[k] = match (k, b) {
                (k, true) if k < off => '│',
                (k, _) if k < off => ' ',
                (k, _) if k == off => '╰',
                _ => '─',
            };
        }
        puts_suffix(&cs, "── ");
        println!(
            "bit {this1:bit_width$} mask 0x{mask:0>mask_width$x}",
            mask = 1u128 << this1
        );
    }

    println!();
    println!("{PREFIX}hex: {num:#x}");
    println!("{PREFIX}dec: {num}");
    println!("{PREFIX}oct: {num:#o}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: jfmt <number>");
        std::process::exit(1);
    }
    let (radix, numstr) = lexnum(&args[1]);
    let Ok(num) = u128::from_str_radix(numstr, radix) else {
        eprintln!("jfmt: could not parse number '{arg}'", arg = args[1]);
        std::process::exit(1);
    };
    jfmt(num);
}
