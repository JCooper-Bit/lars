# vec2_learning_challenges.md

# 🧭 lars Learning Series - Part 1
## Building a 2D Vector Type in Rust

---

###  Learning Goals
By the end of this lesson, students will:

- Understand how 2D vectors are represented mathematically and in code.  
- Implement a `Vec2` struct in Rust that supports basic vector operations.  
- Learn about traits, operator overloading, and code organization in a math-focused crate.  
- Write unit tests to validate mathematical correctness.  

---

## Background: What Is a Vector?

As you hopefully know from school, a **vector** is a quantity that has both **magnitude** (length) and **direction**. If not, you can read an introductory article [here](https://www.mathsisfun.com/algebra/vectors.html).

In 2D space, we write a vector as:

![\mathbf v = \begin{bmatrix} x \\ y \\ \end{bmatrix}](./equations/vec2/Vector_definition.png)

Next, I wish to cover some basic terminology.
**Unit** vectors are vectors that has a magnitude of 1 in a specific direction. for example:

![\hat x = \begin{bmatrix} 1 \\ 0 \\ \end{bmatrix}](./equations/vec2/hat_definition.png)

The dot product is a method for multiplying 2 vectors, it is written mathematically as follows:

![\mathbf a \cdot \mathbf b](./equations/vec2/adotb.png)

There are 2 methods to calculate the dot product

The first method:

![\mathbf a \cdot \mathbf b = |\mathbf a | \times |\mathbf b| \times cos(\theta)](./equations/vec2/dot_product_1.png)

![|\mathbf a | \, is \, the \, magnitude \, of  \, \mathbf a \\
|\mathbf b | \, is \, the \, magnitude \, of  \, \mathbf b \\](equations/vec2/dot%20product_1_1.png)

![\theta \, is \, the \, angle \, between \, \mathbf a \, and \, \mathbf b](equations/vec2/dot_product_1_2.png)

The second method:



Vectors are foundational in:
- Geometry (lines, points, and normals)  
- Physics (velocity, force, acceleration and pretty much everything else!)  
- Computer graphics (movement, transformations)  
- Game development (positions, rotations, collision detection)

Rust’s strong type system and operator traits make it ideal for implementing mathematical objects safely and efficiently.

---

##  Task 1: Defining a 2D Vector Type

First off, we need to create a Rust struct called `Vec2` to represent a 2D vector.
- It should contain `(x,y)` variables
- You should [derive](https://doc.rust-lang.org/rust-by-example/trait/derive.html) common traits such as `Clone`, `Copy`, `Debug`, `PartialEq` and `Constructor`
- Finally, you should define a few useful common constant vectors such as `ZERO`, `ONE` and **unit** vectors for the x and y directions.


This sets the foundation for all vector operations.

---

##  Task 2: Computing Magnitude (Vector Length)

---
##  Task 3: Vector Arithmetic

---

##  Task 4: The Dot Product

---

##  Task 5: The 2D Cross Product


---

##  Task 6: Normalizing a Vector


---

##  Task 7: Component Mapping

---

##  Task 8: Scalar Multiplication

---

##  Task 9: Using Vec2 as a Point


---

##  Task 10: Testing and Validation



