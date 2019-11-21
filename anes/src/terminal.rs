//! A terminal related ANSI escape sequences.

sequence!(
    /// Resizes the text area to the given width and height in characters.
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::ResizeTextArea;
    ///
    /// let mut stdout = stdout();
    /// // Resize the terminal to 80x25
    /// write!(stdout, "{}", ResizeTextArea(80, 25));
    /// ```
    struct ResizeTextArea(u16, u16) =>
    |this, f| write!(f, csi!("8;{};{}t"), this.1, this.0)
);

#[cfg(test)]
test_sequences!(
    resize_text_area(
        ResizeTextArea(80, 25) => "\x1B[8;25;80t",
        ResizeTextArea(1, 1) => "\x1B[8;1;1t",
    ),
);
