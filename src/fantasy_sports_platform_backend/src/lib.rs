#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};


type Memory = VirtualMemory<DefaultMemoryImpl>; 
type IdCell = Cell<u64, Memory>;


// Struct to represent a player
#[derive (candid::CandidType, Clone,Serialize, Deserialize)]
struct Player {
    id: u64,
    name: String,
    age: u32,
    team: String,
    position: String,
    points: u32,
    price: u32,
    is_playing: bool,
    is_captain: bool,
}


// Struct to represent a team lineup
#[derive (candid::CandidType, Clone,Serialize, Deserialize,Default)]
struct TeamLineup {
    id: u64,
    name: String,
    players: Vec<Player>,
    points: u32,
    budget: u32,
    is_playing: bool,
}


// Struct to represent a league
#[derive (candid::CandidType, Clone,Serialize, Deserialize)]
struct League {
    id: u64,
    name: String,
    teams: Vec<TeamLineup>,  //contains the teams in the league
}


// Implement the Storable and BoundedStorable traits for the Player struct
impl Storable for Player {
      fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl  BoundedStorable for Player {
     const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement the Storable and BoundedStorable traits for the TeamLineup struct
impl Storable for TeamLineup {
      fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl  BoundedStorable for TeamLineup {
     const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement the Storable and BoundedStorable traits for the League struct
impl Storable for League {
      fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl  BoundedStorable for League {
     const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
    static PLAYER_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );
    static TEAM_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), 0)
            .expect("Cannot create a counter")
    );
    static LEAGUE_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), 0)
            .expect("Cannot create a counter")
    );
    static PLAYER_MAP: RefCell<StableBTreeMap<u64, Player, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))))
    );
    static TEAM_MAP: RefCell<StableBTreeMap<u64, TeamLineup, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );
    static LEAGUE_MAP: RefCell<StableBTreeMap<u64, League, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );


}


#[derive(candid::CandidType,Serialize, Deserialize)]
struct PlayerPayload {
    name: String,
    age: u32,
    team: String,
    position: String,
    points: u32,
    price: u32,
    is_playing: bool,
    is_captain: bool,
}

#[derive(candid::CandidType,Serialize, Deserialize)]
struct TeamPayload {
    name: String,
    points: u32,
    budget: u32,
    is_playing: bool,
}

#[derive(candid::CandidType,Serialize, Deserialize)]

struct LeaguePayload {
    name: String,
}

//CRUD operations for the Player struct
#[ic_cdk::query]
fn get_all_players() -> Result<Vec<Player>, Error> {
    let player_map: Vec<(u64, Player)> =
        PLAYER_MAP.with(|service| service.borrow().iter().collect());
    let players: Vec<Player> = player_map
        .into_iter()
        .map(|(_, player)| player)
        .collect();

    if !players.is_empty() {
        Ok(players)
    } else {
        Err(Error::NotFound {
            msg: "No players found ".to_string(),
        })
    }
}

#[ic_cdk::query]
fn get_player(id: u64) -> Result<Player, Error> {
    let player = PLAYER_MAP.with(|service| service.borrow().get(&id));
    if let Some(player) = player {
        Ok(player)
    } else {
        Err(Error::NotFound {
            msg: format!("player with id={} not found", id),
        })
    }
}

#[ic_cdk::update]
fn insert_player(player_payload: PlayerPayload) -> Result<Player, String> {
    if player_payload.name.trim().is_empty() ||
       player_payload.age < 15 || player_payload.age > 50 ||
       player_payload.team.trim().is_empty() ||
       player_payload.position.trim().is_empty() {
        return Err("Invalid player data".to_string());
    }

    let id = PLAYER_ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");
    let player = Player {
        id,
        name: player_payload.name,
        age: player_payload.age,
        team: player_payload.team,
        position: player_payload.position,
        points: player_payload.points,
        price: player_payload.price,
        is_playing: player_payload.is_playing,
        is_captain: player_payload.is_captain,
    };
    do_insert_player(&player);
    Ok(player)
}

fn update_if_not_empty(field: &mut String, new_value: String) {
    if !new_value.trim().is_empty() {
        *field = new_value;
    }
}

fn update_if_positive(field: &mut u32, new_value: u32) {
    if new_value > 0 {
        *field = new_value;
    }
}

