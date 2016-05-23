extern crate chrono;
extern crate num;
use num::traits::PrimInt;
use chrono::*;

pub fn after(datetime: DateTime<UTC>) -> DateTime<UTC> {
    let gigasecond = Duration::seconds(10.pow(9));
    datetime.checked_add(gigasecond).unwrap() 
}
