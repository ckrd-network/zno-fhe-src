use std::sync::{Arc, Mutex};

mod fhe;

pub struct PublicKey {
    // fields for the public key
}

pub struct SecretKey {
    // fields for the secret key
}

// Binding to Sized means borrowing is not possible.
// But that's OK because the point is to prevent borrowing in the first place.
trait Builder<S>: Sized {
    fn init(self, scheme: Scheme) -> Fhe<S, Self>;

    fn build(self) -> Option<S>;
}

// Binding to Sized means borrowing is not possible.
// But that's OK because the point is to prevent borrowing.
trait Encryptable<S, D: Decrypted<S, Self>>: Sized {

    fn encrypt(self, scheme: S) -> Vault<S, D, Self>;
}

// Binding to Sized means borrowing is not possible.
// But that's OK because the point is to prevent borrowing.
trait Decryptable<S, E: Encrypted<S, Self>>: Sized {

    fn decrypt(self, scheme: S) -> Vault<S, Self, E>;
}

enum Fhe<T, B: Builder<T>> {
    Builder(B),
    Initialized(T),
    Raw(T),
}
enum Vault<S, D: Decryptable<S, E>, E: Encryptable<S, D>> {
    Encryptable(S,D),
    Decryptable(S,E),
}

enum Error {
    NotDone
}

// FHE scheme trait
trait Scheme {}
// BGV scheme type
struct Bgv;

impl Scheme for Bgv {}
struct BgvBuilder;

impl Builder<Bgv> for BgvBuilder {

    fn init(self, scheme: Scheme) -> Fhe<Bgv, Self> {
        // commence initializing the BGV scheme...
        // ... then move to the next state
        Fhe::Initialized(Bgv)
    }

    fn build(self) -> Fhe<Bgv, Self> {
        // complete initializing the BGV scheme...
        // ... then move to the next state
        Vault::Decryptable(scheme,Raw)
        // Some(Bgv)
    }
}

fn test() {
    #[zno( scheme=bgv, security=128, batch=true, unencrypted_multiply=false,
        beta=1024, key_switching=hybrid-rns, library=helib,
        cyclotomic_order=1024, /* alt to security */
        model=base, multiplications=2, key_omega=3, rotations=0,
        distribution=ternary, summations=1, plaintext_modulus=0 )]
    let raw = vec![0,1,2,3];
    // The above two lines are equivalent to:
    //
    //      #[cfg(debug_assertions)]
    //      let raw = vec![0,1,2,3];
    //
    //      #[cfg(not(debug_assertions))]
    //      fn zno_init() -> zno::Raw {
    //          let m = zno::bgv::M::new(1);
    //          let s = zno::bgv::S::new(2);
    //          let parameters = zno::Parameters::new(zno::Bgv)
    //                              .from(m)
    //                              .from(s);
    //          let context = Arc::new(Mutex::new(zno::Context::new(parameters)));
    //          let builder = zno::BgvBuilder { context };
    //          let engine = zno::build(zno::Builder(builder));
    //          zno::Raw::new(engine, vec![0,1,2,3])
    //      }
    //
    //      #[cfg(not(debug_assertions))]
    //      let raw: zno::Raw = zno_init();

    let enc = raw.encrypt();
    let dec = enc.decrypt();
    let raw = dec.raw();
    assert_eq!(vec![0,1,2,3], raw);
}

fn build<S, B: Builder<S>>(target: Fhe<S, B>) -> Result<S, Error> {
    // in production, `scheme` comes from an environment variable, proc-macro, configuration file, etc.
    let scheme = Some(Bgv);

    match target {
        Fhe::Builder(builder) => {
            if let Some(s) = scheme {
                build(builder.init(s))
            } else {
                Ok(builder.done().ok_or(Error::NotDone)?)
            }
        }
        Fhe::Initialized(scheme) => Ok(scheme),
        Fhe::Raw(_) => todo!(),
    }
}

// Marker trait for the generic constraint on the Fhe types:
//
trait Fhe {}

pub struct Rawtext {
    // fields for the rawtext
}

pub struct Decryptedtext {
    // fields for the decryptedtext
}

pub struct Encryptedtext {
    // fields for the encryptedtext
}

// The raw state
pub struct Raw {
    rawtext: Rawtext,
}

// The initialized state
// context: Arc<Mutex<Context>>,
pub struct Initialized {
    rawtext: Rawtext,
}


// The encrypted state
pub struct Encrypted {
    encryptedtext: Encryptedtext,
}

// The decrypted state
pub struct Decrypted {
    decryptedtext: Decryptedtext,
}

// Transitions:
//    Raw       -> Initialized
//    Initialized -> Encrypted
//    Encrypted -> Decrypted
//    Decrypted -> Encrypted

// Raw is the initial state, so create a function to make a new Raw.
// All usage must go through this function first, which kicks off the entire process.
// Here the FHE context is initialized, encryption keys created, etc.

impl Raw {
    pub fn new(rawtext: Rawtext) -> Self {
        Self { rawtext }
    }

    // the encrypt transition function
    pub fn encrypt(self, public_key: &PublicKey) -> Encrypted {
        // encryption logic here
        let encryptedtext = Encryptedtext {
            // fields for the Encryptedtext
        };
        Encrypted { encryptedtext }
    }
}

impl Decryptable<S, D: Decrypted<S, Self>> for Encrypted {
    // pub fn new(decryptedtext: Decryptedtext) -> Self {
    //     Self { decryptedtext }
    // }

    // The encrypt transition function.
    // Move from the Decrypted state to the Encrypted state.
    fn decrypt(self, public_key: &PublicKey) -> D {
        // encryption logic here
        let decryptedtext = Decryptedtext {
            // fields for the decryptedtext
        };
        D { decryptedtext }
    }
}

impl<S, D: Decryptable<S, E>, E: Encryptable<S, D>> Decryptable<S, E> for Vault<S, D, E> {
    fn decrypt(self, scheme: S) -> Vault<S, D, E> {
        self.decrypt(scheme)
    }
}

impl Encryptable<S, E: Encrypted<S, Self>> for Decrypted {
    // The decrypt transition function.
    // Move from the Encrypted state to the Decrypted state.
    fn encrypt(self, secret_key: &SecretKey) -> Vault<S, E> {
        // decryption logic here
        let encryptedtext = Encryptedtext {
            // fields for the Encryptedtext
        };
        E { encryptedtext }
    }
}