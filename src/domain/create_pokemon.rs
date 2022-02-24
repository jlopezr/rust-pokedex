use std::sync::Arc;

use crate::domain::entities::{PokemonName, PokemonNumber, PokemonTypes};
use crate::repositories::pokemon::{Repository, Insert};

pub struct Request {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Response {
    Ok(u16),
    BadRequest,
    Conflict,
    Error
}

pub enum Error {
    BadRequest,
    Conflict,
    Unknown,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<u16,Error> {
    match (
        PokemonNumber::try_from(req.number),
        PokemonName::try_from(req.name),
        PokemonTypes::try_from(req.types),
    ) {
        (Ok(number), Ok(names), Ok(types)) => match repo.insert(number, names, types) {
            Insert::Ok(number) => Ok(u16::from(number)),
            Insert::Conflict => Err(Error::Conflict),
            Insert::Error => Err(Error::Unknown),
        },
        _ => Err(Error::BadRequest),
    }
}

#[cfg(test)]
mod tests {
    use crate::repositories::pokemon::InMemoryRepository;

    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let res = execute(repo, req);

        match res {
            Ok(res_number) => assert_eq!(res_number, number),
            _ => unreachable!(),
        };        
    }
    
    #[test]
    fn urn_a_bad_request_error_when_request_is_invalid() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request {
            number: 25,
            name: String::from(""),
            types: vec![String::from("Electric")],
        };

        let res = execute(repo, req);

        match res {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_return_a_conflict_error_when_pokemon_number_already_exists() {
        let number = PokemonNumber::try_from(25).unwrap();
        let name = PokemonName::try_from(String::from("Pikachu")).unwrap();
        let types = PokemonTypes::try_from(vec![String::from("Electric")]).unwrap();
        let repo = Arc::new(InMemoryRepository::new());
        repo.insert(number, name, types);
        let req = Request {
            number: 25,
            name: String::from("Charmander"),
            types: vec![String::from("Fire")],
        };

        let res = execute(repo, req);

        match res {
            Err(Error::Conflict) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_an_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let res = execute(repo, req);

        match res {
            Err(Error::Unknown) => {},
            _ => unreachable!(),
        }
    }
}