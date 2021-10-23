use crate::domain::entities::{Pokemon, PokemonName, PokemonNumber, PokemonTypes};

pub enum Insert {
    Ok(PokemonNumber),
    Conflict,
    Error
}

pub trait Repository: Sync + Send {
    fn insert(&mut self, number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Insert;
}

pub struct InMemoryRepository {
    error: bool,
    pokemons: Vec<Pokemon>,
}

impl Repository for InMemoryRepository {
    fn insert(&mut self, number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Insert {
        if self.error {
            return Insert::Error;
        }

        if self.pokemons.iter().any(|pokemon| pokemon.number == number) {
            return Insert::Conflict;
        }

        let number_clone = number.clone();
        self.pokemons.push(Pokemon::new(number_clone, name, types));
        Insert::Ok(number)
    }
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self { 
            error: false,
            pokemons: vec![]
        }
    }

    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}
