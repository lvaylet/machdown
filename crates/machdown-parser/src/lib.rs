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

/// Source span with byte offsets and 1-indexed line/column coordinates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

impl Span {
    pub fn new(
        start_byte: usize,
        end_byte: usize,
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
    ) -> Self {
        Self {
            start_byte,
            end_byte,
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }
}

/// Represents a YAML frontmatter block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontmatterNode {
    pub raw_content: String,
    pub span: Span,
}

impl FrontmatterNode {
    pub fn raw_content(&self) -> &str {
        &self.raw_content
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

/// Obsidian Wiki-Link node (`[[target]]` or `[[target|alias]]`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WikiLinkNode {
    pub target: String,
    pub alias: Option<String>,
    pub span: Span,
}

/// Obsidian Callout block (`> [!type] Title`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalloutNode {
    pub kind: String,
    pub title: Option<String>,
    pub content: Vec<Node>,
    pub span: Span,
}

/// Obsidian Block Identifier (`^block-id-123`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockIdNode {
    pub id: String,
    pub span: Span,
}

/// Inline Math (`$E=mc^2$`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineMathNode {
    pub math: String,
    pub span: Span,
}

/// Display Math (`$$...$$`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayMathNode {
    pub math: String,
    pub span: Span,
}

/// Tag (`#project/alpha`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagNode {
    pub tag: String,
    pub span: Span,
}

/// Heading node (`# Heading`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadingNode {
    pub level: u8,
    pub title: String,
    pub span: Span,
}

/// Node in document AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Frontmatter(FrontmatterNode),
    Heading(HeadingNode),
    Callout(CalloutNode),
    DisplayMath(DisplayMathNode),
    WikiLink(WikiLinkNode),
    InlineMath(InlineMathNode),
    Tag(TagNode),
    BlockId(BlockIdNode),
    Paragraph(Vec<Node>, Span),
    Text(String, Span),
}

/// Represents a parsed Markdown document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    source: String,
    lines: Vec<Line>,
    nodes: Vec<Node>,
}

impl Document {
    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn lines(&self) -> &[Line] {
        &self.lines
    }

    pub fn reconstruct_source(&self) -> String {
        let mut out = String::with_capacity(self.source.len());
        for line in &self.lines {
            out.push_str(line.content());
            out.push_str(line.ending().as_str());
        }
        out
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn frontmatter(&self) -> Option<&FrontmatterNode> {
        self.nodes.iter().find_map(|node| match node {
            Node::Frontmatter(fm) => Some(fm),
            _ => None,
        })
    }

    pub fn wiki_links(&self) -> Vec<&WikiLinkNode> {
        let mut result = Vec::new();
        self.collect_wiki_links(&self.nodes, &mut result);
        result
    }

    fn collect_wiki_links<'a>(&self, nodes: &'a [Node], out: &mut Vec<&'a WikiLinkNode>) {
        for node in nodes {
            match node {
                Node::WikiLink(wl) => out.push(wl),
                Node::Callout(c) => self.collect_wiki_links(&c.content, out),
                Node::Paragraph(children, _) => self.collect_wiki_links(children, out),
                _ => {}
            }
        }
    }

    pub fn callouts(&self) -> Vec<&CalloutNode> {
        let mut result = Vec::new();
        self.collect_callouts(&self.nodes, &mut result);
        result
    }

