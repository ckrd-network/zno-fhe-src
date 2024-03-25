#[derive(Encryptable,Decryptable)]
#[zno(scheme = Bgv, operators = +, operations = 1, security = 128)]
pub struct FheU64(u64);

// Take two encrypted values, return their encrypted sum as a decryptable number.
// fn myapp<A: Fhe, B: Fhe>(a: A, b: B) {}
// #[zno()]
pub fn myapp<A: Fhe, B: Fhe>(a: A, b: B) -> Result<A, Error>{
    // proc macro will impl an into() for FheU64:
    //
    a + b
}

#[derive(Decryptable)]
pub struct MyFhe(u64);

// Take two encrypted values, return their encrypted sum as a decryptable number.
// fn myapp2<A: Fhe, B: Fhe>(a: A, b: B) => Result<> {}
#[zno()]
pub fn myapp2(a: Fhe, b: Fhe) -> Fhe {
    a + b
}

pub fn main() {
    let a = FheU64(1).encrypt();
    let b = FheU64(2).encrypt().decrypt();
    let c: FheU64 = myapp(a, b);
    println!("1 + 2 = {:?} (expect 3)", c.decrypt());

    let aa = FheU64(1).encrypt();
    let ab = FheU64(2);
    let ac: MyFhe = myapp(aa, ab);
    println!("1 + 2 = {:?} (expected 3)", ac.decrypt());
}
