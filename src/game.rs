use rand::prelude::*;

pub fn play() {
    /* This deck is an array of tuples, each tuple is a distinct card 
    Suits: H - Hearts, T - Tiles, C - Clovers, P - Pikes
    Ranks: 2 to 10, 11 - Jack, 12 - Queen, 13 - King, 14 - Ace
    */
    let deck = [(2, 'H'), (3, 'H'), (4, 'H'), (5, 'H'), (6, 'H'), (7, 'H'), (8, 'H'), (9, 'H'), 
    (10, 'H'), (11, 'H'), (12, 'H'), (13, 'H'), (14, 'H'), (2, 'T'), (3, 'T'), (4, 'T'), (5, 'T'), (6, 'T'), (7, 'T'), (8, 'T'), (9, 'T'), 
    (10, 'T'), (11, 'T'), (12, 'T'), (13, 'T'), (14, 'T'), (2, 'C'), (3, 'C'), (4, 'C'), (5, 'C'), (6, 'C'), (7, 'C'), (8, 'C'), (9, 'C'), 
    (10, 'C'), (11, 'C'), (12, 'C'), (13, 'C'), (14, 'C'),(2, 'P'), (3, 'P'), (4, 'P'), (5, 'P'), (6, 'P'), (7, 'P'), (8, 'P'), (9, 'P'), 
    (10, 'P'), (11, 'P'), (12, 'P'), (13, 'P'), (14, 'P')];

   
    
    //Get the player's First Card
    
    let mut rng = rand::thread_rng();
    let _n1: usize = rng.gen_range(0..51);
    let _card1 = deck[_n1];
    let (_rank1, _suit1) = _card1;
    let mut _rk: &str = "";
    if _rank1 > 10 {
        _rk = convert_rank(_rank1);
        println!("Your first card: {} - {} ", _rk, _suit1);

    } else {
        println!("Your first card: {} - {} ", _rank1, _suit1);
    }

    //Get the player's Second Card 

    let mut _n2: usize;
    loop {
        _n2 = rng.gen_range(0..51);
        if _n2 != _n1 {break; }
    }
    let _card2 = deck[_n2];
    let (_rank2, _suit2) = _card2;
    let mut _rk: &str = "";
    if _rank2 > 10 {
        _rk = convert_rank(_rank2);
        println!("Your second card: {} - {} ", _rk, _suit2);
    } else {
        println!("Your second card: {} - {} ", _rank2, _suit2);
    }

    //Get the dealer's First Card
    let mut _n3: usize;
    loop {
        _n3 = rng.gen_range(0..51);
        if _n3 != _n2 && _n3 != _n1 {break; }
    }
    let _card3 = deck[_n3];
    let (_rank3, _suit3) = _card3;

    //Get the dealer's Second Card
    let mut _n4: usize;
    loop {
        _n4 = rng.gen_range(0..51);
        if _n4 != _n1 && _n4 != _n2 && _n4 != _n3 {break; }
    }
    let _card4 = deck[_n3];
    let (_rank4, _suit4) = _card4;


    let mut player_pts1 = 0;
    let mut player_pts2;
    // Since the rank Ace can be 1 or 11
    if _rank1 == 14 || _rank2 == 14 {
        
        if _rank1 == 14 {
            if _rank2 > 10 { 
                black_jack(); 
            } else { 
            player_pts1 += _rank2;
            }
        } else {
            if _rank1 > 10 { 
                black_jack(); 
            } else { 
            player_pts1 += _rank1;
            }
        }
        player_pts1 += 1;
        player_pts2 = player_pts1 + 10;
        println!("You got an Ace card!");
        println!("Your possible points: {} and {}", player_pts1, player_pts2);
    } else {
        player_pts1 += point(_rank1);
        player_pts1 += point(_rank2);
        println!("Your current points {}", player_pts1);
        println!();
    }
    
    //Prompt player for additional cards
    let mut ch: char;
    loop {
        println!("Do you want to stand OR hit?");
        let mut answer = String::new();
        println!("Type stand/hit: ");
        std::io::stdin().read_line(&mut answer).unwrap();
        ch = answer.chars().nth(0).unwrap();
        if ch == 'h' || ch == 'H' { break;}

        //Player gets one more card

        let mut _n5: usize;
        loop {
            _n5 = rng.gen_range(0..51);
            if _n5 != _n1 && _n5!=_n2 && _n5!=_n3 && _n5!=_n4 {break; }
        }
        let _card5 = deck[_n5];
        let (_rank5, _suit5) = _card5;
        let mut _rk: &str = "";
        if _rank5 > 10 {
            _rk = convert_rank(_rank5);
            println!("Your extra card: {} - {} ", _rk, _suit5);
        } else {
            println!("Your extra card: {} - {} ", _rank5, _suit5);
        }

        //in case the player got an Ace before
        if _rank1 == 14 || _rank2 == 14 {
            player_pts1 += point(_rank5);
            if _rank1 == 14 { 
                player_pts2 = point(_rank5) + point(_rank2) + 10;
            } else {
                player_pts2 = point(_rank5) + point(_rank1) + 10;
            }
            
            println!("Your possible points: {} and {}", player_pts1, player_pts2);
            if player_pts1 > 20 || player_pts2 > 20 {
                status(player_pts2);
                println!();
            }
        } else {
            player_pts1 += point(_rank5);
            println!();
            println!("Your current points: {}", player_pts1);
            if player_pts1 > 20 {
                status(player_pts1);
            }
            println!();
        }
    }

    //Dealer's turn

    let mut dealer_point = point(_rank3) + point(_rank4);
    if dealer_point < 17 {
        while dealer_point < 21 {
            let mut _n6: usize;
            loop {
            _n6 = rng.gen_range(0..51);
            if _n6 != _n1 && _n6 != _n2 && _n6 != _n3 && _n6 != _n4 {break; }
            }
            let _card6 = deck[_n6];
            let (_rank6, _suit6) = _card6;
            dealer_point += point(_rank6);
        }
    } else {
        println!("Dealer did not take any extra card");
    }
    println!("Dealer's point: {}", dealer_point);
    println!();
    compare_points(player_pts1, dealer_point);
        
}