    fn collect_callouts<'a>(&self, nodes: &'a [Node], out: &mut Vec<&'a CalloutNode>) {
        for node in nodes {
            match node {
                Node::Callout(c) => {
                    out.push(c);
                    self.collect_callouts(&c.content, out);
                }
                Node::Paragraph(children, _) => self.collect_callouts(children, out),
                _ => {}
            }
        }
    }

    pub fn block_ids(&self) -> Vec<&BlockIdNode> {
        let mut result = Vec::new();
        self.collect_block_ids(&self.nodes, &mut result);
        result
    }

    fn collect_block_ids<'a>(&self, nodes: &'a [Node], out: &mut Vec<&'a BlockIdNode>) {
        for node in nodes {
            match node {
                Node::BlockId(b) => out.push(b),
                Node::Callout(c) => self.collect_block_ids(&c.content, out),
                Node::Paragraph(children, _) => self.collect_block_ids(children, out),
                _ => {}
            }
        }
    }

    pub fn inline_math(&self) -> Vec<&InlineMathNode> {
        let mut result = Vec::new();
        self.collect_inline_math(&self.nodes, &mut result);
        result
    }

    fn collect_inline_math<'a>(&self, nodes: &'a [Node], out: &mut Vec<&'a InlineMathNode>) {
        for node in nodes {
            match node {
                Node::InlineMath(im) => out.push(im),
                Node::Callout(c) => self.collect_inline_math(&c.content, out),
                Node::Paragraph(children, _) => self.collect_inline_math(children, out),
                _ => {}
            }
        }
    }

    pub fn display_math(&self) -> Vec<&DisplayMathNode> {
        let mut result = Vec::new();
        self.collect_display_math(&self.nodes, &mut result);
        result
    }

    fn collect_display_math<'a>(&self, nodes: &'a [Node], out: &mut Vec<&'a DisplayMathNode>) {
        for node in nodes {
            match node {
                Node::DisplayMath(dm) => out.push(dm),
                Node::Callout(c) => self.collect_display_math(&c.content, out),
                Node::Paragraph(children, _) => self.collect_display_math(children, out),
                _ => {}
            }
        }
    }

    pub fn tags(&self) -> Vec<&TagNode> {
        let mut result = Vec::new();
        self.collect_tags(&self.nodes, &mut result);
        result
    }

    fn collect_tags<'a>(&self, nodes: &'a [Node], out: &mut Vec<&'a TagNode>) {
        for node in nodes {
            match node {
                Node::Tag(t) => out.push(t),
                Node::Callout(c) => self.collect_tags(&c.content, out),
                Node::Paragraph(children, _) => self.collect_tags(children, out),
                _ => {}
            }
        }
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
    }

    let nodes = parse_ast_nodes(&lines);

    Document {
        source: source.to_string(),
        lines,
        nodes,
    }
}

