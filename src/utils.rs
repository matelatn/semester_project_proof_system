// src/utils.rs

use rand::Rng;
use sha2::{Digest, Sha256};
use crate::types::{R, KAPPA, N, TAU, ALPHA, Q, Vector, Matrix, ELL, Transcript, Challenge};

////////////////////////////////////////////////////////////////////////////////
// Linear algebra primitives (all arithmetic mod Q where applicable)
////////////////////////////////////////////////////////////////////////////////

/// Transpose a matrix
pub fn transpose<T: Clone>(m: &[Vec<T>]) -> Vec<Vec<T>> {
    let rows = m.len();
    let cols = m[0].len();
    let mut t = vec![vec![m[0][0].clone(); rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            t[j][i] = m[i][j].clone();
        }
    }
    t
}

/// Identity matrix of size dim×dim
pub fn identity(dim: usize) -> Matrix {
    let mut m = vec![vec![0; dim]; dim];
    for i in 0..dim { m[i][i] = 1; }
    m
}

/// Naïve Kronecker product A⊗B (mod Q)
pub fn kron(a: &Matrix, b: &Matrix) -> Matrix {
    let (ar, ac) = (a.len(), a[0].len());
    let (br, bc) = (b.len(), b[0].len());
    let mut out = vec![vec![0; ac * bc]; ar * br];
    for i in 0..ar {
        for j in 0..ac {
            for k in 0..br {
                for l in 0..bc {
                    out[i * br + k][j * bc + l] = (a[i][j] * b[k][l]) % Q;
                }
            }
        }
    }
    out
}

/// Matrix–vector multiplication (mod Q)
pub fn mat_vec(m: &Matrix, v: &Vector) -> Vector {
    let rows = m.len();
    let cols = m[0].len();
    assert_eq!(v.len(), cols, "matrix cols != vector len");
    (0..rows).map(|i| {
        let mut acc = 0;
        for j in 0..cols {
            acc = (acc + m[i][j] * v[j]) % Q;
        }
        acc
    }).collect()
}


////////////////////////////////////////////////////////////////////////////////
// Random generation and gadget routines
////////////////////////////////////////////////////////////////////////////////

/// Generate a random f of dimension R^(ELL+1)·KAPPA·N·TAU
pub fn f_creation() -> Vector {
    let dim = R.pow((ELL + 1) as u32) * KAPPA * N * TAU;
    let mut rng = rand::thread_rng();
    (0..dim).map(|_| rng.gen_range(0..Q)).collect()
}

/// Gadget inverse G⁻¹: bit-decompose each entry in v into ALPHA bits (MSB→LSB)
pub fn gadget_inverse(v: &Vector) -> Vector {
    let mut out = Vec::with_capacity(v.len() * ALPHA);
    for &x in v {
        for bit in (0..ALPHA).rev() {
            out.push((x >> bit) & 1);
        }
    }
    out
}

/// Build gadget matrix G = I_blocks ⊗ [2^(α−1),…,2^0]
pub fn gadget_matrix(r: usize, kappa: usize, n: usize, tau: usize, alpha: usize) -> Matrix {
    let blocks = r * kappa * n * tau;
    // weights row = [2^(α−1), …, 2^0]
    let mut weights = vec![vec![0; alpha]; 1];
    for (i, w) in weights[0].iter_mut().enumerate() {
        *w = 1 << (alpha - 1 - i);
    }
    let eye = identity(blocks);
    kron(&eye, &weights)
}

/// Generate random public matrix A ∈ ℤ_Q^(N × (R·N·ALPHA))
pub fn a_creation() -> Matrix {
    let cols = R * N * ALPHA;
    let mut rng = rand::thread_rng();
    (0..N)
        .map(|_| (0..cols).map(|_| rng.gen_range(0..Q)).collect())
        .collect()
}

/// Generate the X_j matrices: Vec of length ELL+1, each TAU×(R·TAU)
pub fn x_creation() -> Vec<Matrix> {
    let mut rng = rand::thread_rng();
    (0..=ELL)
        .map(|_| {
            (0..TAU)
                .map(|_| (0..(R * TAU)).map(|_| rng.gen_range(0..Q)).collect())
                .collect()
        })
        .collect()
}

/// Compute u₀ = (∏_{j=0..ELL} (I_{r^(ELL−j)·KAPPA·N·TAU} ⊗ X_j)) · f₀  (mod Q)
pub fn compute_u(x_list: &[Matrix], f: &Vector) -> Vector {
    let ell = x_list.len().saturating_sub(1);
    let m = KAPPA * N * TAU;
    let mut u = f.iter().map(|&x| x % Q).collect::<Vector>();
    for (j, xj) in x_list.iter().enumerate() {
        let power = ell.saturating_sub(j);
        let idim = R.pow(power as u32) * m;
        let i_mat = identity(idim);
        let m_kron = kron(&i_mat, xj);
        u = mat_vec(&m_kron, &u);
    }
    u
}

////////////////////////////////////////////////////////////////////////////////
// Hash-based challenge generation
////////////////////////////////////////////////////////////////////////////////

/// Expand a hash digest into at least `needed` bits
pub fn expand_to_bits(seed: &[u8], needed: usize) -> Vec<u8> {
    let mut bits = Vec::with_capacity(needed);
    let mut ctr = 0u8;
    let mut cur = seed.to_vec();
    while bits.len() < needed {
        let mut h = Sha256::new();
        h.update(&cur);
        h.update(&[ctr]);
        let dg = h.finalize();
        for &byte in dg.iter() {
            for b in (0..8).rev() {
                bits.push((byte >> b) & 1);
                if bits.len() == needed {
                    return bits;
                }
            }
        }
        ctr = ctr.wrapping_add(1);
        cur = dg.to_vec();
    }
    bits
}

/// Build an (R·κ)×κ binary challenge matrix from the transcript and seed
pub fn challenge(transcript: &Transcript, seed: &[u8]) -> Challenge {
    let rows = R * KAPPA;
    let cols = KAPPA;
    let needed = rows * cols;
    let mut h = Sha256::new();
    h.update(seed);
    for vec_u in transcript {
        for &x in vec_u {
            h.update(&x.to_le_bytes());
        }
    }
    let dg = h.finalize();
    let bits = expand_to_bits(&dg, needed);
    let mut m: Challenge = vec![vec![0u8; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            m[i][j] = bits[i * cols + j];
        }
    }
    m
}