pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut updated_minefield: Vec<String> = Vec::new();
    let mut vec_minefield: Vec<Vec<u8>> = Vec::new();
    
    for i in 0..minefield.len() {
        let row = match minefield.get(i) {
            None => "",
            Some(s) => s,
        };
        let mut curr_line: Vec<u8> = Vec::new();
        for j in 0..row.len() {
            let curr = row.chars().nth(j).unwrap();

            if curr == '*' {
                curr_line.push(1);
            } else {
                curr_line.push(0);
            }
        }
        vec_minefield.push(curr_line);
    }

    for i in 0..minefield.len() {
        let row = match minefield.get(i) {
            None => "",
            Some(s) => s,
        };
        // with_capacity is basically size, so runtime allocation does not take place
        let mut curr_line: String = String::with_capacity(row.len());
        for j in 0..row.len() {
            let curr = row.chars().nth(j).unwrap();
            if curr != '*' {
                curr_line.push(count_neighbouring_mines(&vec_minefield, i, j, minefield.len(), row.len()));
            } else {
                curr_line.push('*')
            }
        }
        println!("{:?}", curr_line);
        updated_minefield.push(curr_line)
    }
    updated_minefield
}

pub fn count_neighbouring_mines(vec_minefield: &Vec<Vec<u8>>, i: usize, j: usize,
    m_size: usize, n_size: usize) -> char {
    let mut count: u8 = 0;
    for m in -1..=1 {
        for n in -1..=1 {
            if !(m == 0 && n == 0) {
                print!("{:?}, {:?} | ", m, n);
                let x = m + i as i32;
                let y = n + j as i32;
                if is_valid(x, y, m_size as i32, n_size as i32) {
                    count += vec_minefield[x as usize][y as usize];
                }
            }
        }
    }
    if count == 0 {
        return ' '
    }
    (48 + count) as char
}

pub fn is_valid(i: i32, j: i32, x: i32, y: i32) -> bool {
    if i < 0 || i >= x {
        return false
    }
    if j < 0 || j >= y {
        return false
    }
    return true
}