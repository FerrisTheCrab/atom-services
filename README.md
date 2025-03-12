# Services Microservice

## Getting started

### Installation

```sh
cargo install --git https://github.com/ferristhecrab/atom-services
```

You **must** compile services with The following feature flags to run is as an executable
- core (required)

### Running

#### Prerequisite
MongoDB running with [authentication set up](https://www.geeksforgeeks.org/how-to-enable-authentication-on-mongodb/);

```sh
CONFIG=/home/yourname/.config/atomics/services.json atom-services
```

Where `CONFIG` can be replaced with the location to the config file.

## API

Schema definition in [schema](./src/schema), exposed struct `Router` and `InternalRouter` in [router.rs](./src/router.rs) for squashed microservices.

