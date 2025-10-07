# 🧩 Set Theory & Boolean Algebra in Rust

A **Rust-based project** that provides fundamental operations and abstractions for **Set Theory** and **Boolean Algebra** — built with performance, safety, and mathematical rigor in mind.

---

## 🚀 Overview

This project implements the core concepts of:
- **Set Theory:** union, intersection, difference, symmetric difference, power set, Cartesian product, and more.  
- **Boolean Algebra:** logical operations (AND, OR, NOT, XOR), truth tables, simplification, and expression evaluation.

It’s designed to be **modular**, **extensible**, and **lightweight**, serving as both a **learning resource** and a **foundation** for higher-level symbolic or logic-based systems.

---

## 🧠 Features

- ✅ Immutable and efficient set representations  
- ✅ Type-safe Boolean expressions  
- ✅ Truth table generation  
- ✅ Expression simplification  
- ✅ Set-builder notation support
- ✅ Comprehensive test coverage  

---
## 🧰 Installation
```
git clone https://github.com/mamoussa405/ready_set_boole.git
cd ready_set_boole
cargo build
```

---
## 🧪 Running Tests
```
cargo test
```

---
## 🧩 Examples

### Working with Sets

```rust
use set_theory::Set;

fn main() {
    let a = Set::from([1, 2, 3]);
    let b = Set::from([3, 4, 5]);

    let union = a.union(&b);
    let intersection = a.intersection(&b);
    
    println!("A ∪ B = {:?}", union);
    println!("A ∩ B = {:?}", intersection);
}
