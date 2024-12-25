fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    args.next();
    let ty = args.next().unwrap();
    let out = args.next().unwrap();
    match ty.as_str() {
        "signing/ed25519" => {
            if let Ok(s) = std::fs::read(format!("{out}.sk")) {
                return Ok(());
            }
            let s = ed25519_dalek::SigningKey::generate(&mut rand::thread_rng());
            std::fs::write(format!("{out}.sk"), s.as_bytes())?;
            std::fs::write(format!("{out}.vk"), s.verifying_key().as_bytes())?;
        }
        "git/init" => {
            let mut r = std::fs::read(format!("{out}/.gitignore")).unwrap_or_else(|_| vec![]);
            static X: &[u8] = b"\n*.sk\n!test.sk\n";
            if !r.windows(X.len()).any(|w| w == X) {
                r.extend_from_slice(X);
            }
            std::fs::write(format!("{out}/.gitignore"), r)?;
        }
        _ => {
            eprintln!("invalid key type; valid ones are signing/ed25519. you can also initialize a repo with git/init")
        }
    };
    Ok(())
}
