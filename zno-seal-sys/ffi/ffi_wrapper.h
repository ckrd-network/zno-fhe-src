#ifndef FFI_WRAPPER_H
#define FFI_WRAPPER_H

#pragma once

#include <memory>
#include <optional>
#include <vector>

#include <seal/seal.h>

#include <rust/cxx.h>

namespace seal {

    // Declare the type alias in the seal namespace after including the ContextBuilder definition.
    using BGVContextBuilder = ContextBuilder<BGV>;

    std::unique_ptr<BGVContextBuilder> init();

    std::unique_ptr<::seal::Context> build(std::unique_ptr<::seal::BGVContextBuilder> builder);

    // // This function returns a new Context pointer
    // std::unique_ptr<::seal::Context> build_ptr(std::unique_ptr<::seal::BGVContextBuilder> builder);

    std::unique_ptr<std::vector<long int>> to_std_vector(const rust::cxxbridge1::Vec<long int>& rustVec);

    std::unique_ptr<::seal::BGVContextBuilder> set_bits(std::unique_ptr<::seal::BGVContextBuilder> builder, uint32_t bits);
    std::unique_ptr<::seal::BGVContextBuilder> set_c(std::unique_ptr<::seal::BGVContextBuilder> builder, uint32_t c);
    std::unique_ptr<::seal::BGVContextBuilder> set_m(std::unique_ptr<::seal::BGVContextBuilder> builder, uint32_t m);
    std::unique_ptr<::seal::BGVContextBuilder> set_p(std::unique_ptr<::seal::BGVContextBuilder> builder, uint32_t p);
    std::unique_ptr<::seal::BGVContextBuilder> set_r(std::unique_ptr<::seal::BGVContextBuilder> builder, uint32_t r);

    std::unique_ptr<::seal::BGVContextBuilder> set_gens(std::unique_ptr<::seal::BGVContextBuilder> builder, const rust::cxxbridge1::Vec<int32_t>& gens);
    std::unique_ptr<::seal::BGVContextBuilder> set_ords(std::unique_ptr<::seal::BGVContextBuilder> builder, const rust::cxxbridge1::Vec<int32_t>& ords);
    std::unique_ptr<::seal::BGVContextBuilder> set_mvec(std::unique_ptr<::seal::BGVContextBuilder> builder, const rust::cxxbridge1::Vec<uint32_t>& mvec);

    std::unique_ptr<::seal::BGVContextBuilder> is_bootstrappable(std::unique_ptr<::seal::BGVContextBuilder> builder, bool flag);
    std::unique_ptr<::seal::BGVContextBuilder> set_thickboot(std::unique_ptr<::seal::BGVContextBuilder> builder);
    std::unique_ptr<::seal::BGVContextBuilder> set_thinboot(std::unique_ptr<::seal::BGVContextBuilder> builder);

    // Declare the multiplyBy function
    void multiplyBy(std::unique_ptr<::seal::Ctxt>& ciphertext, std::unique_ptr<::seal::Ctxt>& other);


    enum class MErrorKind {
        None,
        OutOfRange,
        Zero,
        Generic,
    };

}  // namespace seal

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
