use helib::prelude::*;
use std::error::Error;
use std::fmt;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::num::ParseIntError;

fn main() {

    let params = BGVParams {
        m: M::new(4095).unwrap(),
        p: P::new(2).unwrap(),
        r: R::new(1).unwrap(),
        c: C::new(2).unwrap(),
        bits: Bits::new(500).unwrap(),
        gens: "2341, 3277, 911".parse::<Gens>().unwrap(),
        ords: "6, 4, 6".parse::<Ords>().unwrap(),
        mvec: "7, 5, 9, 13".parse::<Mvec>().unwrap(),
        bootstrappable: Bootstrappable::new(true),
    };
    // Parsing phase
    let params = BGVParams::try_from("some_input_string")?;

    let he = HESetup::new(&params);

    println!(
        "*********************************************************\n\
         *            Basic Binary Arithmetic Example            *\n\
         *********************************************************"
    );

    let bit_size = 16;

    let a = RandomNumber::new(bit_size);
    let b = RandomNumber::new(bit_size);
    let c = RandomNumber::new(bit_size);

    println!("Pre-encryption data:");
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);

    let encrypted_a = he.encrypt_number(&a)?;
    let encrypted_b = he.encrypt_number(&b)?;
    let encrypted_c = he.encrypt_number(&c)?;

    println!("Encrypted a: {:?}", encrypted_a.0.len()); // this is just the length for demonstration
    println!("Encrypted b: {:?}", encrypted_b.0.len());
    println!("Encrypted c: {:?}", encrypted_c.0.len());

    let encrypted_sum_result = [&encrypted_a.0, &encrypted_b.0, &encrypted_c.0].concat().iter().sum();
    let encrypted_sum_result = encrypted_a.add(&encrypted_b, &env.ea)?.add(&encrypted_c, &env.ea)?;

    let encrypted_product_result = encrypted_a.0.iter().zip(encrypted_b.0.iter()).map(|(a, b)| a * b).collect::<Vec<_>>();

    let encrypted_mul_add = encrypted_a.multiply(&encrypted_b, &env.ea)?.add(&encrypted_c, &env.ea)?;

    // Decryption and result presentation
    // Assuming helib-rust-ffi provides a popcnt method for Vec<Ctxt>
    let encrypted_popcnt_result = helib::popcnt(&encrypted_a.0);

    let result_mul_add = env.decrypt_number(&encrypted_mul_add);
    println!("a*b+c = {}", result_mul_add);

    let result_sum = env.decrypt_number(&encrypted_sum_result);
    println!("a+b+c = {}", result_sum);

    let result_popcnt = env.decrypt_number(&encrypted_popcnt_result);
    println!("popcnt(a) = {}", result_popcnt);
    // Ensure you handle errors properly here and return Results as necessary
}

#[non_exhaustive]
struct ContextBuilder{
    parser: PhantomData
}

struct Context(helib::Context);

impl Parser<BGV>{
    // logic specific to BGV
    fn parse(self)-> Context {
        // ...
        Context.m(params.m)
            .p(params.p)
            .r(params.r)
            .gens(params.gens)
            .ords(params.ords)
            .bits(params.bits)
            .c(params.c)
            .bootstrappable(true)
            .mvec(params.mvec)
            .build()
    }

}

struct BGV;

/// Represents the encryption parameters for the BGV scheme as implemented in HElib.
///
/// BGV (Brakerski-Vaikuntanathan) is a versatile fully homomorphic encryption scheme
/// that supports both "leveled" and "full" FHE computations.
///
/// # Parameters
/// - `m`: A positive integer representing the cyclotomic polynomial `Φ_m(X)`. It affects
///        the number of slots available for encoding messages before encryption. Optimal
///        choice of `m` is crucial for both security and performance.
/// - `p` and `r`: `p` is the plaintext base and `r` is its exponent. The plaintext space
///        is typically `Z_p^r`. For binary operations, `p` is often set to 2.
/// - `bits`: Defines the bit-size for the modulus chain. A larger bit-size allows more
///           levels of operations but increases computation and space requirements.
/// - `c`: The number of columns in the key-switching matrices. It affects the noise and
///        size of the ciphertext.
/// - `gens` and `ords`: Generators and their respective orders for the polynomial ring.
///                      They assist in constructing the encryption ring.
/// - `mvec`: Related to bootstrapping, it helps in defining additional structure.
/// - `bootstrappable`: A boolean indicating if the setup allows bootstrapping, a technique
///                     to reduce noise and refresh ciphertexts for more computation levels.
///
/// # Example
/// ```
/// let params = BGVParams {
///     m: 4095,
///     p: 2,
///     r: 1,
///     bits: 500,
///     c: 2,
///     gens: vec![2341, 3277, 911],
///     ords: vec![6, 4, 6],
///     mvec: vec![7, 5, 9, 13],
///     bootstrappable: true,
/// };
/// ```
#[derive(Clone, Debug)]
pub struct BGVParams {
    /// Defines the cyclotomic polynomial `Φ_m(X)`.
    pub m: i64,
    /// The plaintext base.
    pub p: i64,
    /// Exponent of the plaintext base.
    pub r: i64,
    /// Bit-size for the modulus chain.
    pub bits: i64,
    /// Number of columns in the key-switching matrices.
    pub c: i64,
    /// Generators for the polynomial ring.
    pub gens: Vec<i64>,
    /// Orders of the generators.
    pub ords: Vec<i64>,
    /// Defines additional structure, often related to bootstrapping.
    pub mvec: Vec<i64>,
    /// Flag to indicate if parameters are set for bootstrapping.
    pub bootstrappable: bool,
}

