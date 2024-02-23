use std::vec;

fn main() {
    /*  Example of BGV scheme  */

    // Plaintext prime modulus
    let p: u64 = 4999;
    // Cyclotomic polynomial - defines phi(m)
    let m: u64 = 32109;
    // Hensel lifting (default = 1)
    let r: u64 = 1;
    // Number of bits of the modulus chain
    let bits: u64 = 500;
    // Number of columns of Key-Switching matrix (default = 2 or 3)
    let c: u64 = 2;

    println!("*********************************************************");
    println!("*         Basic Mathematical Operations Example         *");
    println!("*         =====================================         *");
    println!("*                                                       *");
    println!("* This is a sample program for education purposes only. *");
    println!("* It attempts to show the various basic mathematical    *");
    println!("* operations that can be performed on both ciphertexts  *");
    println!("* and plaintexts.                                       *");
    println!("*                                                       *");
    println!("*********************************************************");
    println!();

    println!("Initialising context object...");
    // Initialize context
    // This object will hold information about the algebra created from the
    // previously set parameters
    let context = helib::ContextBuilder::<helib::BGV>::new()
        .m(m)
        .p(p)
        .r(r)
        .bits(bits)
        .c(c)
        .build();

    // Print the context.
    // TODO:
    // - Implement Display trait for Context
    println!("{}", context);

    // Print the security level
    println!("Security: {}", context.security_level());

    // Secret key management
    println!("Creating secret key...");
    println!("Generating key-switching matrices...");
    // Create a secret key associated with the context
    //    helib::SecKey secret_key(&context);
    // Create a secret key for the context public key, Populate secret_key with polynomial i.e. GenSecKey()
    let mut secret_key = context.secret_key()
                                .key_switching(); // Generate key-switching matrices
    // Generate the secret key
    //     secret_key.GenSecKey();
    // println!("Generating key-switching matrices...");
    // Compute key-switching matrices that we need
    // Uses default parameters: bound = HELIB_KEYSWITCH_THRESH, keyID = 0
    //     helib::addSome1DMatrices(&mut secret_key);

    // Public key management
    // Set the secret key (upcast: SecKey is a subclass of PubKey)
    let public_key = &context.get_public_key();

    // Get the EncryptedArray of the context
    let ea = context.encrypted_array();

    // Get the number of slot (phi(m))
    let nslots = ea.size();
    println!("Number of slots: {}", nslots);

    // Create a vector of long with nslots elements
    let mut ptxt = context.plain_text();
    // Fill it with numbers 0..nslots - 1
    // ptxt = [0] [1] [2] ... [nslots-2] [nslots-1]
    for i in 0..ptxt.size() {
        ptxt[i] = i as i64;
    }

    // Print the plaintext
    println!("Initial Plaintext: {}", ptxt);

    // Create a ciphertext object
    // let mut ctxt = public_key.cipher_text();
    // Encrypt the plaintext using the public_key
    //     public_key.encrypt(&mut ctxt, &ptxt);
    let mut ctxt = public_key.encrypt(&ptxt);

    let mut ptxt = context.init(vec![0,1,2,3]);
    let mut etxt = ptxt.encrypt();
    let mut dtxt = etxt.decrypt();

    /********** Operations **********/
    // Ciphertext and plaintext operations are performed
    // "entry-wise".

    // Square the ciphertext
    // [0] [1] [2] [3] [4] ... [nslots-1]
    // -> [0] [1] [4] [9] [16] ... [(nslots-1)*(nslots-1)]
    ctxt.multiplyBy(&ctxt);
    // Plaintext version
    ptxt.multiplyBy(&ptxt);

    // Divide the ciphertext by itself
    // To do this we must calculate the multiplicative inverse using Fermat's
    // Little Theorem.  We calculate a^{-1} = a^{p-2} mod p, where a is non-zero
    // and p is our plaintext prime.
    // First make a copy of the ctxt using copy constructor
    let ctxt_divisor = ctxt.clone();
    // Raise the copy to the exponent p-2
    // [0] [1] [4] ... [16] -> [0] [1] [1] ... [1]
    // Note: 0 is a special case because 0^n = 0 for any power n
    ctxt_divisor.power(p - 2);
    // a^{p-2}*a = a^{-1}*a = a / a = 1;
    ctxt.multiplyBy(&ctxt_divisor);

    // Plaintext version
    let mut ptxt_divisor = ptxt.clone();
    ptxt_divisor.power(p - 2);
    ptxt.multiplyBy(&ptxt_divisor);

    // Double it (using additions)
    // [0] [1] [1] ... [1] [1] -> [0] [2] [2] ... [2] [2]
    ctxt += &ctxt;
    // Plaintext version
    ptxt += &ptxt;

    // Subtract it from itself (result should be 0)
    // i.e. [0] [0] [0] [0] ... [0] [0]
    ctxt -= &ctxt;
    // Plaintext version
    ptxt -= &ptxt;

    // Create a plaintext for decryption
    let mut plaintext_result = helib::Ptxt::<helib::BGV>::new(&context);
    // Decrypt the modified ciphertext
    secret_key.decrypt(&mut plaintext_result, &ctxt);

    println!("Operation: 2(a*a)/(a*a) - 2(a*a)/(a*a) = 0");
    // Print the decrypted plaintext
    // Should be [0] [0] [0] ... [0] [0]
    println!("Decrypted Result: {}", plaintext_result);
    // Print the plaintext version result, should be the same as the ctxt version
    println!("Plaintext Result: {}", ptxt);

    // We can also add constants
    // [0] [0] [0] ... [0] [0] -> [1] [1] [1] ... [1] [1]
    ctxt.addConstant(&NTL::ZZX::from(1));
    // Plaintext version
    ptxt.addConstant(&NTL::ZZX::from(1));

    // And multiply by constants
    // [1] [1] [1] ... [1] [1]
    // -> [1*1] [1*1] [1*1] ... [1*1] [1*1] = [1] [1] [1] ... [1] [1]
    ctxt *= &NTL::ZZX::from(1);
    // Plaintext version
    ptxt *= &NTL::ZZX::from(1);

    // We can also perform ciphertext-plaintext operations
    // ctxt = [1] [1] [1] ... [1] [1], ptxt = [1] [1] [1] ... [1] [1]
    // ctxt + ptxt = [2] [2] [2] ... [2] [2]
    // Note: the output of this is also a ciphertext
    ctxt += &ptxt;

    // Decrypt the modified ciphertext into a new plaintext
    let mut new_plaintext_result = helib::Ptxt::<helib::BGV>::new(&context);
    secret_key.decrypt(&mut new_plaintext_result, &ctxt);

    println!("Operation: Enc{(0 + 1)*1} + (0 + 1)*1");
    // Print the decrypted plaintext
    // Should be [2] [2] [2] ... [2] [2]
    println!("Decrypted Result: {}", new_plaintext_result);
}