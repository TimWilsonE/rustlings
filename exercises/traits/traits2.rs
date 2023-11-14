// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

// This is what I think it would mean to append "Bar"
// impl AppendBar for String {
//     fn append_bar(&self) -> String {
//         self.to_owned() + "Bar"
//     }
// }
// impl<T: AppendBar> AppendBar for Vec<T> {
//     fn append_bar(&self) -> Vec<T> {
//         self.iter().map(|item| {item.append_bar()}).collect()
//     }
// }

// This is silly. Why are we returning self if the function mutates it?
// And why can this append_bar have mut self, whereas the train just has self?
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Vec<String> {
        self.push(String::from("Bar"));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
