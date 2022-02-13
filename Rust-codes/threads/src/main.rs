use std::thread;
use std::time::Duration;

fn main() {
    
  let handle = thread::spawn(||{   // spawn thread
        for i in 1..10{
         println!("Second process = {}",i);
            thread::sleep(Duration::from_millis(500));
        }
   });

   thread::spawn(||{   // spawn thread
    for k in 1..10{
     println!("Third process = {}",k);
        thread::sleep(Duration::from_millis(500));
    }
});
    
    for j in 1..10 {     // main thread

       println!("First process = {}", j);
       thread::sleep(Duration::from_millis(500));
    }

   

    let new = vec![1,2,3,4,5,6,7,8,9,10];

    let handle_2 = thread::spawn(move ||{   // spawn thread
        println!("vector = {:?}",new);
        thread::sleep(Duration::from_millis(500));
        
   });
   
   //handle_2.join().unwrap();
   handle.join().unwrap();
}
