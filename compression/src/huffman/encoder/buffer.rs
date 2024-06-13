pub struct Buffer {
    data : Vec::<u8>,
    head : usize
}

impl Buffer {
    
    pub fn new(mut data : Vec<u8>) -> Self {
        data.push(0);
        Self { data : data, head : 7}
    }

    // pub fn push(&mut self, byte : u8){

    //     let len = self.data.len() - 1;

    //     if self.data[len] == 0 && self.head == 7 {
    //         self.data[len] = byte
    //     } else {
    //         self.data.push( byte );
    //     }

    //     self.data.push( 0 );
    //     self.head = 7;
    // }

    pub fn write(&mut self, encoding : &mut u32 ) {

        // Determine how far to shift the encoding
        let font : u32 = encoding.leading_zeros() + 1;
        let bits : u32 = 32 - font ;
        let diff : u32 =  (((8 + self.head + 1) as u32) - bits) % 8;
        
        // Raise an error if shifting the bit would cause an overflow
        // This is fixable but is not woth the effort right now
        if diff > font {
            panic!(
            "Encoding cannot be coerced into the buffer without information loss"
            )
        }

        // Clear the lead bit and shift
        *encoding &= !(0b1 << bits); 
        *encoding <<= diff ;

        // Split the encoding into u8 bytes
        let bytes = encoding.to_le_bytes();

        // Start from the little end and count bytes to ignore
        let mut index : usize = 3;
        for (i, byte) in bytes.iter().enumerate().rev() {
            match (i == 0, *byte == 0) {
                (true, _) =>  index = 0 ,
                (false, true) => {},
                (false, false) => { index = i; break; }
            }
        }

        // Join the two mismatched bytes 
        let len = self.data.len() - 1;
        self.data[len] |= bytes[index];

        // Add the normal 
        for j in (0..index).rev(){
            self.data.push( bytes[j] )
        }


        match diff.checked_sub(1) {
            Some(d) => self.head = d as usize,
            None => { self.head = 7; self.data.push(0) }
        }

    }

    pub fn get_data(& self) -> &Vec<u8> {
        &self.data
    }


}