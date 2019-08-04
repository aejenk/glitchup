warning: unused manifest key: package.maintenance
   Compiling glitchup v0.2.0 (C:\Users\Textman\Desktop\BendeRS)
warning: method is never used: `setup_config`
   --> src\benders.rs:153:5
    |
153 |     fn setup_config(&mut self) {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: #[warn(dead_code)] on by default

warning: function is never used: `verify_num_option`
   --> src\benders.rs:339:9
    |
339 |         fn verify_num_option(v : &Vec<isize>, name: &str, location: &str) -> Vec<isize> {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: structure field `VoidConfig` should have a snake case name
  --> src\benders.rs:48:5
   |
48 |     VoidConfig: Option<VoidConfig>,
   |     ^^^^^^^^^^ help: convert the identifier to snake case: `void_config`
   |
   = note: #[warn(non_snake_case)] on by default

warning: structure field `ChaosConfig` should have a snake case name
  --> src\benders.rs:49:5
   |
49 |     ChaosConfig: Option<ChaosConfig>,
   |     ^^^^^^^^^^^ help: convert the identifier to snake case: `chaos_config`

warning: structure field `LoopConfig` should have a snake case name
  --> src\benders.rs:50:5
   |
50 |     LoopConfig: Option<LoopConfig>,
   |     ^^^^^^^^^^ help: convert the identifier to snake case: `loop_config`

warning: structure field `ReverseConfig` should have a snake case name
  --> src\benders.rs:51:5
   |
51 |     ReverseConfig: Option<ReverseConfig>,
   |     ^^^^^^^^^^^^^ help: convert the identifier to snake case: `reverse_config`

warning: structure field `ShiftConfig` should have a snake case name
  --> src\benders.rs:52:5
   |
52 |     ShiftConfig: Option<ShiftConfig>,
   |     ^^^^^^^^^^^ help: convert the identifier to snake case: `shift_config`

warning: structure field `ShuffleConfig` should have a snake case name
  --> src\benders.rs:53:5
   |
53 |     ShuffleConfig: Option<ShuffleConfig>,
   |     ^^^^^^^^^^^^^ help: convert the identifier to snake case: `shuffle_config`

warning: structure field `SwapConfig` should have a snake case name
  --> src\benders.rs:54:5
   |
54 |     SwapConfig: Option<SwapConfig>,
   |     ^^^^^^^^^^ help: convert the identifier to snake case: `swap_config`

warning: structure field `IncreaseConfig` should have a snake case name
  --> src\benders.rs:55:5
   |
55 |     IncreaseConfig: Option<IncreaseConfig>,
   |     ^^^^^^^^^^^^^^ help: convert the identifier to snake case: `increase_config`

warning: structure field `GradientConfig` should have a snake case name
  --> src\benders.rs:56:5
   |
56 |     GradientConfig: Option<GradientConfig>
   |     ^^^^^^^^^^^^^^ help: convert the identifier to snake case: `gradient_config`

    Finished dev [unoptimized + debuginfo] target(s) in 3.62s
