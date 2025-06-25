use crate::types::*;
use crate::utils::*;

/// Prover’s Pᵢ step:
///
/// For round `i`, given witness data `w` and instance data `x`,
/// returns `ZiData { y, v }` where:
/// - `y` = s₀^(i) (the first vector in `w.s_list`)
/// - `v` = (I⊗X_{ℓ−i−1})⋯(I⊗X₀) · f⁽ⁱ⁾ (vector in ℤ_Q)
pub fn pi(i: usize, w: &WitnessData, x: &XiData) -> ZiData {
    println!("--- P_{} starting ---", i);
    // 1) y_i = s₀^(i)
    let y = w.s_list[0].clone();
    println!("  y_{} len={} = {:?}", i, y.len(), y);

    // 2) Start from f⁽ⁱ⁾
    let mut v = w.f_vec.clone();
    println!("  f^( {}) len={} = {:?}", i, v.len(), v);

    // 3) Sequentially apply M_j = I_{r^(ℓ−i−j)·κ·n·τ} ⊗ X_j
    let depth = ELL - i;
    for j in 0..depth {
        let block_pow = depth - j;
        let block_dim = R.pow(block_pow as u32) * KAPPA * N * TAU;
        let I = identity(block_dim);
        let Xj = &x.x_list[j];
        let M = kron(&I, Xj);
        v = mat_vec(&M, &v);
        println!(
            "    after X_{} application: v len={} first few={:?}",
            j,
            v.len(),
            &v[..std::cmp::min(5, v.len())]
        );
    }

    println!("--- P_{} done → v len={} ---", i, v.len());
    ZiData { y, v }
}

/// Verifier’s Pᵢ‐check Vᵢ:
/// 1) (I⊗A)·y ≡ t_i  
/// 2) (I⊗X_{ℓ−i−1})·v ≡ u_i  
/// 3) ∥y∥∞ ≤ (R·κ)^i · BETA  
pub fn v_i(
    i: Round,
    x_i: &XiData,
    z_i: &ZiData,
) -> bool {
    println!("--- V_{} starting ---", i);
    let XiData { a, t_i, x_list, u_i } = x_i;
    let ZiData { y, v } = z_i;

    // 1)
    let id_k_tau = identity(KAPPA * TAU);
    let m0 = kron(&id_k_tau, a);
    let lhs0 = mat_vec(&m0, y);
    let b0 = &lhs0 == t_i;
    println!(
        "  Check0: (I⊗A)·y len={} → {:?}\n          t_i        → {:?}\n          pass={}",
        lhs0.len(),
        &lhs0[..std::cmp::min(5, lhs0.len())],
        &t_i[..std::cmp::min(5, t_i.len())],
        b0
    );

    // 2)
    let X_li = &x_list[ELL - i];
    let id_k_n = identity(KAPPA * N);
    let m1 = kron(&id_k_n, X_li);
    let lhs1 = mat_vec(&m1, v);
    let b1 = &lhs1 == u_i;
    println!(
        "  Check1: (I⊗X)·v len={} → {:?}\n          u_i        → {:?}\n          pass={}",
        lhs1.len(),
        &lhs1[..std::cmp::min(5, lhs1.len())],
        &u_i[..std::cmp::min(5, u_i.len())],
        b1
    );

    // 3)
    let inf_norm = y.iter().cloned().max().unwrap_or(0);
    let scale = (R * KAPPA).pow(i as u32) as FieldElement;
    let bound = scale.saturating_mul(BETA);
    let b2 = inf_norm <= bound;
    println!(
        "  Check2: ∥y∥∞={}  bound={}  pass={}",
        inf_norm, bound, b2
    );

    let passed = b0 && b1 && b2;
    println!("--- V_{} result = {} ---", i, passed);
    passed
}

