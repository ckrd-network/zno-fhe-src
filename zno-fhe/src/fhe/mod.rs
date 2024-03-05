mod schema;

pub enum Fhe {
    Builder(Builder),
    Initialized(Initialized),
    Raw(Raw),
    Encrypted(Encrypted),
    Decrypted(Decrypted),
}