#[ic_cdk::update]
fn update_player(id: u64, player_payload: PlayerPayload) -> Result<Player, Error> {
    let player = PLAYER_MAP.with(|service| service.borrow().get(&id));
    if let Some(mut player) = player {
        update_if_not_empty(&mut player.name, player_payload.name);
        update_if_positive(&mut player.age, player_payload.age);
        update_if_not_empty(&mut player.team, player_payload.team);
        update_if_not_empty(&mut player.position, player_payload.position);
        update_if_positive(&mut player.points, player_payload.points);
        update_if_positive(&mut player.price, player_payload.price);
        player.is_playing = player_payload.is_playing;
        player.is_captain = player_payload.is_captain;
        do_insert_player(&player);
        Ok(player)
    } else {
        Err(Error::NotFound {
            msg: format!(
                "Update Player  with id={}. not found",
                id
        )} )
    }
}

// helper method to perform insert.
fn do_insert_player(player: &Player) {
    PLAYER_MAP.with(|service| {
        service
            .borrow_mut()
            .insert(player.id, player.clone())
    });
}

#[ic_cdk::update]
fn delete_player(id: u64) -> Result<Player, Error> {
    let player = PLAYER_MAP.with(|service| service.borrow_mut().remove(&id));
    if let Some(player) = player {
        Ok(player)
    } else {
        Err(Error::NotFound {
            msg: format!("Player with id={} not found", id),
        })
    }
}

//CRUD operations for the TeamLineup struct
#[ic_cdk::query]
fn get_all_teams() -> Result<Vec<TeamLineup>, Error> {
    let teams = TEAM_MAP.with(|service| service.borrow().iter().map(|(_, team)| team.clone()).collect::<Vec<_>>());
    let existing_teams: Vec<TeamLineup> = teams.into_iter().filter(|team| team_exists(team.id)).collect();
    if !existing_teams.is_empty() {
        Ok(existing_teams)
    } else {
        Err(Error::NotFound {
            msg: "No teams found ".to_string(),
        })
    }
}

#[ic_cdk::query]
fn get_team(id: u64) -> Result<TeamLineup, Error> {
    let team = TEAM_MAP.with(|service| service.borrow().get(&id));
    if let Some(team) = team {
        Ok(team)
    } else {
        Err(Error::NotFound {
            msg: format!("Team with id={} not found", id),
        })
    }
}

#[ic_cdk::update]
fn insert_team(team_payload: TeamPayload) -> Result<TeamLineup, String> {
    if team_payload.name.trim().is_empty(){
        return Err("Invalid team data".to_string());
    }

    let id = TEAM_ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");
    let team = TeamLineup {
        id,
        name: team_payload.name,
        players: Vec::new(),
        points: team_payload.points,
        budget: team_payload.budget,
        is_playing: team_payload.is_playing,
    };
    do_insert_team(&team);
    Ok(team)
}

fn do_insert_team(team: &TeamLineup) {
    TEAM_MAP.with(|service| {
        service
            .borrow_mut()
            .insert(team.id, team.clone())
    });
}


#[ic_cdk::update]
fn update_team(id: u64, team_payload: TeamPayload) -> Result<TeamLineup, Error> {
    let team = TEAM_MAP.with(|service| service.borrow().get(&id));
    if let Some(mut team) = team {
        update_if_not_empty(&mut team.name, team_payload.name);
        update_if_positive(&mut team.points, team_payload.points);
        update_if_positive(&mut team.budget, team_payload.budget);
        team.is_playing = team_payload.is_playing;
        do_insert_team(&team);
        Ok(team)
    } else {
        Err(Error::NotFound {
            msg: format!(
                "Update Team  with id={}. not found",
                id
        )} )
    }
}

fn player_exists(id: u64) -> bool {
    PLAYER_MAP.with(|service| service.borrow().contains_key(&id))
}

