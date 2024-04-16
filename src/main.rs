use perf_event::events::Dynamic;
use perf_event::events::Hardware;
use perf_event::Builder;
use perf_event::Group;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let power_event = Dynamic::builder("power")
        .unwrap()
        .event("energy-pkg")
        .unwrap()
        .build()
        .unwrap();

    let mut counter_group = Group::new().unwrap();
    let retired_instructions_counter = counter_group
        .add(&Builder::new(Hardware::INSTRUCTIONS))
        .unwrap();
    let power_counter = counter_group.add(&Builder::new(Hardware::CPU_CYCLES)).unwrap();

    counter_group.enable().unwrap();
    let vec = (0..=49).collect::<Vec<_>>();
    println!("{:?}", vec);
    counter_group.disable().unwrap();

    let results = counter_group.read().unwrap();
    println!("Time elapsed: {:?}", start_time.elapsed());
    println!(
        "Instructions retired: {}",
        results[&retired_instructions_counter]
    );
    println!("Power consumed: {}", results[&power_counter]);
}
