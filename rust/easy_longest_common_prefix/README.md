The choice of `Vec<String>` over `Vec<&str>` is often influenced by the nature of ownership and borrowing in Rust. Let's break down why `Vec<String>` is still a practical choice here:

### 1. **Function Signature Simplicity**:
   - By using `Vec<String>`, the function takes ownership of the strings in the vector. This avoids the need for complex lifetime annotations, which would be required if you used `Vec<&str>` since the `&str` references must live as long as the `strs` vector.
   - Using `Vec<String>` allows the function to avoid worrying about where the original strings came from or how long they'll remain valid.

### 2. **Flexibility with `String`**:
   - Even though you're not modifying the `strs` vector itself, you are working with strings derived from it (e.g., creating a new variable `prefix`).
   - With `String`, you can clone, slice, and manipulate the string easily without needing to juggle lifetimes or mutability constraints of borrowed data.

### 3. **Interoperability**:
   - A `Vec<String>` is often easier to work with than a `Vec<&str>` in functions that might require creating or returning owned `String` objects. This is because `String` can be easily converted to `&str` when needed, but the reverse requires allocation.
   - Using `Vec<&str>` would limit flexibility, especially if there's any chance the function might need to return a `String` or pass an owned string to another function.

### Why Not `Vec<&str>` in This Case?
While it's true that you're not modifying `strs` directly, `Vec<&str>` would impose additional constraints:
- The lifetime of each `&str` in the vector would need to be tied to the lifetime of the `strs` argument.
- If `strs` came from owned `String`s (like `vec!["flower".to_string(), "flow".to_string()]`), you'd have to convert them to slices first, which is inconvenient.

### Conclusion:
Even though you're not modifying `strs` directly, using `Vec<String>` simplifies the function's usability and avoids lifetime complexities. It's a pragmatic choice, aligning with Rust's ownership model while providing flexibility and clarity. If you're confident `Vec<&str>` works in your case, you could switch, but you'll need to ensure the lifetimes align correctly.