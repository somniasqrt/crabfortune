use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_DIRS: &[&str] = &[
    "/opt/homebrew/share/games/fortunes",
    "/usr/local/share/games/fortunes",
    "/usr/share/games/fortunes",
    "/usr/share/fortune",
];

struct Source {
    name: String,
    fortunes: Vec<String>,
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

fn main() {
    let mut sources = Vec::new();
    if let Some(dir) = DEFAULT_DIRS.iter().map(Path::new).find(|d| d.is_dir()) {
        load(dir, &mut sources);
    }
    if sources.is_empty() {
        eprintln!("no fortunes found");
        std::process::exit(1);
    }
    let mut rng = Rng::new();
    let total: usize = sources.iter().map(|s| s.fortunes.len()).sum();
    let mut i = rng.below(total);
    for s in &sources {
        if i < s.fortunes.len() {
            print!("{}", s.fortunes[i]);
            return;
        }
        i -= s.fortunes.len();
    }
}
