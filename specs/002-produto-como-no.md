# Spec 002: Produto como Nó do Grafo

## Objetivo

Definir o contrato do produto usado no projeto, considerando que cada produto é um nó e suas relações com outros produtos são arestas.

## Modelo Conceitual

- Um `Product` é um nó do grafo.
- Um `Edge` é uma conexão de saída de um produto para outro produto.
- O grafo é direcionado: a aresta parte do produto atual para o `to_id`.

## Estrutura do Produto

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
      "relationship": "ACESSORIO_COMPATIVEL"
    }
  ]
}
```

## Campos Obrigatórios

### Product

- `id`: inteiro único do produto
- `name`: nome legível do produto
- `description`: descrição textual usada em busca
- `data`: metadados do produto
- `edges`: lista de relações de saída

### Product.data

- `price`: número decimal maior ou igual a zero
- `category`: texto que classifica o produto

### Edge

- `to_id`: identificador do produto de destino
- `relationship`: tipo semântico da relação

## Regras

1. `id` deve ser único em toda a pasta `data/`.
2. `edges` pode ser vazio.
3. `to_id` pode apontar para um produto ausente, mas isso deve ser tratado como referência incompleta, não como erro fatal.
4. `relationship` deve representar o tipo da conexão, por exemplo `COMPATIVEL_COM` ou `COMPRADO_JUNTO`.
5. O sistema pode ignorar campos desconhecidos, mas não deve perder os campos definidos nesta spec.

## Observações de Implementação

- O projeto atual já usa `id`, `name`, `description`, `data.price` e `edges`.
- Os arquivos de exemplo também possuem `data.category`; esse campo deve fazer parte da spec oficial do produto.
- A modelagem deve permitir futuras relações entre acessórios, compatibilidade e co-compra.
