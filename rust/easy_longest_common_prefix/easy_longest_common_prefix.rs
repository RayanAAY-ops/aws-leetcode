impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs.get(0).expect("vector is empty").clone(); // Clone the first string to get a mutable String
        let mut prefix_len = prefix.len();

        for word in strs.iter().skip(1) {
            // Loop until prefix matches the start of the current word, character by character

            while prefix != &word[0..prefix_len.min(word.len())] {
                prefix_len -= 1; // Decrease the length of the prefix

                // Update prefix to the shortened version (ensure the slice is valid)
                prefix = prefix[0..prefix_len.min(word.len())].to_string(); // Convert slice back to String

                if prefix_len == 0 {
                    return "".to_string(); // If no common prefix, return an empty string
                }
            }
        }

        prefix
    }
}
