use crate::types::*;
use crate::utils::challenge;
use crate::protocols::{pi, next_wi, next_xi};

/// The Prover orchestrates all rounds P₀…P_{ℓ−1} and emits z₀…zₗ.
pub struct Prover;

impl Prover {
    /// # Inputs
    /// - `x0`: initial public instance (A, t⁽⁰⁾, X-list, u⁽⁰⁾)
    /// - `w0`: initial witness (s-list, f⁽⁰⁾)
    /// - `seed`: seed bytes for challenge derivation
    ///
    /// # Output
    /// A Vec<ZiData> = [z₀, z₁, …, zₗ], where zₗ is just the final f-vector.
    pub fn prove(
        x0: &XiData,
        w0: &WitnessData,
        seed: &[u8],
    ) -> Vec<ZiData> {
        let mut zs = Vec::with_capacity(ELL + 1);
        let mut w_i: WitnessData = w0.clone();
        let mut x_i: XiData      = x0.clone();
        let mut transcript: Transcript = Vec::new();

        for i in 0..ELL {
            println!("=== Prover round {} ===", i);
            let z_i = pi(i, &w_i, &x_i);
            zs.push(z_i.clone());

            // Append to transcript
            transcript.push(z_i.y.clone());
            transcript.push(z_i.v.clone());

            // Derive next challenge
            let c_ip1 = challenge(&transcript, seed);

            // Advance to next round
            w_i = next_wi(&w_i, &c_ip1, ELL - i);
            x_i = next_xi(&x_i, &z_i, &transcript, seed);
        }

        // Final output z_ℓ = last f_vec
        println!("=== Prover final round ℓ ===");
        zs.push(ZiData { y: Vec::new(), v: w_i.f_vec.clone() });
        zs
    }
}