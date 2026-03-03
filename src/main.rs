use tree_sitter::{Language, Parser, Point, Query, QueryCursor, StreamingIterator};
use std::{fs::File, io::Read, path::Path};

enum Lang {
    Rust,
    Python,
    Elixir,
    Unknown,
}

impl Lang {
    fn grammar(&self) -> Option<Language> {
        match self {
            Lang::Rust => Some(tree_sitter_rust::LANGUAGE.into()),
            Lang::Python => Some(tree_sitter_python::LANGUAGE.into()),
            Lang::Elixir => Some(tree_sitter_elixir::LANGUAGE.into()),
            _ => None,
        }
    }

    fn comment_query(&self) -> Option<&str> {
        match self {
            Lang::Rust => Some("[(line_comment) (block_comment)] @comment"),
            Lang::Python => Some("(comment) @comment"),
            Lang::Elixir => Some("(comment) @comment"),
            _ => None
        }
    }

    fn of(source_path: &str) -> Option<Lang> {
        let path = Path::new(source_path);
        println!("path: {}", path.to_str().expect("error"));
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "rs" => Some(Lang::Rust),
            "py" | "pyw" => Some(Lang::Python),
            "ex" | "exs" => Some(Lang::Elixir),
            _ => None
        }
    }
}

struct Comment {
    pub text: String,
    pub start: Point,
    pub end: Point
}

struct CommentExtractor {
    parser: Parser,
    cursor: QueryCursor
}

impl CommentExtractor {
    pub fn new() -> Self {
        Self { 
            parser: Parser::new(), 
            cursor: QueryCursor::new() 
        }
    }

    pub fn extract(&mut self, lang: &Lang, source: &str) -> Vec<Comment> {
        let Some(grammar) = lang.grammar() else { return vec![]; };

        self.parser.set_language(&grammar).ok();
        let tree = self.parser.parse(source, None).unwrap();
        println!("tree: {:?}", tree);

        // TODO: rc, arc
        // TODO: lambdába kiszervezni
        // Vagy traiteken keresztüli kezelés
        /*
            core: cargo static lib -> ide regisztrálnak a nyelv cratek
            lang: core-ra hivatkozik, egyelőre libek, aztán majd refactor max
         */
        let query = Query::new(&grammar, lang.comment_query().unwrap()).expect("invalid query");
        let mut matches = self.cursor.matches(&query, tree.root_node(), source.as_bytes());
        let mut comments = Vec::new();

        while let Some(m) = matches.next() {
            println!("match: {:?}", m);
            for capture in m.captures {
                let node = capture.node;
                let comment = Comment {
                    text: source[node.start_byte()..node.end_byte()].to_string(),
                    start: node.start_position(),
                    end: node.end_position()
                };
                comments.push(comment);
            }
        }

        comments
    }
}

fn main() {
    let elixir_source_path = "input/in.ex";
    let mut elixir_source_file = File::open(elixir_source_path).expect("file not found");

    let mut elixir_source = String::new();
    let _result = elixir_source_file.read_to_string(&mut elixir_source).expect("file read error");
    println!("Source:\n{}", elixir_source);

    let elixir_lang = Lang::of(elixir_source_path);
    let mut elixir_extractor = CommentExtractor::new();
    let elixir_comments = elixir_extractor.extract(&elixir_lang.unwrap(), elixir_source.as_str());

    println!("Comments:");
    for c in elixir_comments {
        println!("{}-{}: {}", c.start, c.end, c.text);
    }

    let rust_source_path = "input/in.rs";
    let mut rust_source_file = File::open(rust_source_path).expect("file not found");

    let mut rust_source = String::new();
    let _result = rust_source_file.read_to_string(&mut rust_source).expect("file read error");
    println!("Source:\n{}", rust_source);

    let rust_lang = Lang::of(rust_source_path);
    let mut rust_extractor = CommentExtractor::new();
    let rust_comments = rust_extractor.extract(&rust_lang.unwrap(), rust_source.as_str());

    println!("Comments:");
    for c in rust_comments {
        println!("{}-{}: {}", c.start, c.end, c.text);
    }

    let python_source_path = "input/in.py";
    let mut python_source_file = File::open(python_source_path).expect("file not found");

    let mut python_source = String::new();
    let _result = python_source_file.read_to_string(&mut python_source).expect("file read error");
    println!("Source:\n{}", python_source);

    let python_lang = Lang::of(python_source_path);
    let mut python_extractor = CommentExtractor::new();
    let python_comments = python_extractor.extract(&python_lang.unwrap(), python_source.as_str());

    println!("Comments:");
    for c in python_comments {
        println!("{}-{}: {}", c.start, c.end, c.text);
    }

    /*
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("Error loading Rust grammar");

    let rust_source_code = "// this is a line comment\nfn test() {} // this is a line comment after test fn\n/* this\nis\na\nblock\ncomment.\n*/\n/// this is a doc comment.\n//! this is also a doc comment.";

    let tree = parser.parse(rust_source_code, None).unwrap();

    let query_str = "
        [
            (line_comment)
            (block_comment)
        ] @comment
    ";
    let query =
        Query::new(&tree_sitter_rust::LANGUAGE.into(), query_str).expect("Error compiling query.");
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), rust_source_code.as_bytes());

    while let Some(m) = matches.next() {
        println!("{:?}", m);

        for capture in m.captures {
            println!("{:?}", capture);
            let start = capture.node.start_byte();
            let end = capture.node.end_byte();
            let text = &rust_source_code[start..end];
            println!("{}", text);
        }

        println!();
    }
    */
}
