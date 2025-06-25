// src/types.rs

/// Finite field element type (working in ℤ/Qℤ)
pub type FieldElement = u64;
/// Vector of field elements
pub type Vector = Vec<FieldElement>;
/// Dense matrix of field elements
pub type Matrix = Vec<Vec<FieldElement>>;

/// Round index, from 0 to ELL
pub type Round = usize;
/// Transcript storing the sequence of fᵢ vectors
pub type Transcript = Vec<Vector>;

/// Witness data for round i:
/// - `s_list`: gadget decompositions s₀…s_{ℓ−i}
/// - `f_vec`: current polynomial vector f⁽ⁱ⁾
#[derive(Debug, Clone)]
pub struct WitnessData {
    pub s_list: Vec<Vector>,
    pub f_vec:  Vector,
}

/// Instance data for round i:
/// - `a`: public matrix A
/// - `t_i`: commitment t⁽ᶦ⁾
/// - `x_list`: matrices X₀…X_{ℓ−i−1}
/// - `u_i`: next message u⁽ᶦ⁾
#[derive(Debug, Clone)]
pub struct XiData {
    pub a:       Matrix,
    pub t_i:     Vector,
    pub x_list:  Vec<Matrix>,
    pub u_i:     Vector,
}

/// Prover’s response for Pᵢ:
/// - `y`: first gadget vector s₀^(i)
/// - `v`: transformed f⁽ⁱ⁾ under X-list
#[derive(Debug, Clone)]
pub struct ZiData {
    pub y: Vector,
    pub v: Vector,
}

/// Output of the initial setup:
/// - `w0`: initial witness (s-list and f⁽⁰⁾)
/// - `x0`: initial instance (A, t⁽⁰⁾, X-list, u⁽⁰⁾)
pub struct SetupOutput {
    pub w0: WitnessData,
    pub x0: XiData,
}

/// Binary challenge matrix of size (R·κ) × κ
pub type Challenge = Vec<Vec<u8>>;

// ————————————————————
// Protocol parameters 
// ————————————————————

/// Recursion depth ℓ
pub const ELL: usize   = 3;
/// Compression factor r
pub const R: usize     = 4;
/// Parameter κ
pub const KAPPA: usize = 4;
/// Lattice dimension n
pub const N: usize     = 3;
/// Gadget height τ
pub const TAU: usize   = 1;
/// Gadget width (log₂ q)
pub const ALPHA: usize = 10;
/// Infinity-norm bound β
pub const BETA: FieldElement = 2;
/// Field modulus q = 2ᵅ
pub const Q: FieldElement = 563;