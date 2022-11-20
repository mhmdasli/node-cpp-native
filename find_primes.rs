use std::time::Instant;
fn is_prime(num: i32)-> bool {
    if num <= 1 {
      return false;
    }
    let mut i: i32 = 2;
    loop{
        if i<=num/2{
            i=i+1;
            if num % i == 0 {
                return false;
              }
            continue;
        }
        else{
            break;
        }
    } 
    return true;
  }
  
  fn find_prime(num: i32)-> i32 {
    let mut largest_prime: i32 = 0;
    let mut j: i32 = 2;
    loop{
        if j<=num{
            j=j+1;
            if is_prime(j) {
                largest_prime = j;
            }
            continue;
        }
        else{
            break;
        }
    }
    return largest_prime;
  }

fn main(){
    let start = Instant::now();
    let prime = find_prime(500000);
    let elapsed = start.elapsed();
    println!("prime = {}", prime);
    println!("elapsed: {:?}", elapsed);
}