fn main() {
    let map = OurMap::new();
    map.insert(1,20);
}

struct OurMap {} // struct camelcase, fun snake case, file hyphen

impl OurMap {
        fn new() -> OurMap { // so we have a function that returns a map to us
            OurMap {}
        }     

        fn insert(self, key: u64, value: i8){}
    }

   
 /* So right now this a associated, or static function.. self changes this
     u stands for unsigned, and i stands for signed : assigning types
     apparently there is a way to make it decide for us... 



     () this is the unit type its simiar to void except its a value it represent nothing*/ 
 