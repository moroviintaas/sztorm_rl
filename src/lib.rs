//! # AmfiRL
//! Framework library for reinforcement learning using data model from [`amfi`](amfi_core).
//! Crate contains traits and generic implementation.
//! ## Examples
//! Examples are presented in separate crate - [`amfi_examples`](https://github.com/moroviintaas/amfi_examples)
//! ## Licence: MIT



/// Neural network trait and structs built on [`tch`] crate
pub mod torch_net;
/// Module with traits dedicated to representation of game data as tensors
pub mod tensor_data;
/// Error types defined in this crate
pub mod error;
/// Module dedicated to agents learning policies
pub mod agent;
/// Minimal demonstration game elements to provide very simple examples
pub mod demo;
/// Module dedicated to learning policies
pub mod policy;


