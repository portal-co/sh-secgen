use signature::Keypair;
use slh_dsa::Shake128s;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    args.next();
    let ty = args.next().unwrap();
    let out = args.next().unwrap();
    match ty.as_str() {
        // "signing/ed25519" => {
        //     if let Ok(s) = std::fs::read(format!("{out}.sk")) {
        //         return Ok(());
        //     }
        //     let s = ed25519_dalek::SigningKey::generate(&mut rand::thread_rng());
        //     std::fs::write(format!("{out}.sk"), s.as_bytes())?;
        //     std::fs::write(format!("{out}.vk"), s.verifying_key().as_bytes())?;
        // }
        "kem/xwing" => {
            if let Ok(s) = std::fs::read(format!("{out}.sk")) {
                return Ok(());
            }
            let (sk,pk) = x_wing::generate_key_pair(&mut rand::thread_rng());
            std::fs::write(format!("{out}.sk"), sk.as_bytes())?;
            std::fs::write(format!("{out}.vk"), pk.as_bytes())?;
        }
        "signing/falcon/1024" => {
            if let Ok(s) = std::fs::read(format!("{out}.sk")) {
                return Ok(());
            }
            let (sk,pk) = falcon_rust::falcon1024::keygen(rand::random());
            std::fs::write(format!("{out}.sk"), sk.to_bytes())?;
            std::fs::write(format!("{out}.vk"), pk.to_bytes())?;
        }
        "signing/falcon/512" => {
            if let Ok(s) = std::fs::read(format!("{out}.sk")) {
                return Ok(());
            }
            let (sk,pk) = falcon_rust::falcon512::keygen(rand::random());
            std::fs::write(format!("{out}.sk"), sk.to_bytes())?;
            std::fs::write(format!("{out}.vk"), pk.to_bytes())?;
        }
        "git/init" => {
            let mut r = std::fs::read(format!("{out}/.gitignore")).unwrap_or_else(|_| vec![]);
            static X: &[u8] = b"\n*.sk\n!test.sk\n";
            if !r.windows(X.len()).any(|w| w == X) {
                r.extend_from_slice(X);
            }
            std::fs::write(format!("{out}/.gitignore"), r)?;
        }
        "signing/slh/shake128s" => {
            if let Ok(s) = std::fs::read(format!("{out}.sk")) {
                return Ok(());
            }
            let s = slh_dsa::SigningKey::<Shake128s>::new(&mut rand::thread_rng());
            std::fs::write(format!("{out}.sk"), s.to_bytes())?;
            std::fs::write(format!("{out}.vk"), s.verifying_key().to_bytes())?;
        }
        _ => {
            eprintln!("invalid key type; valid ones are signing/slh/shake128s, kem/xwing, signing/falcon/1024, signing/falcon/512. you can also initialize a repo with git/init")
        }
    };
    Ok(())
}
