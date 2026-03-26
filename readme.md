# Megastore Graph Search

CLI em Rust para carregar produtos da pasta `data/` em memória e realizar buscas textuais em um grafo de produtos relacionados.

## O que o projeto faz

- Lê todos os arquivos `.json` de `data/`
- Converte cada arquivo em um `Product`
- Guarda os produtos em memória
- Permite buscar por nome ou descrição
- Exibe as relações (`edges`) do produto encontrado

## Estrutura do projeto

- `src/main.rs`: loop do CLI e exibição dos resultados
- `src/file_loader.rs`: leitura dos arquivos JSON
- `src/models.rs`: structs `Product`, `ProductData` e `Edge`
- `src/search.rs`: lógica de busca
- `data/`: base de produtos usada pelo programa
- `specs/`: especificações funcionais e do modelo de produto

## Como executar

```bash
cargo run
```

Você verá algo como:

```text
Carregando produtos...
10 produtos carregados. Motor de busca pronto.
>
```

## Comandos do CLI

### `search <termo>`

Busca por produtos cujo nome ou descrição contenham o termo informado.

Exemplos:

```text
> search camera
> search câmera
> search lente
```

Observações:

- O termo deve vir na mesma linha do comando
- A busca ignora diferença entre maiúsculas e minúsculas
- A busca também ignora acentos em termos comuns, como `camera` e `câmera`

Exemplo correto:

```text
> search camera
```

Exemplo incorreto:

```text
> search
Uso: search <termo>
> camera
Comando desconhecido: 'camera'. Use 'help' para listar os comandos.
```

Isso acontece porque `camera` sozinho é interpretado como um novo comando, não como continuação da linha anterior.

### `help`

Mostra os comandos disponíveis:

```text
> help
```

### `exit`

Encerra o programa:

```text
> exit
```

## Exemplo de uso completo

```text
> help
> search camera
> search lente
> exit
```

## Rodando testes

```bash
cargo test
```

## Desenvolvimento

- Formatar código: `cargo fmt`
- Validar compilação: `cargo check`
- Executar testes: `cargo test`

## Modelo de dados

Cada produto é um nó do grafo e possui arestas para outros produtos:

```json
{
  "id": 123,
  "name": "Câmera A7S",
  "description": "Câmera profissional full-frame, ótima para vídeos.",
  "data": {
    "price": 15000.0,
    "category": "câmeras"
  },
  "edges": [
    {
      "to_id": 888,
      "relationship": "COMPATIVEL_COM"
    }
  ]
}
```

## Problemas comuns

### `search` sem resultado

Tente um termo mais específico, como:

- `search camera`
- `search lente`
- `search tripé`

### O programa parece parado

Depois do prompt `>`, digite um comando completo e pressione Enter. O CLI não entra em modo de pergunta e resposta; cada linha precisa conter um comando inteiro.
