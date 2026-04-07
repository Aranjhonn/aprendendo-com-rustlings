# Vectors

Vectors are one of the most-used Rust data structures. In other programming
languages, they'd simply be called Arrays, but since Rust operates on a
bit of a lower level, an array in Rust is stored on the stack (meaning it
can't grow or shrink, and the size needs to be known at compile time),
and a Vector is stored in the heap (where these restrictions do not apply).

Vectors are a bit of a later chapter in the book, but we think that they're
useful enough to talk about them a bit earlier. We shall be talking about
the other useful data structure, hash maps, later.

## Further information

- [Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)

---

## Meu Diário de Aprendizado

**`vecs1.rs`**
- **O Problema:** Nós tínhamos um Array fixo `a` e precisávamos criar um *Vector* (Array dinâmico salvo no *Heap* que pode crescer) com os mesmos dados exatos.
- **A Correção:** Em vez de instanciar `Vec::new()` e dar `.push()` em cada número manualmente, usei a macro inteligente do Rust: `let v = vec![10, 20, 30, 40];`.

**`vecs2.rs`**
- **O Problema:** Entender como passar por todos os itens de uma lista modificando seus valores (nesse caso multiplicando todos por 2).
- **O que aprendi:** O Rust abraça as duas abordagens da programação para varrer listas e nós desenvolvemos ambas:
  1. **A forma Imperativa clássica:** Usando `for element in input`, pegamos item por item e socamos no nosso vector final de forma manual via `output.push(element * 2)`.
  2. **A forma Funcional/Declarativa:** Através da API de Iteradores, usamos `.iter().map(...)`. Esse método funciona como uma esteira de produção: usamos o *closure* (função anônima entre barras paralelas `|element|`) para multiplicar por `2` e, ao final de tudo, esmagamos a esteira dentro de um balde mágico usando o `.collect()`, convertendo todo aquele mapeamento num Vector novinho em folha!
