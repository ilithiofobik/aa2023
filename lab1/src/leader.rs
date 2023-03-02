use rand::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Null,
    Single,
    Collision
}

fn slot (n: usize, p_inv: usize) -> State {
    let mut rng = rand::thread_rng();
    let mut state = State::Null;

    for _ in 0..n {
        let r = rng.gen_range(0..p_inv);
        if r == 0 {
            match state {
                State::Null      => state = State::Single,
                State::Single    => { state = State::Collision; break }
                State::Collision => ()
            }
        }
    }    
    
    state
}

pub fn leader_exact(n: usize) -> usize {
    let mut state = State::Null;
    let mut counter = 0;

    while state != State::Single {
        counter += 1;
        state = slot(n, n);
    }
    
    counter
}

pub fn leader_bound(n: usize, u: usize) -> usize {
    let mut state = State::Null;
    let mut counter = 0;
    let mut p_inv = 2;

    while state != State::Single {
        counter += 1;
        state = slot(n, p_inv);
        if p_inv >= u {
            p_inv = 2;
        } else {
            p_inv *= 2;
        }
    }
    
    counter
}

pub fn leader_round(n: usize, l: usize) -> usize {
    for i in 1u32..=l as u32 {
        let p_inv = 2usize.pow(i);
        if slot(n, p_inv) == State::Single {
            return 1
        }
    }

    0
}