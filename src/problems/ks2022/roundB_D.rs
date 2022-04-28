struct Solution {}

impl Solution {
    fn gen_road(grid: Vec<Vec<char>>) -> Vec<Vec<bool>> {
        let res = grid.into_iter().flat_map(|row| {
            let tmp = row.into_iter().flat_map(|val| {
                match val {
                    '*' => [true, true],
                    '#' => [false, false],
                    _ => panic!("fatel input!")
                }
            }).collect::<Vec<bool>>();
            [tmp.clone(), tmp]
        }).collect::<Vec<Vec<bool>>>();
        res
    }

    fn walk_next(road: &mut Vec<Vec<bool>>, x: usize, y: usize, path: &mut Vec<char>) -> bool {
        println!("x: {x}\ty: {y}\tpath: {path:?}");
        // inputs
        // (x, y): current position

        // return status
        //  success: true
        //  failed: false
        
        // check it
        if !road[0][0] && x==0 && y==0 {
            // println!("Finished");
            return true;
        }

        road[y][x] = false;

        // try south
        if y<road.len()-1 {
            if road[y+1][x] {
                path.push('S');
                if Solution::walk_next(road, x, y+1, path) {
                   return true; 
                } else {
                    path.pop();
                }
            }
        }

        // try east
        if x<road[0].len()-1 {
            if road[y][x+1] {
                path.push('E');
                if Solution::walk_next(road, x+1, y, path) {
                   return true; 
                } else {
                    path.pop();
                }
            }
        }

        // try north
        if y>0 {
            if road[y-1][x] || (x==0 && y-1==0) {
                path.push('N');
                if Solution::walk_next(road, x, y-1, path) {
                   return true; 
                } else {
                    path.pop();
                }
            }
        }

        // try west
        if x>0 {
            if road[y][x-1] || (x-1==0 && y==0) {
                path.push('W');
                if Solution::walk_next(road, x-1, y, path) {
                   return true; 
                } else {
                    path.pop();
                }
            }
        }


        road[y][x] = true;
        false
    }

    pub fn hamiltonian_tour(grid: Vec<Vec<char>>) -> String {
        let expect_num = grid.iter().map(|rows| {
            rows.iter().filter(|&&val| val=='*').count()
        }).sum::<usize>() * 4;

        let mut road = Solution::gen_road(grid);
        println!("road: {road:?}");

        let mut path = Vec::new();

        Solution::walk_next(&mut road, 0, 0, &mut path);

        if path.len()==expect_num {
            return path.into_iter().collect::<String>()
        } else {
            return "IMPOSSIBLE".to_string();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = vec![vec!['*','*'],vec!['*','#']];
        let result = Solution::hamiltonian_tour(grid);

        println!("Result: {result:?}");
        assert!(result.len()>0);
    }

    #[test]
    fn case2() {
        let grid = vec![vec!['*','#','*']];
        let result = Solution::hamiltonian_tour(grid);

        println!("Result: {result:?}");
        assert!(result.len()>0);
    }

    #[test]
    fn case3() {
        let grid = vec![vec!['*','*','*','*'],vec!['*','#','*','#'],vec!['*','*','*','*']];
        let result = Solution::hamiltonian_tour(grid);

        println!("Result: {result:?}");
        assert!(result.len()>0);
    }

    #[test]
    fn case4() {
        let grid = vec![vec!['*','*','#','*'],vec!['*','*','#','*'],vec!['*','*','*','*']];
        let result = Solution::hamiltonian_tour(grid);

        println!("Result: {result:?}");
        assert!(result.len()>0);
    }
}