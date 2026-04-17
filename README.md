# 🦀 Aprendendo com Rustlings

> 📚 Minha jornada aprendendo Rust com os exercícios do Rustlings — com anotações, erros cometidos e lições aprendidas em cada módulo.

---

## 🎯 Sobre este Repositório

Este não é apenas um fork do [Rustlings](https://github.com/rust-lang/rustlings). É um **diário de bordo público** da minha jornada aprendendo a linguagem de programação Rust do zero.

Para cada módulo de exercícios, documentei:
- 🐛 **O Problema** — qual era o erro ou o desafio proposto
- 🔧 **A Correção** — como resolvi e por quê funcionou
- 💡 **O que Aprendi** — o conceito por trás da solução
- ⚠️ **As Pegadinhas** — erros de lógica ou de entendimento que cometi no caminho

O objetivo é aprender Rust de forma profunda e honesta, não apenas "passar nos testes".

---

## 📂 Módulos Documentados

| Módulo | Tópico | Status |
|--------|--------|--------|
| [00 - Intro](./exercises/00_intro/README.md) | Introdução, macros e `println!` | ✅ Concluído |
| [01 - Variables](./exercises/01_variables/README.md) | Variáveis, mutabilidade, shadowing e constantes | ✅ Concluído |
| [02 - Functions](./exercises/02_functions/README.md) | Funções, parâmetros, tipos de retorno e retorno implícito | ✅ Concluído |
| [03 - If](./exercises/03_if/README.md) | Controle de fluxo, `if` como expressão e tipos consistentes nos braços | ✅ Concluído |
| [04 - Primitive Types](./exercises/04_primitive_types/README.md) | Booleanos, chars, arrays, slices e tuplas | ✅ Concluído |
| [05 - Vecs](./exercises/05_vecs/README.md) | Vectors, iteradores, `.map()`, closures e `.collect()` | ✅ Concluído |
| [06 - Move Semantics](./exercises/06_move_semantics/README.md) | Ownership, borrowing e referências | ✅ Concluído |
| [07 - Structs](./exercises/07_structs/README.md) | Structs regulares, de tupla, unitárias e métodos com `impl` | 🚧 Em progresso |
| [08 - Enums](./exercises/08_enums/README.md) | Enums com dados, pattern matching com `match` | 🚧 Em progresso |

---

## 🛠️ Como Rodar os Exercícios

Para executar um exercício específico e ver o resultado do `main` no terminal:

```bash
cargo run --bin <nome_do_exercicio>
```

**Exemplos:**
```bash
cargo run --bin if2
cargo run --bin vecs1
cargo run --bin variables4
```

> **Por que não usar só `cargo run`?**
> O projeto contém dezenas de binários (um por exercício). O Cargo não sabe qual executar sem que você especifique com `--bin`.

---

## 🚀 Sobre o Rustlings

O [Rustlings](https://github.com/rust-lang/rustlings) é um projeto oficial da comunidade Rust que oferece pequenos exercícios guiados para praticar a leitura e escrita de código Rust com o auxílio do compilador.

---

## 📬 Contato

Feito com 🦀 e muita paciência com o *borrow checker*.
