// src/main.rs
mod setup;
mod prover;
mod verifier;
mod types;
mod utils;
mod protocols;

use types::{SetupOutput, XiData, WitnessData, ZiData, Transcript};
use utils::challenge;

fn main() {
    // Entry point: orchestrates setup, proving, and verification

    // ── 1) Initial setup (non-interactive) ───────────────────────────────────
    // This generates:
    //   • w0: the initial witness (s-vectors and f⁽⁰⁾)
    //   • x0: the initial instance (public matrix A, commitment t⁽⁰⁾,
    //         challenge matrices X_list, and u⁽⁰⁾)
    let SetupOutput { w0, x0 } = setup::setup();

    // Build the seed for challenges from t⁽⁰⁾, encoding each FieldElement in little-endian bytes
    let seed: Vec<u8> = x0
        .t_i
        .iter()
        .flat_map(|&x| x.to_le_bytes().to_vec())
        .collect();

    // ── 2) Prover: generate all zᵢ messages ───────────────────────────────────
    // The prover consumes x0 and w0, along with the seed, and returns
    // a Vec<ZiData> containing z₀, z₁, …, z_ℓ.
    let zs: Vec<ZiData> = prover::Prover::prove(&x0, &w0, &seed);

    // ── 3) Verifier: consumes x₀ and the prover’s zᵢ messages ──────────────────
    // Returns true if all rounds’ checks Vᵢ pass, false otherwise.
    let ok = verifier::Verifier::verify(&x0, &zs, &seed);

    println!("✅ Verification passed = {}", ok);
}