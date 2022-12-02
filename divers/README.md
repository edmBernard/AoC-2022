# Standalone executable from other languages

## Rust

### Build

```
rustc standalone.rs
```
It will generate the executable `standalone`, see : [rustc](https://doc.rust-lang.org/rustc/what-is-rustc.html)

### Run

```
./standalone < input.txt
```

## C++ (clang)

### Build

```
clang++ standalone.cpp -std=c++20 -Ofast -o standalone
```
It will generate the executable `standalone`

### Run

```
./standalone < input.txt
```

## Fortran

### Build

```
gfortran standalone.f90 -o standalone
```

It will generate the executable `standalone`

### Run

```
./standalone < input.txt
```

## Python

### Run

```
python standalone.py < input.txt
```
