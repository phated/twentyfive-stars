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
* [sqlx](https://github.com/launchbadge/sqlx)
* [tide](https://github.com/http-rs/tide)
* [postgres](https://www.postgresql.org)

__Client:__
* [ReasonML](https://reasonml.github.io)
* [Reason-Relay](https://reason-relay-documentation.zth.now.sh)
* [Reason-React](https://reasonml.github.io/reason-react/)

## Prerequisite: Auth0

This project uses [Auth0](https://auth0.com) for OAuth2 authentication and JWT creation/validation.

Please follow our [Setting up Auth0](./docs/setting-up-auth0.md) guide before trying to set up the server.

## Setting up the server

You'll need the Rust toolchain and Postgres installed on your computer.

1. Make sure you have Postgres running and copy the `.env.example` to `.env` with the `DATABASE_URL` pointing to your local database.
1. Install the `cargo sqlx` command using `cargo install --git git://github.com/launchbadge/sqlx sqlx-cli`
1. Create the database using `cargo sqlx database create`
1. Migrate the new database using `cargo sqlx migrate run`
1. Build and start the server with `cargo run`

## Seeding the database

You'll need Node.js and Yarn installed on your computer.

1. Get the JWT from a logged in GraphiQL session, by navigating to `localhost:3000/login` and then grabbing the token after "Bearer" under "HTTP Headers" once redirected to the GraphiQL interface.
1. Add the token to your `.env` file as `AUTH_TOKEN`
1. Fetch the Node dependencies using `yarn` command
1. Then seed the database with `yarn seed`

## Testing out the data

You can test the newly seeded data with the built-in playground.

Open your browser to `localhost:3000` and start exploring the schema and issuing requests!

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
