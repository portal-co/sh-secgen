fn main() -> std::io::Result<()>{
    let mut args = std::env::args();
    args.next();
    let out = args.next().unwrap();
    let s = ed25519_dalek::SigningKey::generate(&mut rand::thread_rng());
    std::fs::write(format!("{out}.sk"), s.as_bytes())?;
    std::fs::write(format!("{out}.vk"), s.verifying_key().as_bytes())?;
    Ok(())
}
