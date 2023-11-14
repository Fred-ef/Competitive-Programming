pub fn max_attractions(attractions: &Vec<Vec<u32>>, cities: usize, days: u32) -> u32 {
    // Create a 3D table to store the maximum attractions for each city, day, and activities combination
    let mut tab = vec![vec![vec![0; days as usize + 1]; days as usize + 1]; cities];

    // Initialize the first column with attractions for the first city
    for d in 1..=days {
        tab[0][d as usize][0] = attractions[0][(d - 1) as usize];
    }

    // Fill in the dynamic programming table
    for city in 1..cities {
        for d in 1..=days {
            for i in 0..=d {
                // Initialize with attractions of the current city
                tab[city][d as usize][i as usize] = attractions[city][0];
            }
            for j in 1..=d {
                for k in 1..= {
                    // Calculate attractions for each combination
                    let current_attractions = tab[city][d as usize][i as usize];
                    let previous_attractions = tab[city - 1][(d - j) as usize][k - 1];
                    let total_attractions = current_attractions + previous_attractions;
                    tab[city][d as usize][i as usize] = tab[city][d as usize][i as usize].max(total_attractions);
                }
            }
        }
    }

    // Find the maximum attractions in the last column
    let mut max_attractions = 0;
    for i in 0..=days {
        max_attractions = max_attractions.max(tab[cities - 1][days as usize][i as usize]);
    }

    max_attractions
}