//! machdown-parser
//! Lossless line and document parser for Markdown.

/// Represents the line ending style for a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    None,
    Lf,
    CrLf,
}

impl LineEnding {
    pub fn as_str(&self) -> &'static str {
        match self {
            LineEnding::None => "",
            LineEnding::Lf => "\n",
            LineEnding::CrLf => "\r\n",
        }
    }

    pub fn len(&self) -> usize {
        match self {
            LineEnding::None => 0,
            LineEnding::Lf => 1,
            LineEnding::CrLf => 2,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Represents a single line in a parsed Markdown document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    line_number: usize,
    byte_offset: usize,
    content: String,
    ending: LineEnding,
}

impl Line {
    pub fn new(
        line_number: usize,
        byte_offset: usize,
        content: String,
        ending: LineEnding,
    ) -> Self {
        Self {
            line_number,
            byte_offset,
            content,
            ending,
        }
    }

    /// 1-indexed line number in the document.
    pub fn line_number(&self) -> usize {
        self.line_number
    }

    /// Starting byte offset in the original document.
    pub fn byte_offset(&self) -> usize {
        self.byte_offset
    }

    /// Content of the line excluding the line ending.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// The line ending of this line.
    pub fn ending(&self) -> LineEnding {
        self.ending
    }

    /// Total byte length of the line including the line ending.
    pub fn byte_len(&self) -> usize {
        self.content.len() + self.ending.len()
    }
}

/// Represents a parsed Markdown document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    source: String,
    lines: Vec<Line>,
}

impl Document {
    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn lines(&self) -> &[Line] {
        &self.lines
    }
}

/// Parse source Markdown string into a lossless `Document`.
pub fn parse(source: &str) -> Document {
    let mut lines = Vec::new();
    let mut current_offset = 0;
    let mut line_number = 1;

    let bytes = source.as_bytes();
    let mut line_start = 0;
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'\n' {
            let ending = if i > line_start && bytes[i - 1] == b'\r' {
                LineEnding::CrLf
            } else {
                LineEnding::Lf
            };

            let content_end = if ending == LineEnding::CrLf { i - 1 } else { i };
            let content = source[line_start..content_end].to_string();
            let byte_len = i + 1 - line_start;

            lines.push(Line::new(line_number, current_offset, content, ending));

            current_offset += byte_len;
            line_number += 1;
            line_start = i + 1;
        }
        i += 1;
    }

    // Handle last line if there's remaining content or if document was empty
    if line_start < bytes.len() {
        let content = source[line_start..].to_string();
        lines.push(Line::new(
            line_number,
            current_offset,
            content,
            LineEnding::None,
        ));
    } else if source.is_empty() {
        // Empty document has 0 lines or 1 empty line? Let's check empty source
        // An empty file has 0 lines
    }

    Document {
        source: source.to_string(),
        lines,
    }
}
