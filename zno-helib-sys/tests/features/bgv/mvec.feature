Feature: Robust Configuration of Encryption Parameters in HElib via Rust Interface
  To ensure secure and efficient encryption operations,
  As a developer interfacing with HElib in a Rust environment,
  I need to accurately configure encryption parameters, focusing on the mvecs setting.

  Background:
    Given HElib is initialized with Rust language bindings
    And the BGV encryption scheme is employed

  @validConfiguration
  Scenario Outline: Setting a valid mvecs parameter
    When I configure the mvecs parameter to <valid_array>
    Then the encryption context initializes correctly
    And the mvecs setting matches <valid_array>

    Examples:
      | valid_array   |
      | [1, 2, 3]     |
      | [4, 5, 6]     |
      | [10, 20, 30]  |

  @errorHandling
  Scenario: Setting an invalid mvecs parameter
    When I configure mvecs with "invalid"
    Then an error "Invalid mvecs: Expected integer array" is reported
    And the underlying system remains stable

  @performance
  Scenario Outline: Evaluating performance with different mvecs configurations
    When I configure mvecs with <performance_array>
    Then efficiency improvements are observed
    And performance details are reported

    Examples:
      | performance_array |
      | [5, 7, 11]        |
      | [2, 3, 5, 7]      |
      | [1, 1, 2, 3, 5]   |

  @defaultSetting
  Scenario: Using default mvecs settings
    When mvecs is not specified in configuration
    Then default settings [1, 2, 3, 4] are used

  @errorHandling
  Scenario: Setting mvecs with an excessively large array
    When I configure mvecs with a large array [100, ..., 1000]
    Then an error "Array size exceeds limit" is reported

  @edgeCases
  Scenario Outline: Handling various mvecs edge cases
    When I configure mvecs with <input>
    Then <outcome> is reported

    Examples:
      | input                     | outcome                                 |
      | [2147483647, -2147483648] | "Valid array processed"                 |
      | [[1, 2], [3, 4]]          | "Nested arrays not allowed"             |
      | [1, "two", 3]             | "Inconsistent array types"              |
      | [5, 7, 11] (interrupted)  | "Interruption managed, state consistent"|
      | [4, 5, 6] (reconfigured)  | "Reconfiguration successful"            |

# Note:
# These scenarios test the resilience and adaptability of the Rust-HElib interface and the library's handling of various edge cases.
