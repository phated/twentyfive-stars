# twentyfive-stars

(Work-in-progress) GraphQL server written in Rust.

## About

This project aims to provide a [Relay-compliant GraphQL server](https://relay.dev/docs/en/graphql-server-specification) that models all cards in the [Transformers Trading Card Game](https://transformerstcg.hasbro.com/en-us) as well-defined entities.

The Transformers TCG is an interesting thing to model because the game is still fairly new, yet is very dynamic due to the different types of cards and the variations of data they provide.

## Technologies

This list is likely to change as I explore and improve on this project:

__Server:__
* [rust](https://www.rust-lang.org)!
* [async-graphql](https://github.com/async-graphql/async-graphql)
* [sqlx](https://github.com/launchbadge/sqlx) - custom fork for my user-defined types right now
* [tide](https://github.com/launchbadge/sqlx)
* [postgres](https://www.postgresql.org)

__Client:__
* [ReasonML](https://reasonml.github.io)
* [Reason-Relay](https://reason-relay-documentation.zth.now.sh)
* [Reason-React](https://reasonml.github.io/reason-react/)

## Setting up

TODO

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
