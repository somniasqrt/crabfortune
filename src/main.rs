//! crabfortune - fortune(6), rewritten in Rust 🦀
//!
//! Usage -> crabfortune [-s] [-l] [-n N] [-c] [-e] [--crab] [file|dir ...]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

const CRAB_FORTUNES: &str = include_str!("crab.txt");

const DEFAULT_DIRS: &[&str] = &[
    "/opt/homebrew/share/games/fortunes",
    "/usr/local/share/games/fortunes",
    "/usr/share/games/fortunes",
    "/usr/share/fortune",
];

struct Opts {
    short: bool,
    long: bool,
    len: usize,
    show_source: bool,
    equal: bool,
    crab_only: bool,
    paths: Vec<PathBuf>,
}

struct Source {
    name: String,
    fortunes: Vec<String>,
}

fn usage() -> ! {
    eprintln!("usage: crabfortune [-s] [-l] [-n N] [-c] [-e] [--crab] [file|dir ...]");
    std::process::exit(2);
}

fn parse_args() -> Opts {
    let mut o = Opts {
        short: false,
        long: false,
        len: 160,
        show_source: false,
        equal: false,
        crab_only: false,
        paths: Vec::new(),
    };
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "-s" => o.short = true,
            "-l" => o.long = true,
            "-n" => o.len = args.next().and_then(|n| n.parse().ok()).unwrap_or_else(|| usage()),
            "-c" => o.show_source = true,
            "-e" => o.equal = true,
            "--crab" => o.crab_only = true,
            "-h" | "--help" => usage(),
            _ if a.starts_with('-') => usage(),
            _ => o.paths.push(PathBuf::from(a)),
        }
    }
    if o.short && o.long {
        usage();
    }
    o
}

/// splitmix64, seeded from the clock and the pid. Good enough for wisdom.
struct Rng(u64);

impl Rng {
    fn new() -> Self {
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        Self(t ^ ((std::process::id() as u64) << 32))
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn parse(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for line in text.lines() {
        if line == "%" {
            if !cur.trim().is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            cur.clear();
        } else {
            cur.push_str(line);
            cur.push('\n');
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur);
    }
    out
}

fn is_fortune_file(p: &Path) -> bool {
    let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
    p.is_file() && !matches!(ext, "dat" | "u8" | "pos" | "sh")
}

fn load(path: &Path, into: &mut Vec<Source>) {
    if path.is_dir() {
        let Ok(rd) = fs::read_dir(path) else { return };
        let mut files: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
        files.sort();
        for f in files.iter().filter(|f| is_fortune_file(f)) {
            load(f, into);
        }
    } else if let Ok(bytes) = fs::read(path) {
        let fortunes = parse(&String::from_utf8_lossy(&bytes));
        if !fortunes.is_empty() {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            into.push(Source { name, fortunes });
        }
    }
}

fn main() -> ExitCode {
    let o = parse_args();

    let mut sources = vec![Source {
        name: "crab".into(),
        fortunes: parse(CRAB_FORTUNES),
    }];
    if !o.crab_only {
        if o.paths.is_empty() {
            if let Some(dir) = DEFAULT_DIRS.iter().map(Path::new).find(|d| d.is_dir()) {
                load(dir, &mut sources);
            }
        } else {
            sources.clear();
            for p in &o.paths {
                load(p, &mut sources);
            }
        }
    }

    // length filter
    for s in &mut sources {
        s.fortunes.retain(|f| {
            let n = f.chars().count();
            (!o.short || n <= o.len) && (!o.long || n > o.len)
        });
    }
    sources.retain(|s| !s.fortunes.is_empty());
    if sources.is_empty() {
        eprintln!("crabfortune: no fortunes found. the crab has nothing to say.");
        return ExitCode::FAILURE;
    }

    let mut rng = Rng::new();
    let (src, fortune) = if o.equal {
        // every file equally likely, like `fortune -e`
        let s = &sources[rng.below(sources.len())];
        (s, &s.fortunes[rng.below(s.fortunes.len())])
    } else {
        // every fortune equally likely
        let total: usize = sources.iter().map(|s| s.fortunes.len()).sum();
        let mut i = rng.below(total);
        let s = sources
            .iter()
            .find(|s| {
                if i < s.fortunes.len() {
                    true
                } else {
                    i -= s.fortunes.len();
                    false
                }
            })
            .expect("index is within total");
        (s, &s.fortunes[i])
    };

    if o.show_source {
        println!("({})\n%", src.name);
    }
    print!("{fortune}");
    ExitCode::SUCCESS
}
