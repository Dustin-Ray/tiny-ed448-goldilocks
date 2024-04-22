#![allow(non_snake_case)]
// ------------------------------
// TESTS
// ------------------------------
use crypto_bigint::U448;
use tiny_ed448_goldilocks::curve::{
    extended_edwards::ExtendedPoint,
    field::scalar::{Scalar, R_448},
};

#[test]
// 0 * G = 𝒪
pub fn zerog_id() {
    let p = ExtendedPoint::generator();
    let zero = Scalar::from(0_u64);
    let res = p * zero;
    let id = ExtendedPoint::id_point();

    assert!(res == id)
}

#[test]
// G * 1 = G
pub fn oneg_g() {
    let p = ExtendedPoint::generator();
    let one = Scalar::from(1_u64);
    let res = p * one;
    let id = ExtendedPoint::generator();

    assert!(res == id)
}

// G + (-G) = 𝒪
#[test]
fn gminusg_id() {
    let g = ExtendedPoint::generator();
    let neg_g = ExtendedPoint::generator().negate();
    let id = g.add(&neg_g);

    assert_eq!(id, ExtendedPoint::id_point());
}

#[test]
// 2 * G = G + G
pub fn twog_gplusg() {
    let g: ExtendedPoint = ExtendedPoint::generator();
    let two = Scalar::from(2_u64);
    let res = g * two;
    let res2 = g.add(&g);

    assert!(res == res2)
}

#[test]
// 4 * G = 2 * (2 * G)
fn fourg_twotwoG() {
    let four_g = ExtendedPoint::generator() * Scalar::from(4_u64);
    let two_times_two_g = (ExtendedPoint::generator().double()).double();

    assert!(four_g == two_times_two_g)
}

#[test]
//4 * G != 𝒪
fn fourg_not_id() {
    let four_g = ExtendedPoint::generator() * Scalar::from(4_u64);
    let tw_four_g = ExtendedPoint::generator() * Scalar::from(4_u64);
    let id = ExtendedPoint::id_point();

    assert!(!(four_g == id));
    assert!(!(tw_four_g == id))
}

#[test]
//r*G = 𝒪
fn rg_id() {
    let mut g = ExtendedPoint::generator();
    g = g * Scalar::from(U448::from_be_hex(R_448));
    let id = ExtendedPoint::id_point();

    assert!(g == id)
}

#[test]
// k * G = (k mod r) * G
fn kg_kmodrg() {
    // k * G
    let k = U448::MAX;
    let g = ExtendedPoint::generator();

    // (k mod r) * G
    let gk = g * (Scalar::from(k));
    let r = U448::from_be_hex(R_448);
    let k_mod_r = k.const_rem(&r);
    let mut k_mod_r_timesg = ExtendedPoint::generator();
    k_mod_r_timesg = k_mod_r_timesg * (Scalar::from(k_mod_r.0));

    assert!(gk == k_mod_r_timesg)
}

#[test]
// (k + 1)*G = (k*G) + G
fn k_plus_g() {
    let mut rng = rand::thread_rng();
    let k = rand::Rng::gen::<u64>(&mut rng);

    let k1_g = ExtendedPoint::generator() * Scalar::from::<u64>(k + 1);
    let k_g1 =
        (ExtendedPoint::generator() * Scalar::from::<u64>(k)).add(&ExtendedPoint::generator());

    assert!(k1_g == k_g1)
}

#[test]
//(k + t)*G = (k*G) + (t*G)
fn ktG_kgplustg() {
    let mut rng = rand::thread_rng();
    let k: u32 = rand::Rng::gen::<u32>(&mut rng);
    let t: u32 = rand::Rng::gen::<u32>(&mut rng);

    //(k + t)*G
    let k_plus_t_G = ExtendedPoint::generator() * (Scalar::from(k as u64 + t as u64));

    // (k*G) + (t*G)
    let kg_plus_tg = (ExtendedPoint::generator() * Scalar::from(k as u64))
        .add(&(ExtendedPoint::generator() * Scalar::from(t as u64)));

    assert!(k_plus_t_G == kg_plus_tg)
}

#[test]
//k*(t*G) = t*(k*G) = (k*t mod r)*G
fn ktG_tkG_ktmodrG() {
    let mut rng = rand::thread_rng();
    let k: u32 = rand::Rng::gen::<u32>(&mut rng);
    let t: u32 = rand::Rng::gen::<u32>(&mut rng);

    //k*(t*G)
    let mut ktg = ExtendedPoint::generator() * (Scalar::from(t as u64));
    ktg = ktg * (Scalar::from(k as u64));

    // t*(k*G)
    let mut tkg = ExtendedPoint::generator() * (Scalar::from(k as u64));
    tkg = tkg * (Scalar::from(t as u64));

    // (k*t mod r)*G
    let ktmodr = Scalar::from(k as u64) * (Scalar::from(t as u64));
    let kt_modr_g = ExtendedPoint::generator() * ktmodr;

    assert!(ktg == tkg);
    assert!(tkg == kt_modr_g);
    assert!(kt_modr_g == ktg);
}

// Ensure serde_json is added to your Cargo.toml for this

#[test]
fn extended_point_serde_round_trip() {
    let original_point = ExtendedPoint::generator();
    let serialized_point =
        serde_json::to_string(&original_point).expect("Failed to serialize ExtendedPoint");
    let deserialized_point: ExtendedPoint =
        serde_json::from_str(&serialized_point).expect("Failed to deserialize ExtendedPoint");
    assert_eq!(
        original_point, deserialized_point,
        "Deserialized ExtendedPoint does not match the original"
    );
}
