#ifndef FFI_WRAPPER_H
#define FFI_WRAPPER_H

#pragma once

#include <memory>
#include <optional>
#include <vector>
#include <vector>

#include <helib/helib.h>
#include <helib/Context.h>  // You might need to adjust the path based on your project's directory structure.// Assuming this is the path to the ContextBuilder within HElib
#include <helib/EncryptedArray.h>  // For potential bootstrapping flags or other parameters
#include <helib/apiAttributes.h>
#include <helib/Context.h>

#include <rust/cxx.h>
    std::unique_ptr<helib::Context> create_bgv_context_wrapper(
        uint64_t m,
        uint64_t p,
        uint64_t r,
        uint64_t bits,
        const rust::cxxbridge1::Vec<long int>& gens,
        const rust::cxxbridge1::Vec<long int>& ords
    );
namespace helib {

    // Declare the type alias in the helib namespace after including the ContextBuilder definition.
    using BGVContextBuilder = ContextBuilder<BGV>;

    std::unique_ptr<std::vector<long int>> to_std_vector(const rust::cxxbridge1::Vec<long int>& rustVec);

    std::unique_ptr<::helib::BGVContextBuilder> new_bgv_builder();
    std::unique_ptr<::helib::BGVContextBuilder> set_m(std::unique_ptr<::helib::BGVContextBuilder> builder, int32_t m);
    BGVContextBuilder& set_p(BGVContextBuilder& builder, long p);
    BGVContextBuilder& set_r(BGVContextBuilder& builder, long r);
    // ... Other setter functions ...
    BGVContextBuilder& set_gens(BGVContextBuilder& builder, const rust::cxxbridge1::Vec<long int>& gens);
    BGVContextBuilder& set_ords(BGVContextBuilder& builder, const rust::cxxbridge1::Vec<long int>& ords);
    BGVContextBuilder& set_bits(BGVContextBuilder& builder, long bits);
    BGVContextBuilder& set_c(BGVContextBuilder& builder, long c);
    BGVContextBuilder& is_bootstrappable(BGVContextBuilder& builder, bool flag);
    BGVContextBuilder& set_mvec(BGVContextBuilder& builder, const rust::cxxbridge1::Vec<long int>& mvec);
    BGVContextBuilder& set_thickboot(BGVContextBuilder& builder);
    void set_thinboot(BGVContextBuilder& builder);

    Context* build_ptr(BGVContextBuilder& builder); // This function will now return a new Context pointer


}  // namespace helib

#endif // FFI_WRAPPER_H

// #ifndef FFI_WRAPPER_H
// #define FFI_WRAPPER_H

// #include "helib/helib.h"  // Include the original header

// // Define a function for the default constructor
// Context* create_context_default();

// // Define a function for the constructor with the first parameter set
// Context* create_context_params(unsigned long m, unsigned long p, unsigned long r, const std::vector<long>& gens, const std::vector<long>& ords);

// // Define a function for the constructor with the second parameter set
// Context* create_context_params_extended(long m, long p, long r, const std::vector<long>& gens, const std::vector<long>& ords, const std::optional<ModChainParams>& mparams, const std::optional<BootStrapParams>& bparams);

// // Define a function for the constructor with serializable content
// Context* create_context_serializable(const SerializableContent& content);

// #endif FFI_WRAPPER_H
