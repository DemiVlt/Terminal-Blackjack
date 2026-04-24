use rand::{
    rng,
    rngs::ThreadRng,
    seq::{IteratorRandom, SliceRandom},
};

struct Game {
    name_of_user: String,
    dealer_name: String,
    player_names: Vec<String>,
    retirement_funds: Vec<isize>,
    rng: ThreadRng,
}

fn aux_hand_val(hand: &[Card]) -> u8 {
    let value_of_hand_a11 = hand.iter().map(|x| x.value_a11()).sum::<u8>();
    let value_of_hand_a1 = hand.iter().map(|x| x.value_a1()).sum::<u8>();

    if value_of_hand_a11 > 21 {
        value_of_hand_a1
    } else {
        value_of_hand_a11
    }
}

impl Game {
    fn end_round(&mut self, mut players: Vec<Player>, dealer: Player) {
        println!(
            "The dealer's ({}'s) retirement fund is now {} cigs.",
            dealer.name, dealer.retirement_fund
        );

        players.push(dealer);
        players.rotate_right(
            (0..(self.player_names.len()))
                .choose(&mut self.rng)
                .unwrap(),
        );
        self.dealer_name = players.last().unwrap().name.clone();

        self.retirement_funds = players.iter().map(|x| x.retirement_fund).collect();

        self.player_names = players.into_iter().map(|x| x.name).collect();

        println!("\nAnd that's a round!");
        let _ = std::io::stdin().read_line(&mut "".to_string());
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Player {
    name: String,
    hand: Vec<Card>,
    retirement_fund: isize,
    bet: u32,
}

fn whether_npc(name_of_user: &str, player_name: &str) -> String {
    (name_of_user == player_name)
        .then_some(String::from("(That's you!)"))
        .unwrap_or_default()
}

impl Player {
    fn hitting_is_optimal(&self, mut deck: Vec<Card>, dealer_hand: &[Card]) -> bool {
        deck.push(dealer_hand[0].clone());

        let known = &dealer_hand[1];

        let mut worlds = 0;
        let mut worlds_won = 0;

        for (i, hidden) in deck.iter().enumerate() {
            let dealer_fog_hand_val = aux_hand_val(&[hidden.clone(), known.clone()]);

            let mut rest = deck.clone();
            rest.remove(i);

            for new_card in rest {
                let mut fog_hand = self.hand.clone();
                fog_hand.push(new_card);

                if aux_hand_val(&fog_hand) >= dealer_fog_hand_val && aux_hand_val(&fog_hand) < 21
                    || dealer_fog_hand_val > 21
                {
                    worlds_won += 1;
                }
                worlds += 1;
            }
        }

        let chance_of_victory_upon_hitting = worlds_won as f32 / worlds as f32;

        let chance_of_victory_upon_standing = deck
            .iter()
            .filter(|hidden| {
                self.hand_val() >= aux_hand_val(&[(*hidden).clone(), known.clone()])
                    && self.hand_val() < 21
                    || aux_hand_val(&[(*hidden).clone(), known.clone()]) > 21
            })
            .count() as f32
            / deck.len() as f32;

        chance_of_victory_upon_hitting > chance_of_victory_upon_standing
    }

    fn hand_val(&self) -> u8 {
        aux_hand_val(&self.hand)
    }

    fn busted(&self, _name_of_user: &str) -> bool {
        if self.hand_val() > 21 {
            println!("Oof! {} busted!", self.name,);
            return true;
        }

        false
    }

    fn blackjacked(&mut self, name_of_user: &str) -> bool {
        if self.hand_val() == 21 {
            println!(
                "{} got a blackjack! {}",
                self.name,
                whether_npc(name_of_user, &self.name)
            );
            // 1.5 bonus for blackjack (can backfire if dealer also gets one)
            self.bet *= 3;
            self.bet /= 2;

            return true;
        }

        false
    }

    fn bet_results(&mut self, dealer: &mut Player, won: bool) {
        if won {
            println!(
                "{} won their bet of {} cigs against the dealer!",
                self.name, self.bet
            );

            dealer.retirement_fund -= self.bet as isize;
            self.retirement_fund += self.bet as isize;
        } else {
            println!(
                "{} lost their bet of {} cigs against the dealer!",
                self.name, self.bet
            );

            dealer.retirement_fund += self.bet as isize;
            self.retirement_fund -= self.bet as isize;
        }

        println!("Their retirement fund is now at {}.", self.retirement_fund);
        let _ = std::io::stdin().read_line(&mut "".to_string());
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Card(Rank, Suit);

impl Card {
    fn name(&self) -> String {
        let rank = match self.0 {
            Rank::NumeralCard(x) => &format!("{x}"),
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
            Rank::Ace => "Ace",
        };
        let suit = match self.1 {
            Suit::Spade => "Spades",
            Suit::Heart => "Hearts",
            Suit::Club => "Clubs",
            Suit::Diamond => "Diamonds",
        };

        format!("{} of {}", rank, suit)
    }

    fn value_a11(&self) -> u8 {
        match self.0 {
            Rank::NumeralCard(x) => x,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ace => 11,
        }
    }

    fn value_a1(&self) -> u8 {
        match self.0 {
            Rank::NumeralCard(x) => x,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ace => 1,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Rank {
    NumeralCard(u8),
    Jack,
    Queen,
    King,
    Ace,
}

// NEVER CHANGE THE SIZE OF THIS (no reason to anyway, but still)
#[repr(u8)]
#[derive(Clone, PartialEq, Eq)]
#[allow(dead_code, unused)]
enum Suit {
    Spade = 0,
    Heart = 1,
    Club = 2,
    Diamond = 3,
}

impl From<u8> for Suit {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Suit>(value & 0b11) }
    }
}

// TODO:
// make ["McMurphy", "Taber", "Martini"] modes
// McMurphy mode => can always choose to be dealer
// taber mode => only gives you a finite time to answer
// martini (multiplayer) mode => has you play as every player
// normal characters are bibbit & cheswick
//
// make commands for the player to call, i.e. "help"
// and "scanlon" can always be typed to force quit
//
// declare 64 is starting fund
// tell teach about the default of aces as told by hand_val
//
// REMOVE PLAYERS FROM THE GAME ONCE THEY GO NEGATIVE (don't let them actually go -, make them go
// brankrupt
fn main() {
    print!("{esc}c", esc = 27 as char);
    println!("Who are you?");

    let mut name_of_user = String::new();
    let _ = std::io::stdin()
        .read_line(&mut name_of_user)
        .expect("Failed to read to user_input");
    name_of_user = name_of_user.trim().to_string();

    let mut rng = rng();

    let retirement_funds = vec![64isize; 5];
    let mut player_names: Vec<String> = ["McMurphy", "Taber", "Martini", "Cheswick", "Billy"]
        .into_iter()
        .map(String::from)
        .collect();
    player_names.shuffle(&mut rng);

    if !player_names.contains(&name_of_user) {
        player_names.pop();
        player_names.push(name_of_user.clone());
    }
    player_names.shuffle(&mut rng);

    let dealer_name = player_names.last().unwrap().clone();

    let mut game = Game {
        name_of_user,
        dealer_name,
        player_names,
        retirement_funds,
        rng,
    };

    loop {
        print!("{esc}c", esc = 27 as char);
        println!(
            "A new round begins!\n{} is now the dealer! {}\n",
            game.dealer_name,
            whether_npc(&game.name_of_user, &game.dealer_name)
        );

        let bet = loop {
            println!("Place your bet: ");

            let mut user_input = String::new();
            let _ = std::io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read to user_input");

            if let Ok(bet) = user_input.trim().parse::<u32>() {
                break bet;
            }
        };

        let mut deck = (0..4)
            .flat_map(|s: u8| {
                (2..=10)
                    .map(|x| Card(Rank::NumeralCard(x), s.into()))
                    .chain([
                        Card(Rank::Queen, s.into()),
                        Card(Rank::Jack, s.into()),
                        Card(Rank::King, s.into()),
                        Card(Rank::Ace, s.into()),
                    ])
                    .collect::<Vec<Card>>()
            })
            .collect::<Vec<Card>>();

        deck.shuffle(&mut game.rng);

        let mut players = vec![];

        for (player_name, retirement_fund) in
            game.player_names.iter().zip(game.retirement_funds.iter())
        {
            let hand = vec![deck.pop().unwrap()];

            if player_name == &game.dealer_name {
                println!(
                    "The dealer, {}, places their first card face down. {}",
                    game.dealer_name,
                    whether_npc(&game.name_of_user, player_name)
                );
            } else {
                println!(
                    "{}'s first card is the {}. {}",
                    player_name,
                    hand[0].name(),
                    whether_npc(&game.name_of_user, player_name)
                );
            }

            let _ = std::io::stdin().read_line(&mut "".to_string());

            let player = Player {
                name: player_name.to_string(),
                hand,
                retirement_fund: *retirement_fund,
                bet: match player_name.as_str() {
                    x if x == game.name_of_user => bet,
                    "Martini" => 6,
                    "Cheswick" => (10..16).choose(&mut game.rng).unwrap(),
                    "Taber" => (16..32).choose(&mut game.rng).unwrap(),
                    "McMurphy" => 16,
                    "Billy" => (3..=6).choose(&mut game.rng).unwrap(),
                    _ => panic!("AHHH, this shouldn't happen!"),
                },
            };

            players.push(player);
        }

        println!();

        for player in players.iter_mut() {
            player.hand.push(deck.pop().unwrap());

            println!(
                "{}'s second card is the {}. {}",
                player.name,
                player.hand[1].name(),
                whether_npc(&game.name_of_user, &player.name)
            );

            player.blackjacked(&game.name_of_user);
            let _ = std::io::stdin().read_line(&mut "".to_string());
        }

        println!();

        let dealer_hand_before_they_hit_or_stand = players.last().unwrap().hand.clone();

        for player in players.iter_mut() {
            if player.name == *game.dealer_name || player.hand_val() == 21 {
                continue;
            }
            loop {
                let hit = match player.name.as_str() {
                    x if x == game.name_of_user => loop {
                        println!("Hit or stand?");

                        let mut user_input = String::new();
                        let _ = std::io::stdin()
                            .read_line(&mut user_input)
                            .expect("Failed to read to user_input");

                        match user_input.trim().to_lowercase().as_str() {
                            "hit" => break true,
                            "stand" => break false,
                            _ => continue,
                        }
                    },
                    "Martini" => rand::random_bool(2. / 3.),
                    "Cheswick" => player.hand_val() < (16..=19).choose(&mut game.rng).unwrap(),
                    "Taber" => player.hand_val() < 18,
                    "McMurphy" => player
                        .hitting_is_optimal(deck.clone(), &dealer_hand_before_they_hit_or_stand),
                    "Billy" => player.hand_val() < (14..=16).choose(&mut game.rng).unwrap(),
                    _ => panic!("AHHH, this shouldn't happen!"),
                };

                if !hit {
                    println!("{} stands.", player.name);
                    break;
                }

                let new_card = deck.pop().unwrap();
                let new_card_name = new_card.name();

                player.hand.push(new_card);

                println!("{} hits, getting the {}.", player.name, new_card_name);

                if player.blackjacked(&game.name_of_user) || player.busted(&game.name_of_user) {
                    break;
                }

                if player.name != game.name_of_user {
                    let _ = std::io::stdin().read_line(&mut "".to_string());
                }
            }
            let _ = std::io::stdin().read_line(&mut "".to_string());
        }

        let mut dealer = players.pop().unwrap();

        println!("The dealer's hidden card is about to be revealed! Drumroll, please!");
        println!(
            "{}'s first card was the {}! {}",
            dealer.name,
            dealer.hand[0].name(),
            whether_npc(&game.name_of_user, &dealer.name)
        );

        if dealer.blackjacked(&game.name_of_user) {
            let _ = std::io::stdin().read_line(&mut "".to_string());
            for player in players.iter_mut() {
                player.bet_results(&mut dealer, true);
            }
            game.end_round(players, dealer);
            continue;
        }

        if dealer.hand_val() < 17 {
            println!("The dealer's hand is less than 17, so they're forced to hit.");

            let new_card = deck.pop().unwrap();
            let new_card_name = new_card.name();
            println!("{} hits, getting the {}.", dealer.name, new_card_name);
            dealer.hand.push(new_card);
        } else {
            println!("The dealer's hand is greater than 16, so they're forced to stand.");
        }

        let blackjacked = dealer.blackjacked(&game.name_of_user);
        let busted = dealer.busted(&game.name_of_user);
        let _ = std::io::stdin().read_line(&mut "".to_string());

        for player in players.iter_mut() {
            let player_won = !blackjacked
                && (busted || player.hand_val() > 21 || player.hand_val() < dealer.hand_val());
            player.bet_results(&mut dealer, player_won);
        }
        game.end_round(players, dealer);
    }
}
