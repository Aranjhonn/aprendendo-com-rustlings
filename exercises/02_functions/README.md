# Functions

Here, you'll learn how to write functions and how the Rust compiler can help you debug errors even
in more complex code.

## Further information

- [How Functions Work](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

---

## Meu Diário de Aprendizado

**`functions1.rs`**
- **O Problema:** A função `main` estava tentando chamar uma função `call_me()` que não existia no código.
- **A Correção:** Criação básica de uma função usando a palavra-chave `fn` sem parâmetros e sem retorno.

**`functions2.rs`**
- **O Problema:** O parâmetro da função não tinha um tipo declarado. Diferente do `let` que consegue inferir tipos dinamicamente, os parâmetros de função no Rust precisam de tipos estritos declarados para que o compilador trave qualquer erro.
- **A Correção:** Adicionei o tipo `: u32` logo após o nome da variável na assinatura: `fn call_me(num: u32)`.

**`functions3.rs`**
- **O Problema:** A função `call_me` exigia receber um número, mas ao ser chamada não estava recebendo nenhum argumento.
- **A Correção:** Passei o argumento numérico esperado durante a invocação da função: `call_me(3)`.

**`functions4.rs`**
- **O Problema:** A função calculava valores de desconto e retornava esse cálculo no final, mas a assinatura da função omitia o tipo que ela devolveria.
- **A Correção:** Uso da "setinha" `->` seguida do tipo de dado na declaração: `fn sale_price(price: i64) -> i64`.

**`functions5.rs`**
- **O Problema:** A função afirmava que ia retornar um tipo de número inteiro, mas existia um ponto-e-vírgula (`num * num;`) na linha do cálculo.
- **O que aprendi:** O glorioso **Retorno Implícito** do Rust. Se a última expressão de uma função ou bloco **não tiver** um ponto-e-vírgula, o compilador entende que o resultado dali deve ser *retornado* automaticamente! Se tiver ponto-e-vírgula, o Rust apenas ignora o resultado da computação e retorna nada (também chamado de tipo unitário `()`).
- **A Correção:** Resumidamente bastou apagar o `;` do final da operação.
