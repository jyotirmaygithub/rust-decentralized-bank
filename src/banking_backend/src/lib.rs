use candid::types::result;
use ic_cdk::export_candid;

static mut COUNT: i32 = 1;
static mut AMOUNT: i32 = 10000;

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
fn withdraw(num: i32) -> i32 {
    unsafe {
        if AMOUNT >= num {
            AMOUNT = AMOUNT - num;
            ic_cdk::println!("remaining amount {}", AMOUNT);
            AMOUNT
        } else {
            ic_cdk::println!("withdrawing account is more than what you have");
            AMOUNT
        }
    }
}

#[ic_cdk::query]
fn current_amount() -> i32 {
    unsafe { AMOUNT }
}
export_candid!();
