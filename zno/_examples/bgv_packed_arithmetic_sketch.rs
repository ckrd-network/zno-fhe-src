use std::sync::{Arc, Mutex};
use std::thread;

pub struct PublicKey {
    // fields for the public key
}

pub struct SecretKey {
    public_key: PublicKey,
    // additional fields for the secret key
}

pub struct SecretKeyBuilder {
    public_key: Option<PublicKey>,
    // additional fields for the secret key
}

impl SecretKeyBuilder {
    pub fn new() -> Self {
        Self {
            public_key: None,
            // initialize the additional fields with default values
        }
    }

    pub fn public_key(mut self, public_key: PublicKey) -> Self {
        self.public_key = Some(public_key);
        self
    }

    pub fn get_public_key(&self) -> Option<&PublicKey> {
        self.public_key.as_ref()
    }

    pub fn set_public_key(&mut self, public_key: PublicKey) {
        self.public_key = Some(public_key);
    }

    // methods to set the additional fields of the secret key

    pub fn build(self) -> Result<SecretKey, &'static str> {
        let public_key = self.public_key.ok_or("Missing public key")?;
        Ok(SecretKey {
            public_key,
            // initialize the additional fields with the values from the builder
        })
    }
}

impl SecretKeyBuilder {
    pub fn new() -> Self {
        Self {
            public_key: None,
            // initialize the additional fields with default values
        }
    }

    pub fn public_key(mut self, public_key: PublicKey) -> Self {
        self.public_key = Some(public_key);
        self
    }

    // methods to set the additional fields of the secret key

    pub fn build(self) -> Result<SecretKey, &'static str> {
        let public_key = self.public_key.ok_or("Missing public key")?;
        Ok(SecretKey {
            public_key,
            // initialize the additional fields with the values from the builder
        })
    }
}

pub struct Plaintext<T> {
    // fields for the plaintext
    context: Arc<Mutex<Context>>,
    vector: Vec<T>,
}

pub struct Ciphertext<T> {
    // fields for the ciphertext
    context: Arc<Mutex<Context>>,
    vector: Vec<T>,
}

impl PublicKey {
    pub fn encrypt(&self, plaintext: &Plaintext) -> Ciphertext {
        // encryption logic here
    }
}

impl Ciphertext {
    pub fn decrypt(&self, secret_key: &SecretKey) -> Plaintext {
        // decryption logic here
    }

    // Other operations you can perform on a ciphertext, like addition and multiplication,
    // would also be methods here.
}
let public_key = PublicKey {
    // initialize the public key
};

let secret_key = SecretKeyBuilder::new()
    .public_key(public_key)
    // set the additional fields of the secret key
    .build()
    .expect("Failed to build secret key");

let plaintext = Plaintext {
    // initialize the plaintext
};

let ciphertext = public_key.encrypt(&plaintext);

// perform operations on the ciphertext

let decrypted = ciphertext.decrypt(&secret_key);

let mut ptxt = context.init(vec![0,1,2,3]);
let mut etxt = ptxt.encrypt();
let mut dtxt = etxt.decrypt();

if ptxt == dtxt {
    println!("SLICES ARE EQUAL");
}