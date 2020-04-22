# twentyfive-stars

Real-world GraphQL server written in Rust using [`async-graphql`](https://github.com/sunli829/async-graphql) and [`diesel`](http://diesel.rs/)

## Complex query

I strive to create a data model that represents the complexity I think GraphQL really excels at.

Here is an example of a valid query (with full `Relay` support) that you can send to this server.

```graphql
query {
  allCards {
    pageInfo {
      hasPreviousPage
      hasNextPage
      startCursor
      endCursor
    }
    edges {
      node {
        ... on Node {
          id
        }
        ... on Card {
          tcgId
          rarity
          number
          category
          wave {
            ... on Node {
              id
            }
            tcgId
            name
            released
          }
        }
        ... on CharacterCard {
          modes {
            title
            faction
            stars
            type
            ... on AltMode {
              subtitle
              traits
              health
              attack
              defense
            }
            ... on BotMode {
              subtitle
              traits
              health
              attack
              defense
            }
          }
        }
        ... on BattleCard {
          title
          stars
          faction
          icons
          attackModifier
          defenseModifier
        }
      }
    }
  }
}
```

## License

[MPL-2.0](./LICENSE)
