// src/setup.rs

use crate::types::*;
use crate::utils::{a_creation, f_creation, x_creation, compute_u};
use crate::protocols::commit_all;

/// Performs the non-interactive setup for the protocol:
/// - generates the public parameter A
/// - commits to the initial polynomial f⁽⁰⁾ to obtain t₀ and the list of gadget vectors s₀…s_ℓ
/// - packages these into the initial witness w₀
/// - samples the full list of Xi matrices X₀…X_ℓ
/// - computes the first u₀ using X-list and f⁽⁰⁾
/// - returns both w₀ and x₀ as SetupOutput
pub fn setup() -> SetupOutput {
    // 1) Generate public parameter A (random matrix in Z_q^(N × (R*N*TAU)))
    let a = a_creation();

    // 2) Initial commitment on f⁽⁰⁾:
    //    - f0 ← random polynomial vector
    //    - commit_all produces t0 = commitment output,
    //      and s_list = [s₀, s₁, …, s_ℓ] gadget decompositions
    let f0 = f_creation();
    let (t0, s_list) = commit_all(f0.clone(), &a);

    // 3) Build initial witness w₀ = (s_list, f⁽⁰⁾)
    let w0 = WitnessData {
        s_list,
        f_vec: f0.clone(),
    };

    // 4) Sample the Xi matrices X₀…X_ℓ (each of dimension TAU × (R*TAU))
    let x_list = x_creation();

    // 5) Compute u₀ = compute_u(x_list, f⁽⁰⁾) = (I⊗X_ℓ)…(I⊗X₀) · f⁽⁰⁾
    let u0 = compute_u(&x_list, &f0);

    // 6) Package x₀ = (A, t₀, X_list, u₀)
    let x0 = XiData {
        a:      a.clone(),
        t_i:    t0.clone(),
        x_list: x_list.clone(),
        u_i:    u0,
    };

    SetupOutput { w0, x0 }
}