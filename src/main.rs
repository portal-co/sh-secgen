fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    args.next();
    let ty = args.next().unwrap();
    let out = args.next().unwrap();
    if let Ok(s) = std::fs::read(format!("{out}.sk")){
        return Ok(());
    }
    match ty.as_str() {
        "signing/ed25519" => {
            let s = ed25519_dalek::SigningKey::generate(&mut rand::thread_rng());
            std::fs::write(format!("{out}.sk"), s.as_bytes())?;
            std::fs::write(format!("{out}.vk"), s.verifying_key().as_bytes())?;
        }
        _ => {
            eprintln!("invalid key type; valid ones are signing/ed25519")
        }
    };
    Ok(())
}
