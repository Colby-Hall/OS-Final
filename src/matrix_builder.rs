use rand::distributions::{Distribution, Uniform};


pub struct Matrix {
    pub row_number: usize,
    pub column_number: usize,
}


impl Matrix {
    pub fn fill_matrix(self) -> Vec<Vec<usize>> {
        let mut two_d_array = vec![vec![0usize; self.row_number]; self.column_number];

        let range = Uniform::from(1..1000);
        let mut rng = rand::thread_rng();

        for i in 0..self.row_number {
            for j in 0..self.column_number {
                let number = range.sample(&mut rng);

                two_d_array[i][j] = number;
            }
        }
        /*
        for x in two_d_array.iter() {
            println!("{:?}", x);


        }
        */
        
        

        println!("-----");
        return two_d_array;
    }
}
