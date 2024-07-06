<h1 align="center">
  Doc-Buildr
</h1>

<p align="center">
  <a href="https://github.com/mkpro118/doc-buildr/actions/workflows/tests.yml">
    <img alt="Rust 1.68" src="https://img.shields.io/badge/1.68-grey?style=flat&logo=rust&logoColor=orange&color=%232A2A2A">
  </a>

  <a href="https://github.com/mkpro118/doc-buildr/actions/workflows/tests.yml">
    <img alt="Tests" src="https://github.com/mkpro118/doc-buildr/actions/workflows/tests.yml/badge.svg">
  </a>
  
  <a href="https://github.com/mkpro118/CredentialsManager/blob/main/LICENSE">
    <img alt="MIT LICENSE" src="https://img.shields.io/badge/License-MIT-blue?style=flat&labelColor=%233f3f3f"/>
  </a>
</p>


doc-buildr is a command-line tool that generates markdown documentation from javadoc-style comments in C-style code. It aims to simplify the process of creating readable and maintainable documentation for your projects.

## Features

- Parses javadoc-style comments (`/** ... */`) associated with functions, structs, and enums
- Supports `@param` and `@return` tags for detailed function documentation
- Generates structured markdown output
- Allows specifying an output file or defaults to stdout

## Installation

To install doc-buildr, you need to have Rust and Cargo installed on your system. Then, you can build the project from source:

```bash
git clone https://github.com/yourusername/doc-buildr.git
cd doc-buildr
cargo build --release
```

The compiled binary will be available in `target/release/doc-buildr`.

## Usage

Basic usage:

```bash
doc-buildr input_file.c
```

Process multiple files and specify an output file:

```bash
doc-buildr input1.c input2.c -o output.md
```

If no output file is specified, the documentation will be printed to stdout.

## Example

Given a C file with the following content:

```c
// add.c
/**
 * Adds two integers.
 * @param a The first integer
 * @param b The second integer
 * @return The sum of a and b
 */
int add(int a, int b);
```

doc-buildr will generate markdown documentation like this:

<pre>
# Module add

## Function `add`

```c
int add(int a, int b)
```

Adds two integers.


**Returns**:

`int`: The sum of a and b

**Parameters**:
- `a`: The first integer
- `b`: The second integer
</pre>

which would be rendered as

---

# Module add

## Function `add`

```c
int add(int a, int b)
```

Adds two integers.


**Returns**:

`int`: The sum of a and b

**Parameters**:
- `a`: The first integer
- `b`: The second integer

---

## Contributing

Contributions to doc-buildr are welcome! If you have a bug report, feature request, or want to contribute code, please open an issue or pull request on the GitHub repository.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
