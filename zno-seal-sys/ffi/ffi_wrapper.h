#ifndef FFI_WRAPPER_H
#define FFI_WRAPPER_H

#pragma once

#include <memory>
#include <optional>
#include <sstream>
#include <vector>

#include <seal/seal.h>

#include <rust/cxx.h>

namespace seal {

    // Declare the type alias in the seal namespace after including the ContextBuilder definition.
    using BGVContextBuilder = EncryptionParameters;
    using Parameters = EncryptionParameters;
    using Context = SEALContext;

    using SecurityLevel = sec_level_type;

  /**
   * Returns the version string of the SEAL library.
   *
   * @brief The version string follows the format `major.minor.patch`.
   *
   * The `version` function is a simple function that returns the version string of the SEAL library.
   * The version string is a sequence of `.` delimited integers that represents the version of the SEAL library.
   *
   * The SEAL library uses semantic versioning, which means that the version string follows the format `major.minor.patch`.
   * Where `major`, `minor`, and `patch` are integers.
   *
   *   - The `major` version is incremented for incompatible changes.
   *   - The `minor` version is incremented for new features.
   *   - The `patch` version is incremented for bug fixes.
   *
   * @return The version string.
   */
  std::unique_ptr<std::string> version();

  /**
   * @brief Creates a new EncryptionParameters object with the given scheme.
   *
   * @param scheme The encryption scheme to be used. This should be a value from the seal::scheme_type enum.
   * @return std::unique_ptr<seal::EncryptionParameters> A unique pointer to the created EncryptionParameters object.
   */
  std::unique_ptr<seal::EncryptionParameters> new_encryption_parameters(uint8_t scheme);

  /**
   * @brief Retrieves the encryption scheme used in the given EncryptionParameters object.
   *
   * @note This function does not modify the unique_ptr or the EncryptionParameters object it points to. It only reads from the EncryptionParameters object. Therefore, we pass the `std::unique_ptr` by const reference. This means that the function will not change the state of the `std::unique_ptr` itself (i.e., it won't make it point to a different `EncryptionParameters` object) and it will not change the state of the `EncryptionParameters` object that the `std::unique_ptr` is pointing to (i.e., it won't change any of the properties or call any non-const methods on the `EncryptionParameters` object).  In other words, this function is a "read-only" operation on the `std::unique_ptr` and the `EncryptionParameters` object it points to. It won't change anything about them; it will only retrieve information from them.
   *
   * @param enc_params A const reference to a unique pointer to the EncryptionParameters object.
   * @return uint8_t The encryption scheme used in the EncryptionParameters object. This is a value from the seal::scheme_type enum.
   */
  uint8_t get_scheme(const std::unique_ptr<seal::EncryptionParameters>& enc_params);
  /**
   * @brief Sets the encryption scheme in the given EncryptionParameters object.
   *
   * @note This function will replace the existing EncryptionParameters object with a new one that uses the given scheme.
   *
   * @param enc_params A unique pointer to the EncryptionParameters object. This object will be replaced.
   * @param scheme The new encryption scheme to be used. This should be a value from the seal::scheme_type enum.
   */
  void set_scheme(std::unique_ptr<seal::EncryptionParameters>& enc_params, uint8_t scheme);
  /**
   * Creates a new BGVContextBuilder object.
   *
   * @return A unique pointer to the newly created ContextBuilder<BGV> object.
   */
  std::unique_ptr<BGVContextBuilder> init(std::unique_ptr<u_int8_t> schema);

  std::unique_ptr<::seal::Context> build(std::unique_ptr<::seal::BGVContextBuilder> builder);

  // // This function returns a new Context pointer
  // std::unique_ptr<::seal::Context> build_ptr(std::unique_ptr<::seal::BGVContextBuilder> builder);

  std::unique_ptr<std::vector<long int>> to_std_vector(const rust::cxxbridge1::Vec<long int>& rustVec);

  // Optimal parameters for BGV produced by fhegen need only
  //   - set_poly_modulus_degree
  //   - set_coeff_modulus
  //   - set_plain_modulus
  std::unique_ptr<::seal::BGVContextBuilder> set_m(std::unique_ptr<::seal::BGVContextBuilder> builder, uint32_t m);

  // Examples:
  // - seal/native/examples/4_bgv_basics.cpp
  // - seal/native/examples/1_bfv_basics.cpp
  // Require only these functions, so start here:
  //   - BFVDefault(poly_modulus_degree)
  //   - KeyGenerator
  //     - constructor
  //     - secret_key()
  //     - create_public_key(public_key)
  //     - create_relin_keys(relin_keys)
  //   - PublicKey
  //     - constructor
  //   - RelinKeys
  //     - constructor
  //   - Encryptor
  //     - constructor
  //     - encrypt(x_plain, x_encrypted)
  //   - Evaluator
  //     - constructor
  //     - square(x_encrypted, x_squared)
  //     - square_inplace(x_encrypted)
  //     - relinearize_inplace(x_squared, relin_keys)
  //     - mod_switch_to_next_inplace(x_encrypted)
  //     - add_plain_inplace(x_encrypted, x_plain)
  //     - add_plain(x_encrypted, x_plain, y_encrypted)
  //     - multiply(x_encrypted, y_encrypted, x_times_y)
  //     - multiply_plain_inplace(x_encrypted, x_plain)
  //   - Decryptor
  //     - constructor
  //     - invariant_noise_budget(x_encrypted)
  //     - decrypt(x_encrypted, x_plain)
  //   - BatchEncoder
  //     - constructor
  //     - slot_count()
  //     - encode()
  //     - decode(x_decrypted, x_decoded)
  //   - Plaintext
  //     - constructor
  //   - Cyphertext
  //     - constructor
  //     - size()

};  // namespace seal

#endif // FFI_WRAPPER_H

// #ifndef FFI_WRAPPER_H
// #define FFI_WRAPPER_H

// #include "seal/seal.h"  // Include the original header

// // Define a function for the default constructor
// Context* create_context_default();

// // Define a function for the constructor with the first parameter set
// Context* create_context_params(unsigned long m, unsigned long p, unsigned long r, const std::vector<long>& gens, const std::vector<long>& ords);

// // Define a function for the constructor with the second parameter set
// Context* create_context_params_extended(long m, long p, long r, const std::vector<long>& gens, const std::vector<long>& ords, const std::optional<ModChainParams>& mparams, const std::optional<BootStrapParams>& bparams);

// // Define a function for the constructor with serializable content
// Context* create_context_serializable(const SerializableContent& content);

// #endif FFI_WRAPPER_H