/// NextWᵢ step: compute updated witness for round i+1
pub fn next_wi(
    w_i: &WitnessData,
    c: &Challenge,
    level: Round,
) -> WitnessData {
    println!("--- NextW step (level {}) ---", level);
    let f_i = &w_i.f_vec;
    let c_t: Matrix = transpose(c)
        .into_iter()
        .map(|r| r.into_iter().map(|b| b as FieldElement).collect())
        .collect();

    // new s_list
    let old_s = &w_i.s_list;
    let rounds = old_s.len() - 1;
    let mut new_s_list = Vec::with_capacity(rounds);
    for j in 0..rounds {
        let blk = R.pow((j + 1) as u32) * N * TAU * ALPHA;
        let eye = identity(blk);
        let kron_mat = kron(&c_t, &eye);
        let s_next = mat_vec(&kron_mat, &old_s[j + 1]);
        println!(
            "  new s_{} len={} first few={:?}",
            j,
            s_next.len(),
            &s_next[..std::cmp::min(5, s_next.len())]
        );
        new_s_list.push(s_next);
    }

    // new f
    let blk_f = R.pow(level as u32) * N * TAU;
    let eye_f = identity(blk_f);
    let kron_f = kron(&c_t, &eye_f);
    let f_next = mat_vec(&kron_f, f_i);
    println!(
        "  new f len={} first few={:?}",
        f_next.len(),
        &f_next[..std::cmp::min(5, f_next.len())]
    );

    WitnessData {
        s_list: new_s_list,
        f_vec: f_next,
    }
}

/// NextXᵢ step: compute updated instance for round i+1
pub fn next_xi(
    xi: &XiData,
    zi: &ZiData,
    transcript: &Transcript,
    c: &Challenge,
) -> XiData {
    println!("--- NextX step ---");
    
    let c_t: Matrix = transpose(&c)
        .into_iter()
        .map(|r| r.into_iter().map(|b| b as FieldElement).collect())
        .collect();
    let eye = identity(N * TAU);
    let m = kron(&c_t, &eye);

    let u_next = mat_vec(&m, &zi.v);
    println!(
        "  u next len={} first few={:?}",
        u_next.len(),
        &u_next[..std::cmp::min(5, u_next.len())]
    );

    let gmat = gadget_matrix(R, KAPPA, N, TAU, ALPHA);
    let packed = mat_vec(&gmat, &zi.y);
    println!(
        "  packed y len={} first few={:?}",
        packed.len(),
        &packed[..std::cmp::min(5, packed.len())]
    );

    let t_next = mat_vec(&m, &packed);
    println!(
        "  t next len={} first few={:?}",
        t_next.len(),
        &t_next[..std::cmp::min(5, t_next.len())]
    );

    let mut x_next = xi.x_list.clone();
    x_next.pop();
    println!("  remaining X_j count = {}", x_next.len());

    XiData {
        a: xi.a.clone(),
        t_i: t_next,
        x_list: x_next,
        u_i: u_next,
    }
}

/// commit_all: initial non‐interactive commitment
pub fn commit_all(f_vec: Vector, a: &Matrix) -> (Vector, Vec<Vector>) {
    println!("--- commit_all starting ---");
    let mut s_list = Vec::with_capacity(ELL + 1);
    let mut current = f_vec;
    for j in (0..=ELL).rev() {
        let s_j = gadget_inverse(&current);
        println!(
            "  s_{} len={} first few={:?}",
            j,
            s_j.len(),
            &s_j[..std::cmp::min(5, s_j.len())]
        );
        s_list.push(s_j.clone());

        let dim_i = R.pow(j as u32) * KAPPA * TAU;
        let big = kron(&identity(dim_i), a);
        current = mat_vec(&big, &s_j);
        println!(
            "  current after j={} len={} first few={:?}",
            j,
            current.len(),
            &current[..std::cmp::min(5, current.len())]
        );
    }
    s_list.reverse();
    println!("--- commit_all done → t len={} ---", current.len());
    (current, s_list)
}