fn parse_ast_nodes(lines: &[Line]) -> Vec<Node> {
    let mut nodes = Vec::new();
    if lines.is_empty() {
        return nodes;
    }

    // Check for YAML frontmatter at start of file
    let mut idx = 0;
    if lines[0].content().trim_end() == "---" {
        // Find closing ---
        let mut closing_idx = None;
        for (c_idx, line) in lines.iter().enumerate().skip(1) {
            if line.content().trim_end() == "---" {
                closing_idx = Some(c_idx);
                break;
            }
        }

        if let Some(c_idx) = closing_idx {
            let mut raw_content = String::new();
            for item in 1..c_idx {
                if item > 1 {
                    raw_content.push_str(lines[item - 1].ending().as_str());
                }
                raw_content.push_str(lines[item].content());
            }

            let start_byte = lines[0].byte_offset();
            let end_byte = lines[c_idx].byte_offset() + lines[c_idx].byte_len();

            let span = Span::new(
                start_byte,
                end_byte,
                lines[0].line_number(),
                1,
                lines[c_idx].line_number(),
                lines[c_idx].content().len() + 1 + lines[c_idx].ending().len(),
            );

            nodes.push(Node::Frontmatter(FrontmatterNode { raw_content, span }));
            idx = c_idx + 1;
        }
    }

    while idx < lines.len() {
        let line = &lines[idx];
        let trimmed_start = line.content().trim_start();

        // Check for Display Math `$$`
        if let Some(rest) = trimmed_start.strip_prefix("$$") {
            let start_idx = idx;
            let mut end_idx = idx;
            let mut math_content = String::new();

            if rest.contains("$$") {
                // Single-line display math `$$...$$`
                let end_pos = rest.find("$$").unwrap();
                math_content = rest[..end_pos].to_string();
            } else {
                // Multiline display math
                for m_idx in idx + 1..lines.len() {
                    let mline_content = lines[m_idx].content().trim_start();
                    if mline_content.starts_with("$$") || mline_content.ends_with("$$") {
                        end_idx = m_idx;
                        break;
                    } else {
                        if !math_content.is_empty() {
                            math_content.push_str(lines[m_idx - 1].ending().as_str());
                        }
                        math_content.push_str(lines[m_idx].content());
                    }
                }
            }

            let span = Span::new(
                lines[start_idx].byte_offset(),
                lines[end_idx].byte_offset() + lines[end_idx].byte_len(),
                lines[start_idx].line_number(),
                1,
                lines[end_idx].line_number(),
                lines[end_idx].content().len() + 1 + lines[end_idx].ending().len(),
            );

            nodes.push(Node::DisplayMath(DisplayMathNode {
                math: math_content,
                span,
            }));

            idx = end_idx + 1;
            continue;
        }

        if trimmed_start.starts_with("> [!") {
            if let Some(close_bracket) = trimmed_start.find(']') {
                let kind = trimmed_start[4..close_bracket].to_string();
                let rest = trimmed_start[close_bracket + 1..].trim();
                let title = if rest.is_empty() {
                    None
                } else {
                    Some(rest.to_string())
                };

                let start_idx = idx;
                let mut end_idx = idx;

                while end_idx + 1 < lines.len()
                    && lines[end_idx + 1].content().trim_start().starts_with('>')
                {
                    end_idx += 1;
                }

                let mut inner_nodes = Vec::new();
                for iline in &lines[start_idx..=end_idx] {
                    let icontent = iline.content().trim_start();
                    let stripped = if let Some(s) = icontent.strip_prefix("> ") {
                        s
                    } else if let Some(s) = icontent.strip_prefix('>') {
                        s
                    } else {
                        icontent
                    };

                    let inner_line = Line::new(
                        iline.line_number(),
                        iline.byte_offset() + (iline.content().len() - stripped.len()),
                        stripped.to_string(),
                        iline.ending(),
                    );
                    parse_line(&inner_line, &mut inner_nodes);
                }

                let span = Span::new(
                    lines[start_idx].byte_offset(),
                    lines[end_idx].byte_offset() + lines[end_idx].byte_len(),
                    lines[start_idx].line_number(),
                    1,
                    lines[end_idx].line_number(),
                    lines[end_idx].content().len() + 1 + lines[end_idx].ending().len(),
                );

                nodes.push(Node::Callout(CalloutNode {
                    kind,
                    title,
                    content: inner_nodes,
                    span,
                }));

                idx = end_idx + 1;
                continue;
            }
        }

        parse_line(line, &mut nodes);
        idx += 1;
    }

    nodes
}

