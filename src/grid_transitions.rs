use crate::grid_implementation::Grid;
use crate::grid_implementation::SIZE;
use crate::grid_implementation::Transitions;
use crate::grid_implementation::Transitions::AliveSteady;
use crate::grid_implementation::Transitions::DeadSteady;
use crate::grid_implementation::Transitions::A1;
use crate::grid_implementation::Transitions::A2;
use crate::grid_implementation::Transitions::A3;
use crate::grid_implementation::Transitions::D1;
use crate::grid_implementation::Transitions::D2;
use crate::grid_implementation::Transitions::D3;

use std::fmt;


//"🌑🌒🌓🌔🌕", "🌕🌖🌗🌘🌑"

impl Grid<Transitions> {
    fn transition_to_emote(transition: Transitions) -> String {
        match transition {
            AliveSteady => "🌕".to_string(),
            DeadSteady => "🌑".to_string(),
            A1 => "🌖".to_string(),
            A2 => "🌗".to_string(),
            A3 => "🌘".to_string(),
            D1 => "🌒".to_string(),
            D2 => "🌓".to_string(),
            D3 => "🌔".to_string(),
        }
    }

    fn array_to_emotes(line: [Transitions; SIZE]) -> String {
        line.into_iter().fold(String::new(), |acc, transition| {
            acc + &Self::transition_to_emote(transition)
        })
    }
}

impl fmt::Display for Grid<Transitions> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.matrix.into_iter().fold(String::new(), |acc, line| {
                acc + &Grid::array_to_emotes(line) + "\n"
            })
        )
    }
}
