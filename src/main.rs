type RegexIndex = usize;

const REGEX_COLUMN_SIZE: usize = 130;
const REGEX_END: usize = 129;

#[derive(Default, Clone, Copy)]
struct RegexAction {
    next: RegexIndex,
    offset: i32,
}

#[derive(Clone)]
struct RegexColumn {
    ts: [RegexAction; REGEX_COLUMN_SIZE],
}    

impl RegexColumn {
    fn new() -> Self {
        Self {
            ts: [Default::default(); REGEX_COLUMN_SIZE],
        }
    }
}

struct Regex {
    cs: Vec<RegexColumn>,
}

impl Regex {
    fn must_compile(src: &str) -> Self {
        let mut res = Self { cs: Vec::new() };
        res.cs.push(RegexColumn::new());
        for c in src.chars()  {
            let mut col = RegexColumn::new();

            match c {
                '$' => {
                    col.ts[REGEX_END] = RegexAction {
                        next: res.cs.len() + 1,
                        offset: 1,
                    };
                    res.cs.push(col);
                },
                '.' => {
                    for i in 32..127 {
                        col.ts[i] = RegexAction {
                            next: res.cs.len() + 1,
                            offset: 1,
                        };
                    }
                    res.cs.push(col);
                }
                '*' => {
                    let map_size = res.cs.len();
                    for t in res.cs.last_mut().unwrap().ts.iter_mut() {
                        if t.next == map_size {
                            t.next = map_size - 1;
                        } else if t.next == 0 {
                            t.next = map_size;
                            t.offset = 0;
                        } else {
                            unreachable!();
                        }
                    }
                }
                '+' => {
                    let map_size = res.cs.len();
                    res.cs.push(res.cs.last().unwrap().clone());
                    for t in res.cs.last_mut().unwrap().ts.iter_mut() {
                        if t.next == map_size {
                            continue;
                        } else if t.next == 0 {
                            t.next = map_size + 1;
                            t.offset = 0;
                        } else {
                            unreachable!();
                        }
                    }
                }
                _ => {
                    col.ts[c as usize] = RegexAction {
                        next: res.cs.len() + 1,
                        offset: 1,
                    };
                    res.cs.push(col);
                }
            }
        }
        res
    }

    fn match_str(&mut self, recv_str: &str) -> bool{
        let mut state = 1;
        let mut head = 0;
        let chars = recv_str.chars().collect::<Vec<_>>();

        while 0 < state && state < self.cs.len() && head < chars.len() {
            let action = self.cs[state].ts[chars[head] as usize];

            state = action.next;
            head = (head as i32 + action.offset) as usize;
        }
        
        if state == 0 {
            return false;
        }
        if state < self.cs.len() {
            let action = self.cs[state].ts[REGEX_END as usize];

            state = action.next;
        }

        return state >= self.cs.len()
    }

    fn dump_regex_map(&self) {
        for sym in 0..REGEX_COLUMN_SIZE {
            print!("{:03} => ", sym);
            for column in self.cs.iter() {
                print!("({}, {}) ", column.ts[sym].next, column.ts[sym].offset);
            }
            println!();
        }
    }
}

fn main() {
    let mut regex = Regex::must_compile("a+bc$");
    println!("----------------START OF REGEX MAP----------------");
    regex.dump_regex_map();
    println!("-----------------END OF REGEX MAP-----------------");

    let inputs = vec!["Hello!", "abc", "bc", "abcd", "abcdefghij"];
    for input in inputs.iter() {
        println!("{:?} => {:?}", input, regex.match_str(input))
    }
}