#[ic_cdk::update]
fn add_player_to_team(team_id: u64, player_id: u64) -> Result<(), String> {
    if !player_exists(player_id) {
        return Err("Player does not exist".to_string());
    }
    let team = TEAM_MAP.with(|service| service.borrow().get(&team_id));
    if let Some(mut team) = team {
        if team.players.iter().any(|player| player.id == player_id) {
            return Err("Player already exists in team".to_string());
        }
        let player = PLAYER_MAP.with(|service| service.borrow().get(&player_id));
        if let Some(player) = player {
            team.players.push(player.clone());
            do_insert_team(&team);
            Ok(())
        } else {
            Err("Player does not exist".to_string())
        }
    } else {
        Err("Team does not exist".to_string())
    }

}
#[ic_cdk::update]
fn delete_team(id: u64) -> Result<TeamLineup, Error> {
    let team = TEAM_MAP.with(|service| service.borrow_mut().remove(&id));
    if let Some(team) = team {
        Ok(team)
    } else {
        Err(Error::NotFound {
            msg: format!("Team with id={} not found", id),
        })
    }
}

//CRUD operations for the League struct
#[ic_cdk::query]
fn get_all_leagues() -> Result<Vec<League>, Error> {
    let league_map: Vec<(u64, League)> =
        LEAGUE_MAP.with(|service| service.borrow().iter().collect());
    let leagues: Vec<League> = league_map
        .into_iter()
        .map(|(_, league)| league)
        .collect();

    if !leagues.is_empty() {
        Ok(leagues)
    } else {
        Err(Error::NotFound {
            msg: "No leagues found ".to_string(),
        })
    }
}

#[ic_cdk::query]
fn get_league(id: u64) -> Result<League, Error> {
    let league = LEAGUE_MAP.with(|service| service.borrow().get(&id));
    if let Some(league) = league {
        Ok(league)
    } else {
        Err(Error::NotFound {
            msg: format!("League with id={} not found", id),
        })
    }
}

#[ic_cdk::update]

fn insert_league(league_payload: LeaguePayload) -> Result<League, String> {
    if league_payload.name.trim().is_empty(){
        return Err("Invalid league data".to_string());
    }

    let id = LEAGUE_ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");
    let league = League {
        id,
        name: league_payload.name,
        teams: Vec::new(),
    };
    do_insert_league(&league);
    Ok(league)
}

fn do_insert_league(league: &League) {
    LEAGUE_MAP.with(|service| {
        service
            .borrow_mut()
            .insert(league.id, league.clone())
    });
}

#[ic_cdk::update]
fn update_league(id: u64, league_payload: LeaguePayload) -> Result<League, Error> {
    let league = LEAGUE_MAP.with(|service| service.borrow().get(&id));
    if let Some(mut league) = league {
        update_if_not_empty(&mut league.name, league_payload.name);
        do_insert_league(&league);
        Ok(league)
    } else {
        Err(Error::NotFound {
            msg: format!(
                "Update League  with id={}. not found",
                id
        )} )
    }
}

fn team_exists(id: u64) -> bool {
    TEAM_MAP.with(|service| service.borrow().contains_key(&id))
}

#[ic_cdk::update]
fn add_team_to_league(league_id: u64, team_id: u64) -> Result<(), String> {
    if !team_exists(team_id) {
        return Err("Team does not exist".to_string());
    }
    let league = LEAGUE_MAP.with(|service| service.borrow().get(&league_id));
    if let Some(mut league) = league {
        if league.teams.iter().any(|team| team.id == team_id) {
            return Err("Team already exists in league".to_string());
        }
        let team = TEAM_MAP.with(|service| service.borrow().get(&team_id));
        if let Some(team) = team {
            league.teams.push(team.clone());
            do_insert_league(&league);
            Ok(())
        } else {
            Err("Team does not exist".to_string())
        }
    } else {
        Err("League does not exist".to_string())
    }

}





#[ic_cdk::update]

fn delete_league(id: u64) -> Result<League, Error> {
    let league = LEAGUE_MAP.with(|service| service.borrow_mut().remove(&id));
    if let Some(league) = league {
        Ok(league)
    } else {
        Err(Error::NotFound {
            msg: format!("League with id={} not found", id),
        })
    }
}

#[ic_cdk::query]

fn get_all_players_in_team(id: u64) -> Result<Vec<Player>, Error> {
    let team = TEAM_MAP.with(|service| service.borrow().get(&id));
    if let Some(team) = team {
        Ok(team.players)
    } else {
        Err(Error::NotFound {
            msg: format!("Team with id={} not found", id),
        })
    }
}

#[ic_cdk::query]

