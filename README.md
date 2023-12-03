# fantasy_sports_platform

Welcome to your new fantasy_sports_platform project and to the internet computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

This Rust code represents a backend implementation for a Fantasy Sports Platform. The project provides functionalities to manage players, team lineups, and leagues for a fantasy sports application. It includes CRUD operations to handle player selections, team lineups, and leagues within the platform.

To learn more before you start working with fantasy_sports_platform, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

## Overview

### Structs 
  - `Player`: Represents an individual player in the fantasy sports platform, containing details like ID, name, age, team, position, points, price, and playing status.
  - `TeamLineup`: Represents a team lineup within the platform, containing an ID, name, players in the lineup, points, budget, and playing status.
  - `League`: Represents a league within the fantasy sports platform, containing an ID, name, and teams participating in the league.
### Traits Implemented 
- **Storable**: Implemented for `Player`, `TeamLineup`, and `League` structs to enable serialization and deserialization.
- **BoundedStorable**: Implemented for `Player`, `TeamLineup`, and `League` structs to manage their maximum size and if they have a fixed size.



## CRUD Operations
### Players:
Retrieve All Players 

    fn get_all_players() -> Result<Vec<Player>, Error> {
        // Implementation to retrieve all players
    }

Retrieve Player by ID

    fn insert_player(player_payload: PlayerPayload) -> Result<Player, String> {
        // Implementation to insert a new player
    }


Insert New Player 

    fn insert_player(player_payload: PlayerPayload) -> Result<Player, String> {
        // Implementation to insert a new player
    }

Update Player 

    fn update_player(id: u64, player_payload: PlayerPayload) -> Result<Player, Error> {
        // Implementation to update a player by ID
    }


Delete Player by ID

    fn delete_player(id: u64) -> Result<Player, Error> {
        // Implementation to delete a player by ID
    }


### Team Lineups:

Retrieve All Team Lineups 

    fn get_all_teams() -> Result<Vec<TeamLineup>, Error> {
        // Implementation to retrieve all team lineups
    }

Retrieve Team Lineup by ID 

    fn get_team(id: u64) -> Result<TeamLineup, Error> {
        // Implementation to retrieve a team lineup by ID
    }


Insert New Lineup

    fn insert_team(team_payload: TeamPayload) -> Result<TeamLineup, String> {
        // Implementation to insert a new team lineup
    }

Update Team Lineup 

    fn update_team(id: u64, team_payload: TeamPayload) -> Result<TeamLineup, Error> {
        // Implementation to update a team lineup by ID
    }

Delete Team Lineup by ID

    fn delete_team(id: u64) -> Result<TeamLineup, Error> {
        // Implementation to delete a team lineup by ID
    }


### Leagues

Retieve All Leagues 

    fn get_all_leagues() -> Result<Vec<League>, Error> {
        // Implementation to retrieve all leagues
    }

Retrieve League by ID 

    fn get_league(id: u64) -> Result<League, Error> {
        // Implementation to retrieve a league by ID
    }

Insert New League 

    fn insert_league(league_payload: LeaguePayload) -> Result<League, String> {
        // Implementation to insert a new league
    }

Update League 

    fn update_league(id: u64, league_payload: LeaguePayload) -> Result<League, Error> {
        // Implementation to update a league by ID
    }

Delete League by ID

    fn delete_league(id: u64) -> Result<League, Error> {
        // Implementation to delete a league by ID
    }

### Adding Player to Team

Add Player to Team:

    fn add_player_to_team(team_id: u64, player_id: u64) -> Result<(), String> {
        // Implementation to add a player to a team
    }

### Adding Team to League 

Add Team to League 


    fn add_team_to_league(league_id: u64, team_id: u64) -> Result<(), String> {
      // Implementation to add a team to a league
    }







If you want to start working on your project right away, you might want to try the following commands:

```bash
cd fantasy_sports_platform/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background --clean

# Deploys your canisters to the replica and generates your candid interface

npm run gen-deploy

```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
