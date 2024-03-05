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

    using Schema = scheme_type;
    using SecurityLevel = sec_level_type;

    std::unique_ptr<std::string> version();

    std::unique_ptr<BGVContextBuilder> init(std::unique_ptr<Schema> schema);

    std::unique_ptr<::seal::Context> build(std::unique_ptr<::seal::BGVContextBuilder> builder);

    // // This function returns a new Context pointer
    // std::unique_ptr<::seal::Context> build_ptr(std::unique_ptr<::seal::BGVContextBuilder> builder);

    std::unique_ptr<std::vector<long int>> to_std_vector(const rust::cxxbridge1::Vec<long int>& rustVec);

    // class SEALContext;

    // std::unique_ptr<seal::Schema> ckks() {
    //   return std::make_unique<seal::scheme_type>(seal::scheme_type::ckks);
    // }

    //     class Ciphertext {
    //     public:
    //         void load(const SEALContext &context, const std::string &in_str);
    //         // ... other methods
    //     };

    //     class Plaintext {
    //     public:
    //         Plaintext load(const SEALContext &context, const std::string &in_str);
    //         // ... other methods
    //     };

    //     class SecretKey {
    //     public:
    //         void load(const SEALContext &context, const std::string &in_str);
    //         // ... other methods
    //     };

    //     class PublicKey {
    //     public:
    //         void load(const SEALContext &context, const std::string &in_str);
    //         // ... other methods
    //     };

    //     class RelinKeys {
    //     public:
    //         void load(const SEALContext &context, const std::string &in_str);
    //         // ... other methods
    //     };

    // class GaloisKeysWrapper : public seal::GaloisKeys {
    // public:
    //     GaloisKeysWrapper() : seal::GaloisKeys() {}

    //     std::streamoff loader(const seal::SEALContext &context, const std::string &in_str) {
    //         std::istringstream in_stream(in_str);
    //         return this->load(context, in_stream);
    //     }
    // };

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
