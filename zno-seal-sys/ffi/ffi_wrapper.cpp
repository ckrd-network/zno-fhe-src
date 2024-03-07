#include "ffi_wrapper.h"

/**
 * This file contains the implementation of various functions related to the Foreign Function Interface (FFI) for the seal library.
 * The FFI allows Rust and C++ to interoperate by providing functions that can be called from Rust code.
 * The functions in this file convert data types between Rust and C++ and provide an interface for manipulating the seal library objects.
 */

/**
 * Builds a Context object from a BGVContextBuilder object.
 *
 * @param builder A pointer to the BGVContextBuilder object.
 * @return A unique pointer to the newly created Context object.
 */

namespace seal {

  std::unique_ptr<std::string> version() {
      SEALVersion version;
      auto version_ptr = std::make_unique<std::string>(std::to_string(version.major) + "." + std::to_string(version.minor) + "." + std::to_string(version.patch));
      return version_ptr;
  }

  /**
   * Converts a C-style array to an std::vector.
   *
   * @param array The C-style array to convert.
   * @param size The size of the array.
   * @return An std::vector containing the elements of the array.
   * @note This function is used to convert the `gens`, `ords`, and `mvec` arrays
   * The provided C++ function `to_std_vector` is designed to convert a Rust vector (`rust::cxxbridge1::Vec<long int>`) into a standard C++ vector (`std::vector<long int>`), wrapped in a `std::unique_ptr`. This function is likely part of a Foreign Function Interface (FFI) layer, which allows Rust and C++ to interoperate.
   * The function takes a constant reference to a Rust vector as its argument. This means it can read the Rust vector, but it can't modify it.
   *
   * Inside the function, an empty `std::vector<long int>` is created and wrapped in a `std::unique_ptr`. The `std::unique_ptr` is a smart pointer that retains sole ownership of an object through a pointer and destroys that object when the `unique_ptr` goes out of scope. The `std::make_unique` function is a utility that creates a `unique_ptr` with a new object.
   *
   * Next, the function iterates over the Rust vector using a range-based for loop. For each item in the Rust vector, it pushes the item into the C++ vector. The `push_back` function is a standard method on `std::vector` that appends an element to the end of the vector.
   *
   * Finally, the function returns the `unique_ptr` to the C++ vector. This transfers ownership of the vector to the caller. This is important because it ensures that the memory for the vector will be properly deallocated when the caller is done with it. This is a key part of how C++ manages memory and prevents leaks.
   *
   */
  std::unique_ptr<std::vector<long int>> to_std_vector(const rust::cxxbridge1::Vec<long int>& rustVec) {
    // Create an empty std::vector inside a unique_ptr
    auto stdVec = std::make_unique<std::vector<long int>>();

    // Iterate over the rust::cxxbridge1::Vec and push elements into the std::vector
    for (const auto& item : rustVec) {
      stdVec->push_back(item);
    }

    return stdVec; // this transfers ownership to the caller
  }

  /**
   * Creates a new BGVContextBuilder object.
   *
   * @return A unique pointer to the newly created ContextBuilder<BGV> object.
   */
  std::unique_ptr<BGVContextBuilder> init(std::unique_ptr<Schema> schema) {
      EncryptionParameters parms(*schema);
      return std::make_unique<BGVContextBuilder>(parms);
  }

  // std::unique_ptr<::seal::Context> build(std::unique_ptr<::seal::BGVContextBuilder> builder) {
  //   return std::unique_ptr<seal::Context>(builder->buildPtr());
  // }

  /**
   * Sets the value of `m` in the BGVContextBuilder object.
   *
   * @param builder The BGVContextBuilder object.
   * @param m The value of `m` to be set.
   * @return A unique_ptr to the modified BGVContextBuilder object.
   */
  std::unique_ptr<::seal::BGVContextBuilder> set_m(std::unique_ptr<::seal::BGVContextBuilder> builder, uint32_t m) {
    builder->set_poly_modulus_degree(m);  // Assume `set_poly_modulus_degree` modifies the object and is void.
    return builder; // Return the unique_ptr.
  }

  // /**
  //  * Sets the value of `m` in the BGVContextBuilder object.
  //  *
  //  * @param builder The BGVContextBuilder object.
  //  * @param m The value of `m` to be set.
  //  * @return A unique_ptr to the modified BGVContextBuilder object.
  //  */
  // std::unique_ptr<::seal::BGVContextBuilder> set_m(std::unique_ptr<::seal::BGVContextBuilder> builder, uint32_t m) {
  //   builder->m(m);  // Assume `m` modifies the object and is void.
  //   return builder; // Return the unique_ptr.
  // }

  // /**
  //  * Sets the value of `p` for the given `BGVContextBuilder` object.
  //  *
  //  * @param builder A unique pointer to a `BGVContextBuilder` object.
  //  * @param p The value of `p` to be set.
  //  * @return A unique pointer to the modified `BGVContextBuilder` object.
  //  */
  // std::unique_ptr<::seal::BGVContextBuilder> set_p(std::unique_ptr<::seal::BGVContextBuilder> builder, uint32_t p) {
  //   builder->p(p);  // Assume `p` modifies the object and is void.
  //   return builder; // Return the unique_ptr.
  // }