fn get_all_teams_in_league(id: u64) -> Result<Vec<TeamLineup>, Error> {
    let league = LEAGUE_MAP.with(|service| service.borrow().get(&id));
    if let Some(league) = league {
        Ok(league.teams)
    } else {
        Err(Error::NotFound {
            msg: format!("League with id={} not found", id),
        })
    }
}

#[ic_cdk::query]

fn get_all_players_in_league(id: u64) -> Result<Vec<Player>, Error> {
    let league = LEAGUE_MAP.with(|service| service.borrow().get(&id));
    if let Some(league) = league {
        let mut players: Vec<Player> = Vec::new();
        for team in league.teams.iter() {
            for player in team.players.iter() {
                players.push(player.clone());
            }
        }
        Ok(players)
    } else {
        Err(Error::NotFound {
            msg: format!("League with id={} not found", id),
        })
    }
}

#[ic_cdk::query]

fn get_all_players_in_league_sorted_by_points(id: u64) -> Result<Vec<Player>, Error> {
    let league = LEAGUE_MAP.with(|service| service.borrow().get(&id));
    if let Some(league) = league {
        let mut players: Vec<Player> = Vec::new();
        for team in league.teams.iter() {
            for player in team.players.iter() {
                players.push(player.clone());
            }
        }
        players.sort_by(|a, b| b.points.cmp(&a.points));
        Ok(players)
    } else {
        Err(Error::NotFound {
            msg: format!("League with id={} not found", id),
        })
    }
}

#[ic_cdk::query]

fn get_all_players_in_league_sorted_by_price(id: u64) -> Result<Vec<Player>, Error> {
    let league = LEAGUE_MAP.with(|service| service.borrow().get(&id));
    if let Some(league) = league {
        let mut players: Vec<Player> = Vec::new();
        for team in league.teams.iter() {
            for player in team.players.iter() {
                players.push(player.clone());
            }
        }
        players.sort_by(|a, b| b.price.cmp(&a.price));
        Ok(players)
    } else {
        Err(Error::NotFound {
            msg: format!("League with id={} not found", id),
        })
    }
}


#[ic_cdk::query]


fn get_all_players_in_league_sorted_by_team(id: u64) -> Result<Vec<Player>, Error> {
    let league = LEAGUE_MAP.with(|service| service.borrow().get(&id));
    if let Some(league) = league {
        let mut players: Vec<Player> = Vec::new();
        for team in league.teams.iter() {
            for player in team.players.iter() {
                players.push(player.clone());
            }
        }
        players.sort_by(|a, b| b.team.cmp(&a.team));
        Ok(players)
    } else {
        Err(Error::NotFound {
            msg: format!("League with id={} not found", id),
        })
    }
}

#[ic_cdk::query]

fn get_all_players_in_league_sorted_by_position(id: u64) -> Result<Vec<Player>, Error> {
    let league = LEAGUE_MAP.with(|service| service.borrow().get(&id));
    if let Some(league) = league {
        let mut players: Vec<Player> = Vec::new();
        for team in league.teams.iter() {
            for player in team.players.iter() {
                players.push(player.clone());
            }
        }
        players.sort_by(|a, b| b.position.cmp(&a.position));
        Ok(players)
    } else {
        Err(Error::NotFound {
            msg: format!("League with id={} not found", id),
        })
    }
}

#[ic_cdk::query]

fn get_all_teams_in_league_sorted_by_points(id: u64) -> Result<Vec<TeamLineup>, Error> {
    let league = LEAGUE_MAP.with(|service| service.borrow().get(&id));
    if let Some(league) = league {
        let mut teams: Vec<TeamLineup> = Vec::new();
        for team in league.teams.iter() {
            teams.push(team.clone());
        }
        teams.sort_by(|a, b| b.points.cmp(&a.points));
        Ok(teams)
    } else {
        Err(Error::NotFound {
            msg: format!("League with id={} not found", id),
        })
    }
}


#[ic_cdk::query]
fn get_highest_scoring_player() -> Option<Player> {
    let players = get_all_players();
    players.into_iter().flatten().max_by_key(|player| player.points)
}



// Error type for the service
#[derive(candid::CandidType, Deserialize, Serialize)]
enum  Error {
    NotFound { msg: String },
}

// Export the candid interface
ic_cdk::export_candid!();