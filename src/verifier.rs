use crate::types::*;
use crate::utils::challenge;
use crate::protocols::{v_i, next_xi};

/// The Verifier checks V₀..V_{ℓ−1} on the prover’s zᵢ messages.
pub struct Verifier;

impl Verifier {
    /// # Inputs
    /// - `x0`: initial public instance
    /// - `zs`: prover messages [z₀..zₗ]
    /// - `seed`: same seed used by prover
    ///
    /// # Output
    /// `true` if all rounds pass, else `false`.
    pub fn verify(
        x0: &XiData,
        zs: &[ZiData],
        seed: &[u8],
    ) -> bool {
        let mut transcript: Transcript = Vec::new();
        let mut x_i = x0.clone();

        for i in 0..ELL {
            println!("=== Verifier round {} ===", i);

            // 1) Check Vᵢ
            if !v_i(i, &x_i, &zs[i]) {
                println!("❌ Verification failed at round {}", i);
                return false;
            }

            // 2) Append to transcript
            transcript.push(zs[i].y.clone());
            transcript.push(zs[i].v.clone());

            // 3) Compute next instance
            let c_ip1 = challenge(&transcript, seed);
            x_i = next_xi(&x_i, &zs[i], &transcript, &c_ip1);
        }

        println!("✅ All verification rounds passed");
        true
    }
}