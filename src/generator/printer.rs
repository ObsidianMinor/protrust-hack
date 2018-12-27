use std::fmt::Write;

pub struct Printer<W> {
    inner: W,
    indent: usize
}

impl<W> Printer<W> {
    pub fn new(inner: W) -> Printer<W> {
        Printer { inner, indent: 0 }
    }

    pub fn indent(&mut self) {
        self.indent += 1
    }

    pub fn unindent(&mut self) {
        self.indent -= 1
    }

    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: Write> Write for Printer<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let mut lines = s.split('\n');
        self.inner.write_str(lines.next().unwrap())?; // split returns at least one item

        let indent = "    ".repeat(self.indent);

        while let Some(line) = lines.next() {
            self.inner.write_char('\n')?;
            self.inner.write_str(&indent)?;
            self.inner.write_str(line)?;
        }

        Ok(())
    }
}