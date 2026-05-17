# Modules

In this section we'll give you an introduction to Rust's module system.

## Further information

- [The Module System](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

## Meu Diário de Aprendizado

**`modules1.rs`**
- **O Problema:** A função `make_sausage` estava dentro de um módulo (`mod sausage_factory`) e não podia ser chamada pelo `main` porque era privada.
- **O que aprendi:** No Rust, tudo é privado por padrão. Para tornar uma função (ou qualquer item) acessível fora do seu módulo, precisamos adicionar a palavra-chave `pub` na frente (ex: `pub fn make_sausage()`).

**`modules2.rs`**
- **O Problema:** O `main` estava tentando acessar constantes de submódulos com nomes curtos (`fruit` e `veggie`), mas as constantes originais tinham nomes diferentes (`PEAR` e `CUCUMBER`) e estavam "escondidas" dentro dos submódulos `fruits` e `veggies`.
- **O que aprendi:** Como importar e renomear itens usando a palavra-chave `use` e `as`. A sintaxe `pub use caminho::item as novo_nome;` permite exportar o item com um apelido, facilitando o uso para quem chama de fora do módulo.

**`modules3.rs`**
- **O Problema:** O código usava `SystemTime` e `UNIX_EPOCH` da biblioteca padrão (`std::time`), mas o compilador não reconhecia esses nomes porque eles não haviam sido importados para o escopo atual.
- **O que aprendi:** Como importar múltiplos itens do mesmo módulo em uma única linha usando chaves `{}`. A sintaxe `use std::time::{SystemTime, UNIX_EPOCH};` é super limpa e evita a repetição do caminho completo do módulo.
