pub struct Entry {
    // The integer here is a very rough priority ranking indicating
    // the commonness of the word, specifically in that form.  A
    // lower numerical value indicates a more common word.
    pub keys: Vec<(String, u32)>,
    pub definition: String,
}
