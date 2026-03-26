# Spec 001: Leitura de Produtos e Busca em Grafo

## Objetivo

Permitir que o sistema leia todos os arquivos JSON da pasta `data/`, carregue os produtos em memória e execute buscas sobre um grafo de produtos.

## Contexto Atual

Os arquivos em `data/` representam produtos individuais. Cada produto possui um identificador único e uma lista de arestas (`edges`) que apontam para outros produtos por `to_id`.

Exemplo observado:

- `123.json` representa a `Câmera A7S`
- `888.json` representa a `Lente 50mm f/1.8`
- o produto `123` possui arestas para `888` e `999`

## Requisitos Funcionais

1. O sistema deve ler todos os arquivos `.json` presentes em `data/`.
2. Cada arquivo deve ser desserializado como um produto válido.
3. Os produtos devem ser mantidos em memória em uma estrutura indexada por `id`.
4. O produto deve ser tratado como um nó de um grafo direcionado.
5. Cada item em `edges` deve ser tratado como uma aresta de saída do produto.
6. O sistema deve permitir busca textual por nome e descrição do produto.
7. O resultado da busca deve retornar o produto e suas relações conhecidas.
8. Caso uma aresta aponte para um `to_id` inexistente, o sistema não deve falhar; deve apenas sinalizar referência ausente.

## Estrutura em Memória

Estrutura recomendada:

- `HashMap<i32, Product>` para acesso rápido por identificador
- `Vec<Edge>` dentro de cada produto para representar conexões

## Critérios de Aceitação

1. Ao iniciar, o sistema carrega todos os produtos válidos de `data/`.
2. Uma busca por termo existente retorna um ou mais produtos.
3. O resultado exibe as conexões do produto para outros nós.
4. Referências quebradas em `edges` são tratadas sem interromper a execução.
5. O carregamento e a busca funcionam com pelo menos os exemplos atuais de `data/`.

## Fora de Escopo Nesta Fase

- persistência em banco
- atualização dinâmica dos arquivos em tempo real
- algoritmos avançados de travessia, ranking ou recomendação
- escrita de novos produtos
