use std::{sync::{Arc,RwLock}, thread, time::{Duration, SystemTime}};
use rand::Rng;
#[derive(Debug)]
pub struct Data{
    pub id:i32,
    pub name:String,
    pub time_stamp:SystemTime
}

pub fn thread_task(){
    // let (tx,rx)=mpsc::channel();
    let shared_resource = RwLock::new(Vec::new());
    let arc=Arc::new(shared_resource);
    let ref1=Arc::clone(&arc);
    let ref2=Arc::clone(&arc);
    let ref3=Arc::clone(&arc);
    let count=1;
    // let count_arc=Arc::new(count);
    

    let t1=thread::spawn(move ||loop {
        thread::sleep(Duration::from_secs(1));
        match ref2.write(){
            Ok(mut write_data)=>{
                let (rnd_name,rnd_value) =generate_random_name_and_id();
                write_data.push(Data{id:rnd_value,name:rnd_name,time_stamp:SystemTime::now()});
                println!("Data Added");
            }
            Err(error)=>{
                println!("{:?}",error);
            }
        }
    });

    let t2=thread::spawn(move ||loop {
        thread::sleep(Duration::from_secs(5));
        println!("{:#?}",ref1.read().unwrap().len());
    });    
    
    
    let t3=thread::spawn(move ||loop {
        thread::sleep(Duration::from_secs(15));
        match ref3.write(){
            Ok(mut read_data)=>{
                read_data.retain(|each_data|{
                    match each_data.time_stamp.elapsed(){
                        Ok(time)=>{
                            // println!("Entries before 5 sec removed!");
                            time<Duration::from_secs(5)
                        }
                        Err(error)=>{
                            println!("{:?}",error);
                            false
                        }
                    }
                    
                })
            }   
            Err(error)=>{
                println!("{:?}",error);
            }
        }
        // SystemTime::now().duration_since()
    });
    println!("{:#?}",arc);
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}

pub fn generate_random_name_and_id()->(String,i32){
    let names = vec![
        "Jasmine",
        "Liam",
        "Isabella",
        "Noah",
        "Sophia",
        "Benjamin",
        "Olivia",
        "William",
        "Emma",
        "James",
        "Ava",
        "Alexander",
        "Mia",
        "Ethan",
        "Charlotte",
        "Jacob",
        "Amelia",
        "Michael",
        "Harper",
        "Elijah",
    ];
    let rnd_value =rand::thread_rng().gen_range(0..100);

    let rnd_index =rand::thread_rng().gen_range(0..names.len());
    (names[rnd_index].to_string(),rnd_value)
    
}