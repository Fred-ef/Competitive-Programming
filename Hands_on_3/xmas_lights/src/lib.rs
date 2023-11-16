pub fn patriotic_selections(s: &str) -> u64 {
    let mut count_r = 0;
    let mut count_rw = 0;
    let mut count_rwg = 0;
    let mut count_x = 0;

    for ch in s.chars() {
        match ch {
            'R' => {
                count_r = count_r + 3_u64.pow(count_x);
            }
            'W' => {
                count_rw = count_rw + count_r;
            }
            'G' => {
                count_rwg = count_rwg + count_rw;
            }
            'X' => {
                count_rwg = count_rwg * 3 + count_rw;
                count_rw = count_rw * 3 + count_r;
                count_r = count_r * 3 + 3_u64.pow(count_x);

                count_x += 1;
            }
            _ => (),
        }
    }

    count_rwg
}
