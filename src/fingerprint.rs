use bip32::{Prefix, XPrv};
use bip39::{Language, Mnemonic};

pub fn create_fingerprint(
    seed: impl AsRef<str>,
    passphrase: impl AsRef<str>,
) -> eyre::Result<[u8; 4]> {
    let mneomic = Mnemonic::parse_in(Language::English, seed.as_ref())?;
    let seed = mneomic.to_seed(passphrase.as_ref());

    let xpriv = XPrv::new(seed)?;
    let fingerprint = xpriv.public_key().fingerprint();

    Ok(fingerprint)
}

pub fn create_xpub(seed: impl AsRef<str>, passphrase: impl AsRef<str>) -> eyre::Result<String> {
    // Derive a BIP39 seed value using the given password
    let mneomic = Mnemonic::parse_in(Language::English, seed.as_ref())?;
    let seed = mneomic.to_seed(passphrase.as_ref());

    let child_path = "m/84'/0'/0'";
    let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse()?)?;

    // Get the `XPub` associated with `child_xprv`.
    let xpub = child_xprv.public_key();

    Ok(xpub.to_string(Prefix::XPUB))
}

#[cfg(test)]
mod test {
    use pretty_assertions_sorted::assert_eq;

    #[test]
    fn empty_passphrase_creates_xpub() {
        let seed = "build since save grit begin key leisure similar royal diagram warfare execute laptop dress occur sword use soon above obtain beyond merry notable typical";
        let passphrase = "";

        assert_eq!(super::create_xpub(seed, passphrase).unwrap(), "xpub6Ct5LukVDks4tNsW9PFaKUjCW5dUNzHKL3QmowFSFb42niqFqwU1izyFoWfThGfjrJKg1ezd4dv8ErtqkoHTxofckeZUeDBXS6mLiV8uYUF")
    }

    #[test]
    fn empty_passphrase_creates_fingerprint() {
        let seed = "build since save grit begin key leisure similar royal diagram warfare execute laptop dress occur sword use soon above obtain beyond merry notable typical";
        let passphrase = "";

        assert_eq!(
            &super::create_fingerprint(seed, passphrase).unwrap(),
            hex::decode("80962006").unwrap().as_slice()
        )
    }

    #[test]
    fn with_passphrase_creates_fingerprint() {
        let seed = "build since save grit begin key leisure similar royal diagram warfare execute laptop dress occur sword use soon above obtain beyond merry notable typical";
        let passphrase = "benefitwifesoccerrookienationspecialchild";

        assert_eq!(
            hex::encode(super::create_fingerprint(seed, passphrase).unwrap()),
            "af849feb"
        )
    }
}