fn parse_line(line: &Line, nodes: &mut Vec<Node>) {
    let content = line.content();

    // Check for WikiLinks `[[...]]`
    let mut cursor = 0;
    while let Some(start) = content[cursor..].find("[[") {
        let actual_start = cursor + start;
        if let Some(end) = content[actual_start + 2..].find("]]") {
            let actual_end = actual_start + 2 + end;
            let inner = &content[actual_start + 2..actual_end];

            let (target, alias) = if let Some(pipe_idx) = inner.find('|') {
                (
                    inner[..pipe_idx].to_string(),
                    Some(inner[pipe_idx + 1..].to_string()),
                )
            } else {
                (inner.to_string(), None)
            };

            let start_byte = line.byte_offset() + actual_start;
            let end_byte = line.byte_offset() + actual_end + 2;

            let span = Span::new(
                start_byte,
                end_byte,
                line.line_number(),
                actual_start + 1,
                line.line_number(),
                actual_end + 3,
            );

            nodes.push(Node::WikiLink(WikiLinkNode {
                target,
                alias,
                span,
            }));

            cursor = actual_end + 2;
        } else {
            break;
        }
    }

    // Check for Inline Math `$math$` (ignoring `$$`)
    cursor = 0;
    while let Some(start) = content[cursor..].find('$') {
        let actual_start = cursor + start;
        if actual_start + 1 < content.len() && content.as_bytes()[actual_start + 1] == b'$' {
            cursor = actual_start + 2;
            continue;
        }

        if let Some(end) = content[actual_start + 1..].find('$') {
            let actual_end = actual_start + 1 + end;
            let math = content[actual_start + 1..actual_end].to_string();

            let start_byte = line.byte_offset() + actual_start;
            let end_byte = line.byte_offset() + actual_end + 1;

            let span = Span::new(
                start_byte,
                end_byte,
                line.line_number(),
                actual_start + 1,
                line.line_number(),
                actual_end + 2,
            );

            nodes.push(Node::InlineMath(InlineMathNode { math, span }));
            cursor = actual_end + 1;
        } else {
            break;
        }
    }

    // Check for Block Identifiers `^block-id` (preceded by whitespace or line start)
    cursor = 0;
    while let Some(start) = content[cursor..].find('^') {
        let actual_start = cursor + start;
        let is_valid_start = actual_start == 0
            || content.as_bytes()[actual_start - 1] == b' '
            || content.as_bytes()[actual_start - 1] == b'\t';

        if is_valid_start {
            let rest = &content[actual_start + 1..];
            let id_len = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
                .map(|c| c.len_utf8())
                .sum::<usize>();

            if id_len > 0 {
                let id = rest[..id_len].to_string();
                let start_byte = line.byte_offset() + actual_start;
                let end_byte = start_byte + 1 + id_len;

                let span = Span::new(
                    start_byte,
                    end_byte,
                    line.line_number(),
                    actual_start + 1,
                    line.line_number(),
                    actual_start + 1 + 1 + id_len,
                );

                nodes.push(Node::BlockId(BlockIdNode { id, span }));
                cursor = actual_start + 1 + id_len;
                continue;
            }
        }
        cursor = actual_start + 1;
    }

    // Check for Tags `#tag/subtag` (ignoring ATX headings)
    let is_atx_heading = {
        let trimmed = content.trim_start();
        if trimmed.starts_with('#') {
            let hash_count = trimmed.chars().take_while(|c| *c == '#').count();
            (1..=6).contains(&hash_count)
                && (trimmed[hash_count..].starts_with(' ') || trimmed[hash_count..].is_empty())
        } else {
            false
        }
    };

    if is_atx_heading {
        let trimmed = content.trim_start();
        let hash_count = trimmed.chars().take_while(|c| *c == '#').count();
        let title = trimmed[hash_count..].trim().to_string();
        let span = Span::new(
            line.byte_offset(),
            line.byte_offset() + line.byte_len(),
            line.line_number(),
            1,
            line.line_number(),
            line.content().len() + 1 + line.ending().len(),
        );
        nodes.push(Node::Heading(HeadingNode {
            level: hash_count as u8,
            title,
            span,
        }));
    } else {
        cursor = 0;
        while let Some(start) = content[cursor..].find('#') {
            let actual_start = cursor + start;
            let rest = &content[actual_start + 1..];
            let tag_len = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '/' || *c == '-' || *c == '_')
                .map(|c| c.len_utf8())
                .sum::<usize>();

            if tag_len > 0 {
                let tag = rest[..tag_len].to_string();
                let start_byte = line.byte_offset() + actual_start;
                let end_byte = start_byte + 1 + tag_len;

                let span = Span::new(
                    start_byte,
                    end_byte,
                    line.line_number(),
                    actual_start + 1,
                    line.line_number(),
                    actual_start + 1 + 1 + tag_len,
                );

                nodes.push(Node::Tag(TagNode { tag, span }));
                cursor = actual_start + 1 + tag_len;
            } else {
                cursor = actual_start + 1;
            }
        }
    }
}