  // /**
  //  * Sets the value of r in the BGVContextBuilder.
  //  *
  //  * @param builder The BGVContextBuilder object.
  //  * @param r The value of r to be set.
  //  * @return A unique pointer to the updated BGVContextBuilder object.
  //  */
  // std::unique_ptr<::seal::BGVContextBuilder> set_r(std::unique_ptr<::seal::BGVContextBuilder> builder, uint32_t r) {
  //   builder->r(r);  // Assume `r` modifies the object and is void.
  //   return builder; // Return the unique_ptr.
  // }

  // /**
  //  * Sets the value of bits in the BGVContextBuilder.
  //  *
  //  * @param builder The BGVContextBuilder object.
  //  * @param bits The value of bits to be set.
  //  * @return A unique pointer to the updated BGVContextBuilder object.
  //  */
  // std::unique_ptr<::seal::BGVContextBuilder> set_bits(std::unique_ptr<::seal::BGVContextBuilder> builder, uint32_t bits) {
  //   builder->bits(bits);  // Assume `bits` modifies the object and is void.
  //   return builder; // Return the unique_ptr.
  // }

  // /**
  //  * Sets the value of c in the BGVContextBuilder.
  //  *
  //  * @param builder The BGVContextBuilder object.
  //  * @param c The value of c to be set.
  //  * @return A unique pointer to the updated BGVContextBuilder object.
  //  */
  // std::unique_ptr<::seal::BGVContextBuilder> set_c(std::unique_ptr<::seal::BGVContextBuilder> builder, uint32_t c) {
  //   builder->c(c);  // Assume `c` modifies the object and is void.
  //   return builder; // Return the unique_ptr.
  // }

  // /**
  //  * Sets the value of `gens` in the BGVContextBuilder.
  //  *
  //  * @param builder The BGVContextBuilder object.
  //  * @param gens The value of `gens` to be set.
  //  * @return The modified BGVContextBuilder object.
  //  */
  // BGVContextBuilder& set_gens(BGVContextBuilder& builder, const rust::cxxbridge1::Vec<long int>& gens) {
  //   auto gens_std_ptr = to_std_vector(gens); // This is now a std::unique_ptr<std::vector<long int>>
  //   builder.gens(*gens_std_ptr); // Dereference the std::unique_ptr to get the std::vector
  //   return builder;
  // }

  // /**
  //  * Sets the value of `ords` in the BGVContextBuilder.
  //  *
  //  * @param builder The BGVContextBuilder object.
  //  * @param ords The value of `ords` to be set.
  //  * @return The modified BGVContextBuilder object.
  //  */
  // BGVContextBuilder& set_ords(BGVContextBuilder& builder, const rust::cxxbridge1::Vec<long int>& ords) {
  //   auto ords_std_ptr = to_std_vector(ords); // This is now a std::unique_ptr<std::vector<long int>>
  //   builder.ords(*ords_std_ptr); // Dereference the std::unique_ptr to get the std::vector
  //   return builder;
  // }

  // /**
  //  * Sets the value of `mvec` in the BGVContextBuilder.
  //  *
  //  * @param builder The BGVContextBuilder object.
  //  * @param mvec The value of `mvec` to be set.
  //  * @return The modified BGVContextBuilder object.
  //  */
  // BGVContextBuilder& set_mvec(BGVContextBuilder& builder, const rust::cxxbridge1::Vec<long int>& mvec) {
  //   auto mvec_std_ptr = to_std_vector(mvec); // This is now a std::unique_ptr<std::vector<long int>>
  //   builder.mvec(*mvec_std_ptr); // Dereference the std::unique_ptr to get the std::vector
  //   return builder;
  // }

  // /**
  //  * Sets the value of `bootstrappable` in the BGVContextBuilder.
  //  *
  //  * @param builder The BGVContextBuilder object.
  //  * @param flag The value of `bootstrappable` to be set.
  //  * @return A unique pointer to the updated BGVContextBuilder object.
  //  */
  // std::unique_ptr<::seal::BGVContextBuilder> is_bootstrappable(std::unique_ptr<::seal::BGVContextBuilder> builder, bool flag) {
  //   builder->bootstrappable(flag);  // Assume `bootstrappable` modifies the object and is void.
  //   return builder; // Return the unique_ptr.
  // }

  // /**
  //  * Sets the value of `thickboot` in the BGVContextBuilder.
  //  *
  //  * @param builder The BGVContextBuilder object.
  //  * @return A unique pointer to the updated BGVContextBuilder object.
  //  */
  // std::unique_ptr<::seal::BGVContextBuilder> set_thickboot(std::unique_ptr<::seal::BGVContextBuilder> builder) {
  //   builder->thickboot();
  //   return builder;
  // }

  // /**
  //  * Sets the value of `thinboot` in the BGVContextBuilder.
  //  *
  //  * @param builder The BGVContextBuilder object.
  //  * @return A unique pointer to the updated BGVContextBuilder object.
  //  */
  // std::unique_ptr<::seal::BGVContextBuilder> set_thinboot(std::unique_ptr<::seal::BGVContextBuilder> builder) {
  //   builder->thinboot();
  //   return builder;
  // }

  // /**
  //  * Gets the value of `m` from the Context object.
  //  *
  //  * @param context The Context object.
  //  * @return The value of `m`.
  //  */
  // long get_m(const Context& context) { return context.getM(); }

  // // Define the C++ helper function for the FFI
  // void multiplyBy(std::unique_ptr<::seal::Ctxt>& ciphertext, std::unique_ptr<::seal::Ctxt>& other) {
  //   ciphertext->multiplyBy(*other);
  // }

}  // namespace seal
