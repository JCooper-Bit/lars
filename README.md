# 🧮 Lars — Learning Algebra in Rust!

**Lars** (Learning Algebra in Rust) is a learning-focused Rust crate that provides simple yet powerful linear algebra utilities built from first principles by a student currently going through Linear Algebra classes.

The goal of this project is to **implement linear algebra concepts** (vectors, matrices, transformations) from scratch in Rust, to deeply understand both the **math** and **systems-level programming** behind them.

--- 
## 📚Articles/tutorials
Alongside developing the crate, I am also writing a series of guides to allow other people to follow along and develop their own linear algebra functionality through a series of short tasks. these tasks are designed with modularity in mind, so if for example you only require a 2D vector struct, you can just do **those** tasks.

As well as mathematical and conceptual explanations, each task has a corresponding solution with full code examples.

The guides are written with rust in mind, but it would not be hard to adapt them to any language you wish to use.

You can access the articles [here](https://jcooper-bit.github.io/lars-site/)

---

## ✨ Features (Current)

✅ **Vectors (`Vec2`, `Vec3`)**
- Addition, subtraction, negation
- Scalar and component-wise multiplication/division
- Dot and cross products
- Magnitude (`mag`, `mag_sq`) and normalization
- Functional mapping (`map`)
- Unit tests and full documentation examples

✅ **Matrices (`Mat2`, `Mat3`)**
- Addition, subtraction
- Scalar,Vector and Matrix multiplication
- Determinants
- Inversion (where applicable)
- Unit tests and full documentation examples

Example:
```rust
use lars::Vec2;

fn main() {
    let a = Vec2::new(3.0, 4.0);
    println!("|a| = {}", a.mag()); // 5.0
}
```


---

## 🔧 Implementation Details

### 📚 [Documentation](https://jcooper-bit.github.io/lars/lars)

The `Vec2` and `Vec3` structs are implemented with full operator overloading via [`derive_more`](https://crates.io/crates/derive_more), and includes:
- Safe, idiomatic Rust design
- Panics documented for mathematical errors
- Strong inline documentation (`///`) and tests
- Educational explanations in code comments

Example:
```rust
let a = Vec2::new(1.0, 2.0);
let b = Vec2::new(3.0, 4.0);

println!("Dot: {}", a.dot(&b));    // 11
println!("Cross: {}", a.cross(&b)); // 1
```

---

## 📦 Project Structure

```
src/
 ├── lib.rs
 ├── vector/
 │    ├── mod.rs
 │    ├── vec2.rs   # ✅ Implemented
 │    └── vec3.rs   # ✅ Implemented
 ├── matrix/
 │    ├── mod.rs
 │    ├── mat2.rs   # ✅ Implemented
 │    └── mat3.rs   # ⏳ Planned
 ├── traits.rs      # ⏳ Planned
 └── transformations.rs  # ⏳ Planned
```

---

## 🤝 Contributing

This is primarily a **personal learning project**, but suggestions and PRs are of course always welcome,  especially those that improve mathematical correctness, numerical stability, or Rust best practices.

---
