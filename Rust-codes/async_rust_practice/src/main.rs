use futures::executor::block_on;
//use std::time::Instant;
use std::time::Duration;
use async_std::task;
//use std::thread;
//let song: Song;

fn main() {
    // println!("Hello, world!");
    let task = async_main();
    block_on(task);
   // let future = hello_world();
   // block_on(future);

//    block_on(learn_song());
//    block_on(sing_song());
//    block_on(dance());

//block_on(async_main());
}
 
async fn async_main(){
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1,f2);
}

// async fn hello_world(){
//     task::sleep(Duration::from_secs(10)).await;
//     println!("Hello, Async rust programing for containerized applications......!");
// }

async fn learn_song() {//-> Song {
    println!("Learning singing the song..........!!!\n");
    task::sleep(Duration::from_secs(5)).await;
    println!("Stop Learning the Song.....\n");
}

async fn sing_song(){//song: Song){
    println!("Singing the song now.............!!!!!!");
    task::sleep(Duration::from_secs(5)).await;
}

async fn dance(){
    println!("Dancing started on the notes of song..........!!!!!!!!!!!!!!!\n");
    task::sleep(Duration::from_secs(5)).await;
    println!("Stop the Dance............!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n");
} 

async fn learn_and_sing(){
    learn_song().await;
    println!("Start sing song....");
    sing_song().await;//song).await;
    println!("End sing song....");
}


