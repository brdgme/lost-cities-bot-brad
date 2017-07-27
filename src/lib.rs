extern crate lost_cities;
extern crate brdgme_game;

use lost_cities::{Game, Phase, PlayerState};
use lost_cities::card::{expeditions, last_expedition, Card, Expedition, Value};
use brdgme_game::command::Spec as CommandSpec;
use brdgme_game::bot::{BotCommand, Botter};

pub struct Brad;

impl Botter<Game> for Brad {
    fn commands(
        &mut self,
        player: usize,
        player_state: &PlayerState,
        _players: &[String],
        _command_spec: &CommandSpec,
        _game_id: Option<String>,
    ) -> Vec<BotCommand> {
        if player != player_state.public.current_player {
            // Not my turn!
            return vec![];
        }
        match player_state.public.phase {
            Phase::PlayOrDiscard => handle_play_or_discard(player, player_state),
            Phase::DrawOrTake => handle_draw_or_take(player, player_state),
        }
    }
}

fn handle_play_or_discard(player: usize, player_state: &PlayerState) -> Vec<BotCommand> {
    let mut commands: Vec<BotCommand> = vec![];
    // Give a quality score to playing or discarding each card
    for c in &player_state.hand {
        let last = last_expedition(&player_state.public.expeditions[player], c.expedition);
        if last.is_none() || last.unwrap().value <= c.value {
            // We can play this one, we'll assign quality to it based on what's already out
            commands.push(BotCommand {
                quality: match last {
                    Some(ref lc) => play_quality_existing_expedition(c, lc, player, player_state),
                    None => play_quality_new_expedition(c, player, player_state),
                },
                commands: vec![format!("play {}", c)],
            });
        }
        commands.push(format!("discard {}", c).into());
    }
    commands
}

fn play_quality_existing_expedition(
    card: &Card,
    last: &Card,
    player: usize,
    player_state: &PlayerState,
) -> u8 {
    unimplemented!();
}

fn play_quality_new_expedition(card: &Card, player: usize, player_state: &PlayerState) -> u8 {
    unimplemented!();
}

fn handle_draw_or_take(player: usize, player_state: &PlayerState) -> Vec<BotCommand> {
    let mut commands: Vec<BotCommand> = vec![];
    if player_state.public.deck_remaining > 1 {
        // We only ever take if there is more than 1 card remaining, as otherwise we're just giving
        // our opponent one more turn than us.
    }
    commands.push("draw".into());
    commands
}

fn opponent(player: usize) -> usize {
    match player {
        0 => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
