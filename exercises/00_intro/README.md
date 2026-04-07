# Intro

Rust uses the `print!` and `println!` macros to print text to the console.

## Further information

- [Hello World](https://doc.rust-lang.org/rust-by-example/hello.html)
- [Formatted print](https://doc.rust-lang.org/rust-by-example/hello/print.html)

---

## Meu Diário de Aprendizado

**`intro1.rs`**
O primeiro exercício serviu apenas como uma apresentação para certificar que o compilador do Rust estava rodando perfeitamente e para mostrar como os testes automatizados funcionam no terminal.

**`intro2.rs`**
- **O Problema:** Um erro simples de impressão no terminal.
- **O que aprendi:** Em Rust, para imprimir texto, utilizamos as *macros* e não funções puras. O grande diferencial é que as macros sempre terminam com um ponto de exclamação no seu nome.
- **A Correção:** Adicionei o macro `println!` corretamente passando a string `"Hello world!"`.
