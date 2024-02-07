use self::{
    bifergation::bifergation,
    data::{CS_PROVIDERS, PENDING_QUERIES},
    data_randomizer::skill_changer_service,
    monitoring::priority_checker,
    random_data_gen::random_data_gen,
    task_assigner::task_assigner,
};
use std::thread;

pub mod bifergation;
pub mod data;
pub mod data_randomizer;
pub mod monitoring;
pub mod random_data_gen;
pub mod task_assigner;

pub fn task_scheduler() {
    let t1 = thread::spawn(move || {
        random_data_gen();
    });

    let t2 = thread::spawn(move || {
        task_assigner();
    });
    let t3 = thread::spawn(move || {
        bifergation();
    });
    let t4 = thread::spawn(move || {
        priority_checker();
    });
    let t5 = thread::spawn(move || {
        skill_changer_service();
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();
    t5.join().unwrap();
    // println!("{:?}",USER_QUERIES.read().unwrap());
    // println!("{:?}",CS_PROVIDERS.read().unwrap());
    // loop {
    //     thread::sleep(Duration::from_secs(2));
    // }
}
