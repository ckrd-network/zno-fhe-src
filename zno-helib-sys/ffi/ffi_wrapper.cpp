#include "ffi_wrapper.h"

// example of how you might use these functions:
//
// helib::BGVContextBuilder builder;
// helib::set_m(builder, 5);
// helib::set_p(builder, 2);
// // ... more settings ...
// Context* context = helib::build_ptr(builder);

namespace helib {

    std::unique_ptr<std::vector<long int>> to_std_vector(const rust::cxxbridge1::Vec<long int>& rustVec) {
        // Create an empty std::vector inside a unique_ptr
        auto stdVec = std::make_unique<std::vector<long int>>();

        // Iterate over the rust::cxxbridge1::Vec and push elements into the std::vector
        for (const auto& item : rustVec) {
            stdVec->push_back(item);
        }

        return stdVec; // this transfers ownership to the caller
    }

    std::unique_ptr<::helib::BGVContextBuilder> new_bgv_builder() {
        return std::make_unique<::helib::BGVContextBuilder>();
    }

    std::unique_ptr<helib::Context> build_ptr(std::unique_ptr<::helib::BGVContextBuilder> builder) {
        try {
            if (!builder) {
                throw std::invalid_argument("Received null for builder in build_ptr");
            }

            helib::Context* raw_ptr = builder->buildPtr();
            return std::unique_ptr<helib::Context>(raw_ptr);
        } catch(const std::exception& e) {
            // Log the error message if needed: e.what()
            return nullptr; // Indicates an error occurred.
        }
    }


    std::unique_ptr<::helib::BGVContextBuilder> set_m(std::unique_ptr<::helib::BGVContextBuilder> builder, uint32_t m) {
        builder->m(m);  // Assumes `m` modifies the object and is void.
        return builder; // Return the unique_ptr.
    }

    BGVContextBuilder& set_p(BGVContextBuilder& builder, uint32_t p) {
        builder.p(p);
        return builder;
    }

    BGVContextBuilder& set_r(BGVContextBuilder& builder, uint32_t r) {
        builder.r(r);
        return builder;
    }

    BGVContextBuilder& set_bits(BGVContextBuilder& builder, uint32_t bits) {
        builder.bits(bits);
        return builder;
    }

    BGVContextBuilder& set_c(BGVContextBuilder& builder, uint32_t c) {
        builder.c(c);
        return builder;
    }

    BGVContextBuilder& is_bootstrappable(BGVContextBuilder& builder, bool flag) {
        builder.bootstrappable(flag);
        return builder;
    }

    BGVContextBuilder& set_gens(BGVContextBuilder& builder, const rust::cxxbridge1::Vec<long int>& gens) {
        auto gens_std_ptr = to_std_vector(gens); // This is now a std::unique_ptr<std::vector<long int>>
        builder.gens(*gens_std_ptr); // Dereference the std::unique_ptr to get the std::vector
        return builder;
    }

    BGVContextBuilder& set_ords(BGVContextBuilder& builder, const rust::cxxbridge1::Vec<long int>& ords) {
        auto ords_std_ptr = to_std_vector(ords); // This is now a std::unique_ptr<std::vector<long int>>
        builder.ords(*ords_std_ptr); // Dereference the std::unique_ptr to get the std::vector
        return builder;
    }

    BGVContextBuilder& set_mvec(BGVContextBuilder& builder, const rust::cxxbridge1::Vec<long int>& mvec) {
        auto mvec_std_ptr = to_std_vector(mvec); // This is now a std::unique_ptr<std::vector<long int>>
        builder.mvec(*mvec_std_ptr); // Dereference the std::unique_ptr to get the std::vector
        return builder;
    }

    BGVContextBuilder& set_thickboot(BGVContextBuilder& builder) {
        builder.thickboot();
        return builder;
    }

    void set_thinboot(BGVContextBuilder& builder) {
        builder.thinboot();
        // return builder;
    }

    std::unique_ptr<OptionalLong> get_m(const Context& context) {
        auto result = std::make_unique<OptionalLong>();
        std::optional<long> opt = context.getM();

        if (opt.has_value()) {
            result->has_value = true;
            result->value = *opt;
        } else {
            result->has_value = false;
            // value can remain uninitialized or be set to 0, since it's ignored when has_value is false
        }

        return result;
    }

}  // namespace helib_wrapper
