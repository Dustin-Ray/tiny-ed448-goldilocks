# Ed448-Goldilocks


<p align="center">
  <img src="./img.webp" width="250" height="250">
</p>

> WARNING: This software has not been audited. Use at your own risk.

# 1. Objective:

The Goldilocks variant of curves in Edward's form present a compelling balance of security and performance. We wish to leverage this curve to satisfy that the following group properties hold:


$0 * G = 𝒪$

$G * 1 = G$

$G + (-G) = 𝒪$

$2 * G = G + G$

$4 * G > 𝒪$

$r * G = 𝒪$

$(k + 1) * G =  (k * G) + G$

$k*G = (k % r) * G$

$(k + t) * G = (k * G) + (t * G)$

$k * (t * G) = t * (k * G) = (k * t % r) * G$

$4 * G = 2 * (2 * G)$


## What we want:
  - The fastest possible composition and doubling operations
  - Fixed-time side-channel resistance for scalar and field operations
  - Only the functionality that we need for Schnorr signatures and asymmetric DH, ideally making this implementation as lean as possible.

# 2. Strategy:

Largely following the approaches of [this](https://github.com/crate-crypto/Ed448-Goldilocks) and [this](https://docs.rs/curve25519-dalek/4.1.1/curve25519_dalek/), we perform the following series of transformations during a point / scalar multiplication:

1. Start in twisted form
2. Decompose the scalar and recenter to radix 16 between -8 and 8
3. Create a lookup table of multiples of P mapped to radix 16 digits, with fixed-time lookups to ensure side-channel resistance.
4. In variable_base_mul, we perform the doublings in twisted form, and the additions and fixed-time conditional negation in projective niels form.
5. The point is returned in extended form, and finally converted to affine form for user-facing operations.

At a higher level, we have:

$s \cdot P = \text{Affine} \rightarrow \text{Extended} \rightarrow \text{Twisted} \rightarrow \text{Projective Niels} \rightarrow \text{Twisted} \rightarrow \text{Extended} \rightarrow \text{Affine}$

# 3. Fixed-Time

The lookup table for the decomposed scalar is computed and traversed in fixed-time:

```rust
/// Selects a projective niels point from a lookup table in fixed-time
pub fn select(&self, index: u32) -> ProjectiveNielsPoint {
    let mut result = ProjectiveNielsPoint::id_point();
    for i in 1..9 {
        let swap = index.ct_eq(&(i as u32));
        result.conditional_assign(&self.0[i - 1], swap);
    }
    result
}
```
This ensures fixed-time multiplication without the need for a curve point in Montgomery form. Further, we make use of the [crypto-bigint](https://github.com/RustCrypto/crypto-bigint) library which ensures fixed-time operations for our Scalar type. Field elements are represented by the fiat-crypto [p448-solinas-64](https://github.com/mit-plv/fiat-crypto/blob/master/fiat-rust/src/p448_solinas_64.rs) prime field. It is formally verified and heavily optimized at the machine-level.

The following test:

```rust
#[test]
fn test_sig_timing_side_channel() {
    for i in 0..10 {
        let mut msg = Message::new(get_random_bytes(5242880));
        let pw = get_random_bytes(16 << i);
        let mut key_pair = KeyPair::new(&pw, "test key".to_string(), 512);

        let now = Instant::now();
        msg.sign(&mut key_pair, 512);
        println!("{} needed {} microseconds", i, now.elapsed().as_micros());
        msg.verify(&key_pair.pub_key);
        assert!(msg.op_result.unwrap());
    }
}
```

Displays fixed-time execution over a range of key sizes:
```
running 1 test
0 needed 46608 microseconds
1 needed 43783 microseconds
2 needed 44706 microseconds
3 needed 45827 microseconds
4 needed 45121 microseconds
5 needed 44641 microseconds
6 needed 44427 microseconds
7 needed 44661 microseconds
8 needed 44431 microseconds
9 needed 44638 microseconds
```

This is by no means a complete assessment of the security of this implementation, but it's a good signal that the fixed-time backend is doing what we expect. Nothing about the size of the secret key is being revealed by runtime here

# 4. Benchmarks

Run with:
```bash
cargo bench
```

Approximate runtimes for Intel® Core™ i7-10710U × 12

| Operation   | ~Time (ms)  |
|------------|------------|
| Encrypt| 75 |
| Decrypt| 75 |
| Sign| 42 |
| Verify| 18 |

# 5. Signatures and DH:

Using this crate as the elliptic-curve backend for [capyCRYPT](https://github.com/drcapybara/capyCRYPT), we have:

### Asymmetric Encrypt/Decrypt:
```rust
use capycrypt::{
    KeyEncryptable,
    KeyPair,
    Message,
    sha3::aux_functions::byte_utils::get_random_bytes
};

// Get 5mb random data
let mut msg = Message::new(get_random_bytes(5242880));
// Create a new private/public keypair
let key_pair = KeyPair::new(&get_random_bytes(32), "test key".to_string(), 512);

// Encrypt the message
msg.key_encrypt(&key_pair.pub_key, 512);
// Decrypt the message
msg.key_decrypt(&key_pair.priv_key);
// Verify
assert!(msg.op_result.unwrap());
```

### Schnorr Signatures:
```rust
use capycrypt::{
    Signable,
    KeyPair,
    Message,
    sha3::aux_functions::byte_utils::get_random_bytes,
};
// Get random 5mb
let mut msg = Message::new(get_random_bytes(5242880));
// Get a random password
let pw = get_random_bytes(64);
// Generate a signing keypair
let key_pair = KeyPair::new(&pw, "test key".to_string(), 512);
// Sign with 256 bits of security
msg.sign(&key_pair, 512);
// Verify signature
msg.verify(&key_pair.pub_key);
// Assert correctness
assert!(msg.op_result.unwrap());
```

## Acknowledgements

The authors wish to sincerely thank Dr. Paulo Barreto for the general design of this library as well as the curve functionality. We also wish to extend gratitude to the curve-dalek authors [here](https://github.com/crate-crypto/Ed448-Goldilocks) and [here](https://docs.rs/curve25519-dalek/4.1.1/curve25519_dalek/) for the excellent reference implementations and exemplary instances of rock-solid cryptography. Thanks to [otsmr](https://github.com/otsmr) for the callout on the original attempt at an affine-coordinate Montgomery ladder.