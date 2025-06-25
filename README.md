# Recursive-Friendly Zero-Knowledge Proofs in Rust

This project is a Rust implementation of a recursion-friendly zero-knowledge proof system based on lattice-based polynomial commitments. It follows the "Basic Construction" presented in Section 5 of the paper:

**Cini, Malavolta, Nguyen, Wee**  
*Polynomial Commitments from Lattices: Post-Quantum Security, Fast Verification and Transparent Setup*  
[Cryptology ePrint Archive: 2024/281](https://eprint.iacr.org/2024/281)

---

## Project Description

We implement a proof system that proves knowledge of a committed vector $\mathbf{f} \in \mathbb{Z}_q^{r^{\ell+1} \kappa n \tau}$ such that:

$$
\left( I_{\kappa n} \otimes X_\ell \right)
\cdots
\left( I_{r^\ell \kappa n} \otimes X_0 \right) \cdot \mathbf{f} = \mathbf{u} \mod q
$$

The protocol is structured recursively and transformed into a non-interactive zero-knowledge proof (NIZK) using the Fiat–Shamir heuristic.

---

## Code Structure

- `main.rs` – Entry point: runs setup, proof generation, and verification.
- `protocols.rs` – Core logic for prover/verifier rounds and instance/witness update.
- `prover.rs` – Implements the non-interactive prover (using SHA256).
- `verifier.rs` – Reconstructs and verifies the full proof.
- `setup.rs` – Commitment scheme and generation of public parameters.
- `types.rs` – Core data structures: vectors, matrices, witnesses, instances.
- `utils.rs` – Math utilities: Kronecker product, SHA256 hashing, matrix ops.

Project metadata:

- `Cargo.toml` – Defines dependencies and build configuration.
- `Cargo.lock` – Version-locked dependency list (auto-generated).

---

## Protocol Parameters

| Name     | Description                          | Value |
|----------|--------------------------------------|-------|
| $\ell$   | Recursion depth                      | 3     |
| $r$      | Folding factor                       | 4     |
| $\kappa$ | Statistical width parameter          | 4     |
| $n$      | A Matrix row count                   | 3     |
| $\tau$   | Gadget height                        | 1     |
| $\alpha$ | Gadget width                         | 10    |
| $q$      |Prime Field modulus                   | 563   |
| $\beta$  | Infinity norm bound for commitments  | 2     |

These settings are small and pedagogical. Real-world deployments require larger values (e.g. $\alpha \approx 60$).

---

## Protocol Outline

The protocol defines a recursive relation $R_{\ell, \beta_\ell}$ over:

- Instance: $x = (A, \mathbf{t}, (X_j)_{j=0}^{\ell}, \mathbf{u})$
- Witness: $w = ((s_j)_{j=0}^{\ell}, \mathbf{f})$

Each recursive level $i$ verifies:

- A linear commitment to $s_0^{(i)}$
- Matrix consistency: $v_i$ transforms to $u^{(i)}$
- Norm bound: ‖s₀^(i)‖∞ ≤ β_{ℓ−i}

The prover and verifier algorithms $(P_i, V_i)$ are recursively composed, and the protocol is made non-interactive via Fiat–Shamir:

$$
c_{i+1} := \text{SHA256}(\mathbf{t} \|\|\ \text{transcript})
$$

---

## How to Run

Ensure you have Rust installed, then:

```bash
git clone https://github.com/matelatn/semester_project_proof_system.git
cd semester_project_proof_system
cargo run
```
This runs the full flow: setup, commit, prove, and verify.
## Performance Notes

This implementation is intended for **educational and experimental purposes only**. It is not optimized for large-scale use. Performance bottlenecks include costly Kronecker products, large matrix operations, and high memory usage. Adapting it to real-world cryptographic settings would require efficient data structures, optimized algebra, and larger, practical parameters.

## Disclaimer

This code is a proof-of-concept designed in the context of a university project. For full details and formal proofs, please refer to the original paper.

## Author

**Nathan Matelat**  
*SPRING 2025 Semester Project @ EURECOM*  
*Supervised by Antonio Faonio*  
Contact: [nathan.matelat@eurecom.fr](mailto:nathan.matelat@eurecom.fr)
