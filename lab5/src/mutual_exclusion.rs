use std::collections::HashSet;

pub const NUSIZE: usize = 3;
pub const NU8: u8       = NUSIZE as u8;

macro_rules! is_illegal {
    ($ps:ident) => {{
        let mut count = u8::from($ps[0] == $ps[NUSIZE - 1]);

        for i in 1..NUSIZE {
            if $ps[i - 1] != $ps[i] {
                count += 1;
                if count == 2 {
                    return true
                }
            }
        }

        false
    }};
}

fn gen_configs_helper(i: usize, acc: HashSet<[u8; NUSIZE]>) -> HashSet<[u8; NUSIZE]> {
    if i == NUSIZE {
        acc
    } else {
        let new_acc: HashSet<[u8; NUSIZE]>  =
            (0..=NU8)
            .flat_map(|j| {
                acc
                .iter()
                .map(move |&ps| {
                    let mut new_ps = ps;
                    new_ps[i] = j;
                    new_ps
                })
            })
            .collect();

        gen_configs_helper(i + 1, new_acc)
    }
}

pub fn gen_all_configs() -> HashSet<[u8; NUSIZE]> {
    let timer = std::time::Instant::now();
    let mut set = HashSet::new();
    set.insert([0; NUSIZE]);
    let configs = gen_configs_helper(1, set);
    println!("gen_all_configs time elapsed: {:?}", timer.elapsed());
    configs
}

pub fn max_steps() -> usize {
    let all_configs = gen_all_configs();
    println!("all_configs.len(): {}", all_configs.len());

    let mut configs: HashSet<[u8; NUSIZE]> = 
        all_configs
        .into_iter()
        .filter(|config| is_illegal!(config))
        .collect();

    let mut steps = 0;

    while !configs.is_empty() {
        let mut new_configs = HashSet::new();

        for config in configs {
            if config[0] == config[NUSIZE - 1] {
                let mut new_one = config;
                new_one[0] = (new_one[0] + 1) % (NU8 + 1);
                new_configs.insert(new_one);
            }
            
            (1..NUSIZE).for_each(|i| {
                if config[i - 1] != config[i] {
                    let mut new_one = config;
                    new_one[i] = new_one[i - 1];
                    new_configs.insert(new_one);
                }
            });
        }

        configs =
            new_configs
            .into_iter()
            .filter(|config| is_illegal!(config))
            .collect();

        steps += 1;
        println!("finished step {}", steps)
    }

    steps
}