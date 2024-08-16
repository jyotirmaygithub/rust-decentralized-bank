use candid::{types::result, Nat};
use ic_cdk::export_candid;

static mut COUNT: i32 = 1;

// these are the macros which we need to write so that they can be expose at the time of building the candid file.
#[ic_cdk::update]
// the function to increment and decrement is working fine..
fn greet() {
    unsafe {
        COUNT += 2;
        ic_cdk::println!("Updated count: {}", COUNT);
    }
    decrement() 
}

fn decrement() {
    unsafe {
        COUNT -= 1;
        ic_cdk::println!("value depreciation {} ", COUNT)
    }
}

#[ic_cdk::update]
fn withdraw(num: i32){
    let amount :i32 = 10000;
    let  result :i32 = amount - num;
    ic_cdk::println!("remaining amount {}", result);
}

// we have a another intersting stuff in icp which is off stable which kind of stores the data even after refreshing.
export_candid!();