pub fn convert_rank(rank: i32) -> &'static str {
    if rank == 11 {
        return "Jack"; 
    } else if rank == 12 {
        return "Queen"; 
    } else if rank == 13 {
        return "King";
    } else {
        return "Ace";
    }
}

pub fn point(_r: i32) -> i32 {
    let mut point = 0;
    if _r < 11 {
        point += _r;
    } else  {
        point += 10;
    }
    return point;
}

pub fn black_jack() {
    println!("You got a BlackJack!");
    println!("Congratulations!");
}
pub fn status(point: i32) {
    if point == 21 {
        println!("Great! Let's hit and see what the Dealer got!");
    } else {
        println!("Oops! Hit and finger crossed!");
    }
}
pub fn compare_points(pl_point: i32, d_point : i32) {
    if pl_point < 21 {
        if d_point < 21 {
            if pl_point > d_point {
                println!("Congratulation! You have won this game!");
            } else {
                println!("Sorry! Your dealer has won!"); 
            }
        } else if d_point > 21 {
            println!("Congratulation! You have won this game!");
        } else {
            println!("Sorry! Your dealer has won!"); 
        }
    } else if pl_point == 21 {
        if d_point == 21 {
            println!("IT WAS A TIE!");
        } else {
            println!("Congratulation! You have won this game!");
        }
    } else {
        if d_point > 21 {
            println!("IT WAS A TIE!");
        } else {
            println!("Sorry! Your dealer has won!"); 
        }
    }
    println!("Your points: {} - Dealer points: {}", pl_point, d_point);
}