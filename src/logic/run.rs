use rand::seq::IndexedRandom;

use crate::prelude::*;

pub struct LogicRun<'a> {
    cards: &'a Cards,
    cards_vec: CardsAsVec<'a>,
    session: SessionRuntime,
}

impl<'a> LogicRun<'a> {
    pub fn new(dn: &'a AggrDyn<'a>, key: &'a String) -> Self {
        let cards = dn.cards_map.get(key).expect("not cards in map");
        Self {
            cards: cards,
            session: SessionRuntime::new(),
            cards_vec: cards.to_vec(),
        }
    }

    fn ask_answ<'b>(cards_vec: &'b CardsAsVec) -> Option<&'b (&'b String, &'b Card)> {
        cards_vec.choose(&mut rng())
    }

    fn eq(answer_user: String, card: &&Card) -> bool {
        &card.answer == &answer_user
    }

    fn cli_impl(eq: bool, session: &mut SessionRuntime, card: &&Card) {
        if eq {
            println!("right");
            print!("+");
            session.points += 1;
        } else {
            println!("wrong");
            print!("-");
            session.points -= 1;
        }
        print!("1 point\n\n");
        println!("explanation:\n{}\n\n", card.explanation);
    }

    pub fn interaction(&mut self) -> impl Future<Output = RResult<()>> {
        async move {
            loop {
                let (key, card) = Self::ask_answ(&self.cards_vec).expect("not find key in data");
                println!("next card:");
                println!("{key}\n\n{}", card.ask);
                let cli = get_cli_cycle::<RunCli>(true).await?;
                if cli.c.is_none() {
                    
                } 
                if let Some(command) = cli.c {
                    match command {
                        RunSub::Exit => break,
                    }
                } else {
                    let eq = if let Some(answer_user) = cli.answer {
                        Self::eq(answer_user, card)
                    } else {false};
                    println!("response accepted:\n");
                    Self::cli_impl(eq, &mut self.session, card);
                }
            }
            let session_stat = self.session.clone().to_stat(self.cards.info.clone());
            println!("{session_stat}");
            session_stat.push_file()?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude_tests::prelude::*;

    #[test]
    fn cli_impl_res_1() {
        let answer = "answer".to_string();
        let mut session = SessionRuntime::new();
        let card = Card::new(
            "key".to_string(),
            "ask".to_string(),
            "answer".to_string(),
            "explanation".to_string(),
        );
        LogicRun::cli_impl(LogicRun::eq(answer, &&card), &mut session, &&card);
        assert_eq_pr!(session.points, 1);
    }
}