pub enum M {
    Some(NonZeroU32),  // assuming 4095 as a possible value, you can add
    Err(ParseIntError),
}

impl TryFrom<&str> for BGVParams {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Proper parsing logic here...
    }
}

struct HESetup {
    context: Context,
    secret_key: SecretKey,
    public_key: PublicKey,
    ea: EncryptedArray,
    nslots: usize,
    bootstrappable: boolean,
    unpack_slot_encoding: Vec<helib::zzX>,
}

impl HESetup {
    fn new(params: &EncryptionParams) -> Self {
        let context = build_context(params);
        let secret_key = SecretKey::new(&context);
        let public_key = PublicKey::new(&secret_key);

        let ea = context.ea();
        let nslots = ea.size();
        let unpack_slot_encoding = build_unpack_slot_encoding(ea);
        HESetup {
            context,
            secret_key,
            public_key,
            ea,
            nslots,
            bootstrappable: true,
            unpack_slot_encoding,
        }
    }
    fn build_context(params: EncryptionParams) -> Context {
        ContextBuilder::new(params)
    }

    fn encrypt_number(&self, number: &RandomNumber) -> Result<EncryptedVec, Box<dyn std::error::Error>> {
        let mut encrypted_data = vec![0; number.bit_size];
        for i in 0..number.bit_size {
            encrypted_data.push(encrypt_bit(self.public_key, self.ea, number.value, i));
        }
        Ok(encrypted_data)
    }

    fn encrypt_bit(public_key: &PubKey, ea: &EncryptedArray, data: i64, shift: usize) -> Result<EncryptedVec, Box<dyn std::error::Error>> {
        let bit = (data >> shift) & 1;
        let mut ctxt = EncryptedVec::new(public_key);
        ea.encrypt(&mut ctxt, public_key, vec![bit]);
        Ok(ctxt)
    }

    fn decrypt_number(&self, encrypted: &EncryptedVec) -> i64 {
        let mut decrypted_data = Vec::new();
        decrypt_binary_nums(
            &mut decrypted_data,
            &encrypted.0,
            &self.secret_key,
            &self.ea,
        );
        decrypted_data.last().cloned().unwrap_or(0)
    }
}
struct SecretKey(helib::SecKey);

impl SecretKey {
    fn new(context: &helib::Context) -> Self {
        let mut sk = helib::SecKey::new(context);
        sk.gen_seckey();
        sk.gen_re_crypt_data();
        Self(sk)
    }
}

struct PublicKey(helib::PubKey);

impl From<&SecretKey> for PublicKey {
    fn from(sk: &SecretKey) -> Self {
        Self(sk.0.get_pub_key().clone())
    }
}

// Newtype (a.k.a: tuple-struct)
struct EncryptedVec(Vec<CipherText>);

impl EncryptedVec {

    fn multiply(&self, other: &Self, ea: &EncryptedArray) -> Result<Self, Box<dyn Error>> {
        let result = self.0.iter().zip(&other.0)
            .map(|(a, b)| a.multiply(b))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(EncryptedVec(result))
    }

    fn add(&self, other: &Self, ea: &EncryptedArray) -> Result<Self, Box<dyn Error>> {
        let result = self.0.iter().zip(&other.0)
            .map(|(a, b)| a.add(b))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(EncryptedVec(result))
    }

    // You can add more methods for other arithmetic operations as needed.
}

struct RandomNumber {
    bit_size: usize,
    value: i64,
}

impl RandomNumber {
    fn new(bit_size: usize) -> Self {
        let value = rand::thread_rng().gen_range(0, 2i64.pow(bit_size as u32));
        Self { bit_size, value }
    }
}

impl fmt::Display for RandomNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryFrom<&str> for BGVParams {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Parse the input string to populate the EncryptionParams.
        // Return error if the parsing fails.
        // Placeholder for the actual parsing logic:
        Ok(EncryptionParams {
            p: 2,
            m: 4095,
            r: 1,
            bits: 500,
            c: 2,
            mvec: vec![7, 5, 9, 13],
            gens: vec![2341, 3277, 911],
            ords: vec![6, 4, 6],
        })
    }
}