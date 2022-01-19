#![allow(dead_code, unused_variables)]

pub struct UnOrderedArray {
    m_array: Vec<i32>,
}

impl UnOrderedArray {
    pub fn new() -> UnOrderedArray {
        UnOrderedArray {
            m_array: Vec::new(),
        }
    }

    pub fn add_last(&mut self, item: i32) {
        self.m_array.push(item)
    }

    pub fn linear_search(&self, item: i32) -> i32 {
        for i in 0..self.m_array.len() {
            if self.m_array[i] == item {
                return i as i32;
            }
        }
        return -1;
    }

    pub fn selection_sort(&mut self) {
        let end = self.m_array.len();

        for start in 0..end {
            let index_of_min = start;
            for i in start..end {
                if self.m_array[i] < self.m_array[index_of_min] {
                    let temp = self.m_array[index_of_min];
                    self.m_array[index_of_min] = self.m_array[i];
                    self.m_array[i] = temp;
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut x = String::new();

        for i in 0..self.m_array.len() {
            x += format!("{} ", self.m_array[i as usize]).as_str();
        }
        x
    }
}
