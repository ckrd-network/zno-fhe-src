# Issues

- Make `function_properties_ref_t` and `std::shared_ptr` not const, in order to work around cxx - assumes google/autocxx#799 & dtolnay/cxx#850 are unresolved.

  - the error
  ```
  gen0.cxx:3218:93: error: cannot convert ‘const std::shared_ptr<const helib::EncryptedArray>& (helib::Context::*)() const’ to ‘const std::shared_ptr<helib::EncryptedArray>& (helib::Context::*)() const’ in initialization

  cargo:warning= 3218 |   ::std::shared_ptr<::helib::EncryptedArray> const &(::helib::Context::*shareEA$)() const = &::helib::Context::shareEA;

  cargo:warning=      |                                                                                             ^~~~~~~~~~~~~~~~~~~~~~~~~~
  ```
  - the generated code
    ```
    ::std::shared_ptr<::helib::EncryptedArray> const *helib$cxxbridge1$Context$shareEA(::helib::Context const &self) noexcept {
      ::std::shared_ptr<::helib::EncryptedArray> const &(::helib::Context::*shareEA$)() const = &::helib::Context::shareEA;
      return &(self.*shareEA$)();
    }
    ```
  - the source code

-
