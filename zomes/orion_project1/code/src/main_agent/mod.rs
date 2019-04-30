//this package might possibly be merged with 'broker'

use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};

// todo

fn get_key_pair() -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push("keys");
    path.push("rsa_private.pem");
    path.to_path_buf()
}

fn sign(data: &str) -> Result<String, error::Error> {
    let kp = get_key_pair();
    let mut signer = Signer::new(MessageDigest::sha256(), &kp)?;
    signer.update(data.as_bytes())?;
    signer.sign_to_vec()?
}

fn verify_signature(signature: &str) -> bool {
    let kp = get_key_pair();
    let mut verifier = Verifier::new(MessageDigest::sha256(), &kp)?;
    verifier.update(signature.as_bytes())?;
    verifier.verify(&signature).unwrap()
}
