use anyhow::Result;
use encoding_rs::WINDOWS_1251;
use rand::{Rng, rng};

const BIBLE_DATA: &[u8] = include_bytes!("../assets/Bible_txt.txt");

enum LineType {
    Book(String),
    Chapter(String),
    Verse(u32, String),
    Unknown,
}

impl<S: AsRef<str>> From<S> for LineType {
    fn from(value: S) -> Self {
        let s = value.as_ref();
        if s.starts_with("===") && s.ends_with("===") {
            return LineType::Chapter(s[4..s.len() - 4].to_string());
        }

        if s.starts_with("==") && s.ends_with("==") {
            return LineType::Book(s[3..s.len() - 3].to_string());
        }

        if let Some((num, v)) = s.split_once(' ') {
            if let Ok(num) = num.parse() {
                return LineType::Verse(num, v.to_string());
            }
        }

        LineType::Unknown
    }
}

#[derive(Default)]
struct Bible {
    books: Vec<Book>,
}

#[derive(Default)]
struct Book {
    name: String,
    chapters: Vec<Chapter>,
}

#[derive(Default)]
struct Chapter {
    number: String,
    verses: Vec<Verse>,
}

#[derive(Default)]
struct Verse {
    number: u32,
    text: String,
}

#[derive(Default)]
struct App {
    bible: Bible,
}

impl App {
    fn load_holy_bible() -> Result<Self> {
        let (res, _, _) = WINDOWS_1251.decode(BIBLE_DATA);

        let content = res.into_owned();

        let mut bible = Bible::default();
        let mut cur_book: Option<Book> = None;
        let mut cur_chapter: Option<Chapter> = None;

        for line in content.lines().filter(|l| !l.is_empty()) {
            let line_type = LineType::from(line.trim());
            match line_type {
                LineType::Book(name) => {
                    if let Some(chapter) = cur_chapter.take() {
                        if let Some(book) = &mut cur_book {
                            book.chapters.push(chapter);
                        }
                    }
                    if let Some(book) = cur_book.take() {
                        bible.books.push(book);
                    }
                    cur_book = Some(Book {
                        name,
                        ..Default::default()
                    });
                }
                LineType::Chapter(num) => {
                    if let Some(chapter) = cur_chapter.take() {
                        if let Some(book) = &mut cur_book {
                            book.chapters.push(chapter);
                        }
                    }
                    cur_chapter = Some(Chapter {
                        number: num,
                        ..Default::default()
                    });
                }
                LineType::Verse(num, text) => {
                    if let Some(chapter) = &mut cur_chapter {
                        chapter.verses.push(Verse { number: num, text });
                    }
                }
                LineType::Unknown => {}
            }
        }
        if let Some(chapter) = cur_chapter.take() {
            if let Some(book) = &mut cur_book {
                book.chapters.push(chapter);
            }
        }
        if let Some(book) = cur_book.take() {
            bible.books.push(book);
        }
        Ok(App { bible })
    }
}

fn main() -> Result<()> {
    let app = App::load_holy_bible()?;
    let book = app
        .bible
        .books
        .get(rng().random_range(0..app.bible.books.len()))
        .unwrap();
    let chapter = book
        .chapters
        .get(rng().random_range(0..book.chapters.len()))
        .unwrap();
    let verse = chapter
        .verses
        .get(rng().random_range(0..chapter.verses.len()))
        .unwrap();

    println!(
        "{}. {} [{}:{}]",
        verse.number, verse.text, book.name, chapter.number
    );
    Ok(())
}
