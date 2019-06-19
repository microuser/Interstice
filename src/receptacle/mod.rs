pub mod campsite;
pub mod reservation;
pub mod search;

pub struct Receptacle {
    pub campsites : Vec<self::campsite::Campsite> ,
    pub reservations : Vec<self::reservation::Reservation> ,
    pub search : self::search::Search ,
}

pub fn tell_receptalce(){

    println!("tell receptacle()");
}


