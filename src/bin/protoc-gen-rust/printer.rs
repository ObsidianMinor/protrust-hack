use std::fmt::Write;

pub struct Printer<W> {
    inner: W,
    indent: usize,
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

pub struct DocPrinter<'a, W> {
    inner: &'a mut Printer<W>,
    list_stack: Vec<Option<usize>>,
}

impl<'a, W> DocPrinter<'a, W> {
    pub fn new(inner: &'a mut Printer<W>) -> DocPrinter<'a, W> {
        DocPrinter {
            inner,
            list_stack: Vec::new()
        }
    }

    pub fn start_list(&mut self, start: Option<usize>) {
        self.list_stack.push(start)
    }

    pub fn end_list(&mut self) {
        self.list_stack.pop();
    }

    pub fn start_item(&mut self) {
        match self.list_stack.last_mut() {
            Some(Some(ref mut position)) => *position += 1,
            _ => { }
        }
    }

    pub fn end_item(&mut self) {
        match self.list_stack.last_mut() {
            Some(Some(ref mut position)) => *position -= 1,
            _ => { }
        }
    }

    pub fn current_item_number(&self) -> Option<usize> {
        match self.list_stack.last() {
            Some(x) => *x,
            None => None
        }
    }
}

impl<'a, W: Write> Write for DocPrinter<'a, W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let mut lines = s.split('\n');
        self.inner.write_str(lines.next().unwrap())?;
        let indent = "    ".repeat(self.inner.indent);
        let list_indent = "  ".repeat(self.list_stack.len());
        let inner = &mut self.inner.inner;

        while let Some(line) = lines.next() {
            inner.write_char('\n')?;
            inner.write_str(&indent)?;
            inner.write_str("/// ")?;
            inner.write_str(&list_indent)?;
            inner.write_str(line)?;
        }

        Ok(())
    }
} 